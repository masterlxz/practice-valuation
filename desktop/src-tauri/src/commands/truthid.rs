use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::ecies;
use crate::error::AppError;
use crate::lan_sweep;

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

// `rename_all = "camelCase"` é essencial, não cosmético: tanto o
// `SignRequestResponse` do TruthID Desktop (loopback,
// `desktop/src-tauri/src/sign_request.rs`) quanto o resultado que o Mobile
// entrega via LAN cross-device (`sign_request_approval_screen.dart::_deliver`)
// mandam `userOpHash`/`transactionHash` em camelCase — sem este atributo, os
// campos (sendo `Option<T>`) simplesmente nunca casam e ficam `None` em
// silêncio, mesmo quando a resposta real trazia um hash.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// Mesmo TTL que `qrPayload.ts::SESSION_TTL_MS` já usa pro pareamento do
/// Vault (extensão) — tempo suficiente pro usuário pegar o celular, escanear,
/// revisar e aprovar antes do QR expirar.
const CROSS_DEVICE_SESSION_TTL_MS: i64 = 3 * 60 * 1000;

/// Intervalo entre passadas de varredura LAN — o celular só começa a servir
/// depois que o usuário aprovar (e, no caso do sign-request, depois que a
/// UserOperation terminar de executar, até ~60s), então uma única passada
/// logo após mostrar o QR quase sempre vai vazia; o chamador repete até expirar.
const SWEEP_RETRY_INTERVAL: Duration = Duration::from_secs(2);

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_millis() as i64
}

fn random_session_id() -> String {
    let mut bytes = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Schema v1 do QR de `/sign-request` cross-device — precisa bater campo a
/// campo com `_validatePayload` em
/// `mobile/lib/screens/sign_request_approval_screen.dart` (TruthID).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SignRequestQrPayload {
    action: &'static str,
    v: u8,
    session_id: String,
    ephemeral_pub_key: String,
    expires_at: i64,
    app_name: &'static str,
    dest: &'static str,
    value: &'static str,
    call_data: &'static str,
    function_signature: &'static str,
}

#[derive(Serialize)]
pub struct CrossDeviceSession {
    session_id: String,
    ephemeral_priv_key_hex: String,
    expires_at_ms: i64,
    qr_payload_json: String,
}

/// Gera uma nova sessão cross-device de `/sign-request`: par efêmero (ECIES),
/// `sessionId` aleatório e o JSON do QR pra o frontend renderizar. Não fala
/// com a rede — só monta o convite. Quem varre a LAN esperando a resposta do
/// celular é `await_cross_device_sign_request_response`, chamado em seguida
/// pelo frontend assim que o QR aparece na tela. Reusa as mesmas constantes
/// `TEST_DEST_ADDRESS`/`TEST_FUNCTION_SIGNATURE` da PoC loopback
/// (`send_test_sign_request`) — mesma transferência de valor zero pro
/// endereço de burn, sem efeito econômico, mesma decisão da Sessão 103.
#[tauri::command]
pub fn create_cross_device_sign_request() -> Result<CrossDeviceSession, AppError> {
    let session_id = random_session_id();
    let (ephemeral_priv_key_hex, ephemeral_pub_key_hex) = ecies::generate_ephemeral_keypair();
    let expires_at_ms = now_ms() + CROSS_DEVICE_SESSION_TTL_MS;

    let payload = SignRequestQrPayload {
        action: "truthid-sign-request",
        v: 1,
        session_id: session_id.clone(),
        ephemeral_pub_key: ephemeral_pub_key_hex,
        expires_at: expires_at_ms,
        app_name: APP_NAME,
        dest: TEST_DEST_ADDRESS,
        value: "0",
        call_data: "0x",
        function_signature: TEST_FUNCTION_SIGNATURE,
    };
    let qr_payload_json =
        serde_json::to_string(&payload).map_err(|e| AppError::TruthId(e.to_string()))?;

    Ok(CrossDeviceSession {
        session_id,
        ephemeral_priv_key_hex,
        expires_at_ms,
        qr_payload_json,
    })
}

/// Varre a LAN repetidamente (portas `lan_sweep::CANDIDATE_PORTS`, mesmo
/// bloco que `RemoteSignerLanServer` do Mobile usa) até o celular responder
/// ou a sessão expirar. Decifra o blob ECIES com a chave efêmera privada
/// gerada por `create_cross_device_sign_request` e decodifica o mesmo
/// formato de `TruthIdSignResult` que o canal loopback já usa.
#[tauri::command]
pub async fn await_cross_device_sign_request_response(
    session_id: String,
    ephemeral_priv_key_hex: String,
    expires_at_ms: i64,
) -> Result<TruthIdSignResult, AppError> {
    let client = reqwest::Client::new();

    loop {
        if let Some(blob) = lan_sweep::sweep_once(&session_id, &client).await {
            let plaintext =
                ecies::decrypt(&blob, &ephemeral_priv_key_hex).map_err(AppError::TruthId)?;
            let result: TruthIdSignResult =
                serde_json::from_slice(&plaintext).map_err(|e| AppError::TruthId(e.to_string()))?;
            return Ok(result);
        }

        if now_ms() >= expires_at_ms {
            return Err(AppError::TruthId(
                "timed out waiting for the phone to respond".to_string(),
            ));
        }

        tokio::time::sleep(SWEEP_RETRY_INTERVAL).await;
    }
}
