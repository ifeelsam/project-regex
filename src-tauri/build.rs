use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn main() {
    ensure_sidecar_stubs();
    tauri_build::build();
}

fn ensure_sidecar_stubs() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let binaries_dir = manifest_dir.join("binaries");
    let _ = fs::create_dir_all(&binaries_dir);

    let triple = env::var("TAURI_ENV_TARGET_TRIPLE")
        .or_else(|_| env::var("TARGET"))
        .unwrap_or_else(|_| "aarch64-apple-darwin".into());

    let stub_body = "#!/bin/sh\necho \"Regex: install the real binary — see docs/media-tools.md\" >&2\nexit 1\n";

    for tool in ["yt-dlp", "ffmpeg", "whisper"] {
        let path = binaries_dir.join(format!("{tool}-{triple}"));
        if path.exists() {
            continue;
        }
        fs::write(&path, stub_body).expect("write sidecar stub");
        let mut perms = fs::metadata(&path).expect("metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).expect("chmod stub");
        println!("cargo:warning=created sidecar stub {}", path.display());
    }
}
