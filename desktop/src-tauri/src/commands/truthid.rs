use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// Mesma faixa de portas que o TruthID Desktop tenta em
/// `desktop/src-tauri/src/local_signer_server.rs` (bloco próprio, longe do
/// LAN da Fase 13.9 e do Vite dev server) — precisa ser espelhada manualmente
/// aqui, é a única forma de descoberta (localhost, sem mDNS/broadcast).
const CANDIDATE_PORTS: [u16; 5] = [47950, 47951, 47952, 47953, 47954];

const APP_NAME: &str = "Practice Valuation";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Endereço de burn — dest do sign-request de teste. value="0" e callData="0x"
/// (transferência de valor puro, sem chamar nenhuma função) fazem desta prova
/// de conceito uma UserOperation real (assinada, enviada ao bundler, com
/// userOpHash/transactionHash de verdade) sem nenhum efeito econômico.
const TEST_DEST_ADDRESS: &str = "0x000000000000000000000000000000000000dEaD";

/// Como callData é vazio, o seletor calculado a partir desta assinatura nunca
/// vai bater com os 4 primeiros bytes do callData — a tela de aprovação do
/// TruthID mostra "não verificado" + bytes crus, o comportamento correto pra
/// uma transferência sem chamada de função (não é bug desta fatia).
const TEST_FUNCTION_SIGNATURE: &str = "practiceValuationTestPing()";

#[derive(Deserialize)]
struct PingResponse {
    version: String,
}

#[derive(Deserialize)]
struct HandshakeResponse {
    accepted: bool,
    error: Option<String>,
}

/// Tenta cada porta candidata em ordem até achar um TruthID Desktop
/// respondendo — não há descoberta de rede real, é tudo loopback.
async fn discover() -> Result<(u16, String), AppError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(500))
        .build()?;

    for port in CANDIDATE_PORTS {
        let url = format!("http://127.0.0.1:{port}/truthid/v1/ping");
        if let Ok(resp) = client.get(&url).send().await {
            if let Ok(body) = resp.json::<PingResponse>().await {
                return Ok((port, body.version));
            }
        }
    }

    Err(AppError::TruthIdNotFound)
}

#[derive(Serialize)]
pub struct TruthIdHandshakeResult {
    port: u16,
    desktop_version: String,
    accepted: bool,
}

#[tauri::command]
pub async fn test_truthid_connection() -> Result<TruthIdHandshakeResult, AppError> {
    let (port, desktop_version) = discover().await?;

    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{port}/truthid/v1/handshake");
    let body: HandshakeResponse = client
        .post(&url)
        .json(&serde_json::json!({ "appName": APP_NAME, "appVersion": APP_VERSION }))
        .send()
        .await?
        .json()
        .await?;

    if !body.accepted {
        return Err(AppError::TruthId(
            body.error.unwrap_or_else(|| "handshake rejected".to_string()),
        ));
    }

    Ok(TruthIdHandshakeResult { port, desktop_version, accepted: true })
}

#[derive(Serialize, Deserialize)]
pub struct TruthIdSignResult {
    status: String,
    user_op_hash: Option<String>,
    transaction_hash: Option<String>,
    error: Option<String>,
}

#[tauri::command]
pub async fn send_test_sign_request() -> Result<TruthIdSignResult, AppError> {
    let (port, _) = discover().await?;

    // Margem sobre o timeout de 5min que o próprio TruthID já aplica no
    // handler HTTP (ver sign_request.rs, SIGN_REQUEST_TIMEOUT) — este client
    // só não pode expirar antes dele.
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(310))
        .build()?;
    let url = format!("http://127.0.0.1:{port}/truthid/v1/sign-request");
    let result: TruthIdSignResult = client
        .post(&url)
        .json(&serde_json::json!({
            "appName": APP_NAME,
            "dest": TEST_DEST_ADDRESS,
            "value": "0",
            "callData": "0x",
            "functionSignature": TEST_FUNCTION_SIGNATURE,
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(result)
}
