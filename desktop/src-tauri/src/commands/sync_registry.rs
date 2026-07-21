use serde::Serialize;

use crate::error::AppError;
use crate::sync_registry;

// Tipo próprio pro Tauri/JS em vez de expor o `CidRecord` gerado pela macro
// `sol!` diretamente — mesma razão do split `TruthIdWireResult`/
// `TruthIdSignResult` em `commands/truthid.rs`: os tipos ABI (`U256`,
// `FixedBytes<32>`) não implementam `serde::Serialize` e não são o formato
// que o frontend quer (strings simples).
#[derive(Serialize)]
pub struct CidRecordResponse {
    pub cid: String,
    pub content_hash: String,
    pub updated_at: u64,
    pub version: u64,
    pub exists: bool,
}

impl From<sync_registry::CidRecord> for CidRecordResponse {
    fn from(record: sync_registry::CidRecord) -> Self {
        CidRecordResponse {
            cid: record.cid,
            content_hash: record.contentHash.to_string(),
            updated_at: record.updatedAt.to::<u64>(),
            version: record.version.to::<u64>(),
            exists: record.exists,
        }
    }
}

/// Lê o registro de sync de um endereço via `eth_call` público — prova de
/// conceito da Fase 8.1 (leitura), sem escrita ainda (Fase 8.2). `None` tanto
/// quando o endereço nunca gravou nada quanto (por enquanto) quando o
/// contrato ainda não foi deployado de verdade.
#[tauri::command]
pub async fn get_sync_record(address: String) -> Result<Option<CidRecordResponse>, AppError> {
    let who = sync_registry::parse_address(&address)?;
    let record = sync_registry::get_record(who).await?;

    Ok(record.map(CidRecordResponse::from))
}
