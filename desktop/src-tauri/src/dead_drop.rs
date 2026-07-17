use std::time::Duration;

use crate::ipns_key::compute_ipns_name;

/// Gateway público, mesmo default que `extension/src/session/deadDropPolling.ts`
/// usa — sem configuração de gateway próprio nesta PoC.
const GATEWAY_URL: &str = "https://ipfs.io";
const FETCH_TIMEOUT: Duration = Duration::from_secs(10);

/// Uma tentativa de resolver o dead-drop pro `sessionId` dado — port de
/// `tryFetchDeadDrop` (`extension/src/session/deadDropPolling.ts`). O
/// gateway responde `500`, não `404`, quando o nome IPNS ainda não propagou
/// — trata qualquer resposta não-200 (e qualquer erro de rede/DNS/timeout)
/// como "ainda não", nunca lança. `cachebust` na query evita que um CDN na
/// frente do gateway sirva uma resposta de "não encontrado" já em cache
/// mesmo depois do registro ter propagado de verdade.
pub async fn try_fetch_dead_drop(session_id_hex: &str, client: &reqwest::Client) -> Option<Vec<u8>> {
    let ipns_name = compute_ipns_name(session_id_hex).ok()?;
    let cachebust = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_millis();
    let url = format!("{GATEWAY_URL}/ipns/{ipns_name}?cachebust={cachebust}");

    let resp = client.get(&url).timeout(FETCH_TIMEOUT).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.bytes().await.ok().map(|b| b.to_vec())
}
