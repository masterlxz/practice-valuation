use alloy_primitives::Address;
use alloy_sol_types::{sol, SolCall, SolError};
use serde_json::json;

use crate::error::AppError;

// Interface ABI do `contracts/src/SyncRegistry.sol` — só o que este cliente
// precisa (leitura). `updateRecord` entra aqui quando a Fase 8.2 (escrita via
// canal delegado do TruthID) for implementada.
sol! {
    struct CidRecord {
        string cid;
        bytes32 contentHash;
        uint256 updatedAt;
        uint256 version;
        bool exists;
    }

    function getRecord(address who) external view returns (CidRecord memory);

    error RecordNotFound(address who);
}

/// RPC pública da Base Sepolia (testnet) — sem chave, mesmo endpoint já usado
/// como fallback pelo resto do ecossistema TruthID (`desktop/src/config/wagmi.ts`
/// de lá). Base Mainnet só entra quando o fluxo completo da Fase 8 estiver
/// provado ponta a ponta no testnet.
const RPC_URL: &str = "https://sepolia.base.org";

/// Preenchido depois do deploy real do `SyncRegistry` (passo manual, feito com
/// o dono do projeto presente — ver Fase 8.1 no PROJECT_STATE.md). Enquanto
/// não houver deploy, qualquer chamada de leitura falha contra este endereço
/// vazio (não existe contrato lá) — comportamento esperado, não um bug.
const SYNC_REGISTRY_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

/// Busca o registro de sync de um endereço via `eth_call` público — não
/// precisa de assinatura, qualquer RPC serve. Retorna `None` (não `Err`)
/// quando o contrato reverte com `RecordNotFound`, já que "não tem registro
/// ainda" é um resultado esperado, não uma falha.
pub async fn get_record(who: Address) -> Result<Option<CidRecord>, AppError> {
    let calldata = getRecordCall { who }.abi_encode();

    let client = reqwest::Client::new();
    let response: serde_json::Value = client
        .post(RPC_URL)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_call",
            "params": [
                { "to": SYNC_REGISTRY_ADDRESS, "data": format!("0x{}", hex::encode(&calldata)) },
                "latest"
            ]
        }))
        .send()
        .await?
        .json()
        .await?;

    if let Some(error) = response.get("error") {
        // Reverte com `RecordNotFound(address)` → "ainda não tem registro",
        // não é um erro de verdade. Qualquer outro revert/erro de RPC propaga.
        let revert_data = error
            .get("data")
            .and_then(|d| d.as_str())
            .and_then(|hex_str| hex::decode(hex_str.trim_start_matches("0x")).ok());
        if let Some(bytes) = revert_data {
            if RecordNotFound::abi_decode(&bytes).is_ok() {
                return Ok(None);
            }
        }

        let message = error
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("unknown RPC error");
        return Err(AppError::Rpc(message.to_string()));
    }

    let result_hex = response
        .get("result")
        .and_then(|r| r.as_str())
        .ok_or_else(|| AppError::Rpc("RPC response missing 'result' field".to_string()))?;
    let bytes = hex::decode(result_hex.trim_start_matches("0x"))
        .map_err(|e| AppError::Rpc(format!("invalid hex in RPC result: {e}")))?;

    let record = getRecordCall::abi_decode_returns(&bytes)
        .map_err(|e| AppError::Rpc(format!("failed to decode contract return value: {e}")))?;

    Ok(Some(record))
}

pub fn parse_address(raw: &str) -> Result<Address, AppError> {
    raw.parse()
        .map_err(|_| AppError::InvalidAddress(raw.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_sol_types::SolValue;

    // Vetor de teste "à mão" — sem rede: confirma que o calldata montado pro
    // eth_call tem a forma esperada (seletor de 4 bytes + address em 32,
    // padding à esquerda) antes de confiar nisso contra o contrato real.
    #[test]
    fn get_record_calldata_has_selector_plus_padded_address() {
        let who: Address = "0x000000000000000000000000000000000000dEaD".parse().unwrap();
        let calldata = getRecordCall { who }.abi_encode();

        assert_eq!(calldata.len(), 4 + 32);
        assert_eq!(&calldata[4..16], &[0u8; 12]); // padding do address
        assert_eq!(&calldata[16..], who.as_slice());
    }

    #[test]
    fn decodes_a_full_cid_record_return_value() {
        let record = CidRecord {
            cid: "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string(),
            contentHash: [0x11u8; 32].into(),
            updatedAt: alloy_primitives::U256::from(1_752_600_000u64),
            version: alloy_primitives::U256::from(3u64),
            exists: true,
        };

        // Round-trip: codifica o valor de retorno como o contrato faria e
        // confirma que `abi_decode_returns` (usado de verdade em `get_record`)
        // recompõe exatamente os mesmos campos.
        let encoded = record.abi_encode();
        let decoded = getRecordCall::abi_decode_returns(&encoded).unwrap();

        assert_eq!(decoded.cid, record.cid);
        assert_eq!(decoded.contentHash, record.contentHash);
        assert_eq!(decoded.updatedAt, record.updatedAt);
        assert_eq!(decoded.version, record.version);
        assert_eq!(decoded.exists, record.exists);
    }

    #[test]
    fn parse_address_rejects_garbage() {
        assert!(parse_address("not-an-address").is_err());
    }

    #[test]
    fn parse_address_accepts_checksummed_and_lowercase() {
        assert!(parse_address("0x000000000000000000000000000000000000dEaD").is_ok());
        assert!(parse_address("0x000000000000000000000000000000000000dead").is_ok());
    }
}
