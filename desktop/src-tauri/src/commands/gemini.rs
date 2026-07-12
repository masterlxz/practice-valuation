use serde::{Deserialize, Serialize};

use crate::commands::api_key::KEYRING_SERVICE;
use crate::error::AppError;

const GEMINI_PROVIDER: &str = "gemini";
const GEMINI_MODEL: &str = "gemini-3.1-flash-lite";

// Mirrors the Gemini REST API shape 1:1 (contents/parts/role) so the same
// struct can grow into full chat history in Fase 7.4 without changing shape.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiContent {
    pub role: String,
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Serialize)]
struct GeminiRequestBody<'a> {
    contents: &'a [GeminiContent],
}

#[derive(Deserialize)]
struct GeminiResponseBody {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

fn read_gemini_api_key() -> Result<String, AppError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, GEMINI_PROVIDER)?;
    match entry.get_password() {
        Ok(key) => Ok(key),
        Err(keyring::Error::NoEntry) => Err(AppError::MissingApiKey(GEMINI_PROVIDER.to_string())),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
pub async fn ask_gemini(history: Vec<GeminiContent>) -> Result<String, AppError> {
    let api_key = read_gemini_api_key()?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{GEMINI_MODEL}:generateContent?key={api_key}"
    );
    let body = GeminiRequestBody { contents: &history };

    let client = reqwest::Client::new();
    let response = client.post(&url).json(&body).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return Err(AppError::GeminiApi(format!("{status}: {error_body}")));
    }

    let parsed: GeminiResponseBody = response.json().await?;
    parsed
        .candidates
        .into_iter()
        .next()
        .and_then(|candidate| candidate.content.parts.into_iter().next())
        .map(|part| part.text)
        .ok_or_else(|| AppError::GeminiApi("empty response from Gemini".to_string()))
}
