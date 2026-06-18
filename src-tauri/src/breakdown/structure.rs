use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StructureOutput {
    pub hook: String,
    pub beats: Vec<String>,
    pub cta: String,
    pub source: String,
}

pub async fn analyze_structure(
    _app: &tauri::AppHandle,
    transcript: &str,
) -> StructureOutput {
    // Optional LLM step is abstracted here. Without an API key we derive a
    // readable structure from the transcript itself.
    let sentences: Vec<&str> = transcript
        .split(|c: char| ['.', '!', '?'].contains(&c))
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let hook = sentences.first().copied().unwrap_or("").to_string();
    let cta = sentences.last().copied().unwrap_or("").to_string();
    let beats = if sentences.len() > 2 {
        sentences[1..sentences.len() - 1]
            .iter()
            .take(6)
            .map(|s| (*s).to_string())
            .collect()
    } else {
        Vec::new()
    };

    StructureOutput {
        hook,
        beats,
        cta,
        source: "heuristic".into(),
    }
}
