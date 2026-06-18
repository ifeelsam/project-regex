use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;
use tokio::fs;

use crate::db;
use crate::db::assets;
use crate::db::breakdowns::{AssetType, BreakdownStatus};

use super::structure::{analyze_structure, StructureOutput};

#[derive(Clone, Serialize)]
pub struct BreakdownProgress {
    pub breakdown_id: String,
    pub project_id: Option<String>,
    pub item_id: String,
    pub step: String,
    pub progress: f32,
    pub message: String,
}

pub type CancelMap = Arc<Mutex<HashMap<String, Arc<AtomicBool>>>>;

pub fn register_cancel(cancels: &CancelMap, breakdown_id: &str) -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    cancels
        .lock()
        .expect("cancel map lock")
        .insert(breakdown_id.to_string(), flag.clone());
    flag
}

pub fn cancel_job(cancels: &CancelMap, breakdown_id: &str) -> bool {
    if let Some(flag) = cancels.lock().expect("cancel map lock").get(breakdown_id) {
        flag.store(true, Ordering::SeqCst);
        return true;
    }
    false
}

pub fn clear_cancel(cancels: &CancelMap, breakdown_id: &str) {
    cancels
        .lock()
        .expect("cancel map lock")
        .remove(breakdown_id);
}

fn check_cancelled(flag: &AtomicBool) -> Result<(), String> {
    if flag.load(Ordering::SeqCst) {
        Err("Breakdown cancelled.".into())
    } else {
        Ok(())
    }
}

fn emit_progress(app: &AppHandle, event: BreakdownProgress) {
    let _ = app.emit("breakdown-progress", event);
}

async fn run_sidecar(
    app: &AppHandle,
    name: &str,
    args: &[String],
    flag: &AtomicBool,
) -> Result<String, String> {
    check_cancelled(flag)?;

    let sidecar = app
        .shell()
        .sidecar(name)
        .map_err(|_| format!("{name} is not bundled — see docs/media-tools.md"))?;

    let (mut rx, _child) = sidecar
        .args(args)
        .spawn()
        .map_err(|e| format!("failed to start {name}: {e}"))?;

    let mut stdout = String::new();
    let mut stderr = String::new();

    while let Some(event) = rx.recv().await {
        check_cancelled(flag)?;
        match event {
            CommandEvent::Stdout(line) => stdout.push_str(&String::from_utf8_lossy(&line)),
            CommandEvent::Stderr(line) => stderr.push_str(&String::from_utf8_lossy(&line)),
            CommandEvent::Error(err) => return Err(format!("{name} error: {err}")),
            CommandEvent::Terminated(payload) => {
                if payload.code == Some(0) {
                    return Ok(stdout);
                }
                let detail = if stderr.is_empty() { stdout } else { stderr };
                return Err(format!("{name} failed: {detail}"));
            }
            _ => {}
        }
    }

    Err(format!("{name} exited unexpectedly"))
}

async fn work_dir(app: &AppHandle, breakdown_id: &str) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("breakdowns")
        .join(breakdown_id);
    fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("could not create work directory: {e}"))?;
    Ok(dir)
}

async fn find_source_file(dir: &Path) -> Option<PathBuf> {
    let mut entries = fs::read_dir(dir).await.ok()?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.file_name().and_then(|n| n.to_str())?.starts_with("source.") {
            return Some(path);
        }
    }
    None
}

pub async fn run(
    app: AppHandle,
    pool: sqlx::SqlitePool,
    cancels: CancelMap,
    breakdown_id: String,
    item_id: String,
    project_id: Option<String>,
    personal_use_confirmed: bool,
) {
    let flag = register_cancel(&cancels, &breakdown_id);

    let result = run_inner(
        &app,
        &pool,
        &breakdown_id,
        &item_id,
        project_id.as_deref(),
        personal_use_confirmed,
        &flag,
    )
    .await;

    clear_cancel(&cancels, &breakdown_id);

    match result {
        Ok(()) => {
            let _ = db::breakdowns::set_status(&pool, &breakdown_id, BreakdownStatus::Done, None).await;
            emit_progress(
                &app,
                BreakdownProgress {
                    breakdown_id: breakdown_id.clone(),
                    project_id: project_id.clone(),
                    item_id: item_id.clone(),
                    step: "done".into(),
                    progress: 1.0,
                    message: "Breakdown complete.".into(),
                },
            );
        }
        Err(message) => {
            let _ = db::breakdowns::set_status(
                &pool,
                &breakdown_id,
                BreakdownStatus::Failed,
                Some(&message),
            )
            .await;
            emit_progress(
                &app,
                BreakdownProgress {
                    breakdown_id,
                    project_id,
                    item_id,
                    step: "failed".into(),
                    progress: 0.0,
                    message,
                },
            );
        }
    }
}

async fn run_inner(
    app: &AppHandle,
    pool: &sqlx::SqlitePool,
    breakdown_id: &str,
    item_id: &str,
    project_id: Option<&str>,
    personal_use_confirmed: bool,
    flag: &AtomicBool,
) -> Result<(), String> {
    if !personal_use_confirmed {
        return Err(
            "Confirm personal-use before processing media you have the right to reference.".into(),
        );
    }

    let _ = db::breakdowns::set_status(pool, breakdown_id, BreakdownStatus::Running, None)
        .await
        .map_err(|e| e.to_string())?;

    let item = db::items::get(pool, item_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Item not found.".to_string())?;

    let dir = work_dir(app, breakdown_id).await?;

    emit_progress(
        app,
        BreakdownProgress {
            breakdown_id: breakdown_id.to_string(),
            project_id: project_id.map(str::to_string),
            item_id: item_id.to_string(),
            step: "prepare".into(),
            progress: 0.05,
            message: "Preparing breakdown…".into(),
        },
    );

    let mut source_path: Option<PathBuf> = None;
    let mut transcript_text = String::new();

    if let Some(url) = item.url.as_deref().filter(|u| !u.is_empty()) {
        emit_progress(
            app,
            BreakdownProgress {
                breakdown_id: breakdown_id.to_string(),
                project_id: project_id.map(str::to_string),
                item_id: item_id.to_string(),
                step: "download".into(),
                progress: 0.15,
                message: "Fetching permitted source…".into(),
            },
        );

        let output_template = dir.join("source.%(ext)s");
        let output = output_template.to_string_lossy().to_string();
        run_sidecar(
            app,
            "yt-dlp",
            &[
                "-o".into(),
                output,
                "--no-playlist".into(),
                "--max-filesize".into(),
                "500M".into(),
                url.to_string(),
            ],
            flag,
        )
        .await?;

        source_path = find_source_file(&dir).await;
        if source_path.is_none() {
            return Err(
                "Could not download this source. It may be protected or unavailable for personal use."
                    .into(),
            );
        }
    }

    if let Some(ref source) = source_path {
        let frames_dir = dir.join("frames");
        fs::create_dir_all(&frames_dir)
            .await
            .map_err(|e| e.to_string())?;
        let pattern = frames_dir.join("frame_%04d.jpg");

        emit_progress(
            app,
            BreakdownProgress {
                breakdown_id: breakdown_id.to_string(),
                project_id: project_id.map(str::to_string),
                item_id: item_id.to_string(),
                step: "keyframes".into(),
                progress: 0.35,
                message: "Extracting keyframes…".into(),
            },
        );

        run_sidecar(
            app,
            "ffmpeg",
            &[
                "-y".into(),
                "-i".into(),
                source.display().to_string(),
                "-vf".into(),
                "fps=1/8".into(),
                "-frames:v".into(),
                "12".into(),
                pattern.display().to_string(),
            ],
            flag,
        )
        .await?;

        let mut frame_index = 0i64;
        let mut frame_entries = fs::read_dir(&frames_dir).await.map_err(|e| e.to_string())?;
        while let Ok(Some(entry)) = frame_entries.next_entry().await {
            check_cancelled(flag)?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("jpg") {
                continue;
            }
            frame_index += 1;
            let start_ms = (frame_index - 1) * 8000;
            assets::insert(
                pool,
                breakdown_id,
                project_id,
                Some(item_id),
                AssetType::Frame,
                &path.to_string_lossy(),
                Some(start_ms),
                Some(start_ms + 8000),
                &format!("Frame {frame_index}"),
                "{}",
            )
            .await
            .map_err(|e| e.to_string())?;
        }

        emit_progress(
            app,
            BreakdownProgress {
                breakdown_id: breakdown_id.to_string(),
                project_id: project_id.map(str::to_string),
                item_id: item_id.to_string(),
                step: "clips".into(),
                progress: 0.5,
                message: "Generating clip segments…".into(),
            },
        );

        let clips_dir = dir.join("clips");
        fs::create_dir_all(&clips_dir)
            .await
            .map_err(|e| e.to_string())?;

        for (idx, start) in [(0, 0i64), (1, 15_000), (2, 30_000)] {
            check_cancelled(flag)?;
            let clip_path = clips_dir.join(format!("clip_{idx:02}.mp4"));
            let _ = run_sidecar(
                app,
                "ffmpeg",
                &[
                    "-y".into(),
                    "-ss".into(),
                    format!("{:.3}", start as f64 / 1000.0),
                    "-i".into(),
                    source.display().to_string(),
                    "-t".into(),
                    "12".into(),
                    "-c".into(),
                    "copy".into(),
                    clip_path.display().to_string(),
                ],
                flag,
            )
            .await;

            if clip_path.exists() {
                assets::insert(
                    pool,
                    breakdown_id,
                    project_id,
                    Some(item_id),
                    AssetType::Clip,
                    &clip_path.to_string_lossy(),
                    Some(start),
                    Some(start + 12_000),
                    &format!("Clip {}", idx + 1),
                    "{}",
                )
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        emit_progress(
            app,
            BreakdownProgress {
                breakdown_id: breakdown_id.to_string(),
                project_id: project_id.map(str::to_string),
                item_id: item_id.to_string(),
                step: "audio".into(),
                progress: 0.62,
                message: "Extracting audio…".into(),
            },
        );

        let audio_path = dir.join("audio.wav");
        run_sidecar(
            app,
            "ffmpeg",
            &[
                "-y".into(),
                "-i".into(),
                source.display().to_string(),
                "-vn".into(),
                "-ar".into(),
                "16000".into(),
                "-ac".into(),
                "1".into(),
                "-c:a".into(),
                "pcm_s16le".into(),
                audio_path.display().to_string(),
            ],
            flag,
        )
        .await?;

        assets::insert(
            pool,
            breakdown_id,
            project_id,
            Some(item_id),
            AssetType::Audio,
            &audio_path.to_string_lossy(),
            None,
            None,
            "Extracted audio",
            "{}",
        )
        .await
        .map_err(|e| e.to_string())?;

        emit_progress(
            app,
            BreakdownProgress {
                breakdown_id: breakdown_id.to_string(),
                project_id: project_id.map(str::to_string),
                item_id: item_id.to_string(),
                step: "transcribe".into(),
                progress: 0.75,
                message: "Transcribing with whisper…".into(),
            },
        );

        let model_path = app
            .path()
            .app_data_dir()
            .map_err(|e| e.to_string())?
            .join("models")
            .join("ggml-base.en.bin");

        if model_path.exists() {
            let transcript_base = dir.join("transcript");
            if run_sidecar(
                app,
                "whisper",
                &[
                    "-m".into(),
                    model_path.display().to_string(),
                    "-f".into(),
                    audio_path.display().to_string(),
                    "-of".into(),
                    transcript_base.display().to_string(),
                    "-otxt".into(),
                ],
                flag,
            )
            .await
            .is_ok()
            {
                let txt_path = dir.join("transcript.txt");
                if let Ok(text) = fs::read_to_string(&txt_path).await {
                    transcript_text = text;
                    assets::insert(
                        pool,
                        breakdown_id,
                        project_id,
                        Some(item_id),
                        AssetType::Transcript,
                        &txt_path.to_string_lossy(),
                        None,
                        None,
                        "Whisper transcript",
                        "{}",
                    )
                    .await
                    .map_err(|e| e.to_string())?;

                    let _ = db::transcripts::insert_whisper(pool, item_id, &transcript_text).await;
                }
            }
        }
    }

    if transcript_text.is_empty() {
        if let Ok(Some(existing)) = db::transcripts::latest_for_item(pool, item_id).await {
            transcript_text = existing.text;
        } else {
            transcript_text = format!("{}\n\n{}", item.note, item.develop_note);
        }
    }

    emit_progress(
        app,
        BreakdownProgress {
            breakdown_id: breakdown_id.to_string(),
            project_id: project_id.map(str::to_string),
            item_id: item_id.to_string(),
            step: "structure".into(),
            progress: 0.9,
            message: "Analyzing structure…".into(),
        },
    );

    let structure: StructureOutput = analyze_structure(app, &transcript_text).await;
    let structure_path = dir.join("structure.json");
    let structure_json = serde_json::to_string_pretty(&structure).map_err(|e| e.to_string())?;
    fs::write(&structure_path, &structure_json)
        .await
        .map_err(|e| e.to_string())?;

    assets::insert(
        pool,
        breakdown_id,
        project_id,
        Some(item_id),
        AssetType::Structure,
        &structure_path.to_string_lossy(),
        None,
        None,
        "Hook / beats / CTA",
        &structure_json,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
