use ed25519_dalek::SigningKey;
use hkdf::Hkdf;
use sha2::Sha256;

/// Recalcula o nome IPNS (`k51...`) onde o Mobile publica o dead-drop de uma
/// sessão cross-device — port direto de `computeIpnsName`
/// (`extension/src/session/ipnsKey.ts`) / `ipns_key_service.dart` (TruthID).
/// O Practice Valuation nunca gera nem vê a chave privada — só recalcula a
/// metade pública da mesma derivação determinística a partir do `sessionId`
/// já embutido no QR, sem nenhuma troca extra com o celular.
///
/// `HKDF_SALT`/`HKDF_INFO` precisam bater byte-a-byte com os outros dois
/// lados (Dart/TS) — é o elo crítico de interop. Protobuf/multihash/CID são
/// montados manualmente (sem crate), mesma decisão consciente que o Dart já
/// tomou: só 2 campos fixos, não vale a pena um encoder protobuf genérico.
const HKDF_SALT: &[u8] = b"TruthID Vault IPNS";
const HKDF_INFO: &[u8] = b"dead-drop-key-v1";

const KEY_TYPE_ED25519: u8 = 1;
const MULTICODEC_LIBP2P_KEY: u8 = 0x72;
const MULTIHASH_IDENTITY: u8 = 0x00;
const CID_VERSION_1: u8 = 0x01;
const BASE36_ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

pub fn compute_ipns_name(session_id_hex: &str) -> Result<String, String> {
    let session_id_bytes = hex::decode(session_id_hex).map_err(|e| e.to_string())?;

    let hk = Hkdf::<Sha256>::new(Some(HKDF_SALT), &session_id_bytes);
    let mut seed = [0u8; 32];
    hk.expand(HKDF_INFO, &mut seed)
        .map_err(|_| "HKDF expand failed".to_string())?;

    let signing_key = SigningKey::from_bytes(&seed);
    let public_key = signing_key.verifying_key().to_bytes();

    // Protobuf `PublicKey` do libp2p (`crypto.proto`): `Type` (varint,
    // Ed25519=1) + `Data` (bytes = chave pública, 32 bytes).
    let mut public_key_protobuf = Vec::with_capacity(4 + public_key.len());
    public_key_protobuf.push(0x08);
    public_key_protobuf.push(KEY_TYPE_ED25519);
    public_key_protobuf.push(0x12);
    public_key_protobuf.push(public_key.len() as u8);
    public_key_protobuf.extend_from_slice(&public_key);

    // Multihash "identity" (código 0x00) — válido porque o protobuf de 36
    // bytes de uma chave pública Ed25519 sempre cabe no limite de 42 bytes
    // da regra de peer-id do libp2p.
    let mut multihash = Vec::with_capacity(2 + public_key_protobuf.len());
    multihash.push(MULTIHASH_IDENTITY);
    multihash.push(public_key_protobuf.len() as u8);
    multihash.extend_from_slice(&public_key_protobuf);

    let mut cid = Vec::with_capacity(2 + multihash.len());
    cid.push(CID_VERSION_1);
    cid.push(MULTICODEC_LIBP2P_KEY);
    cid.extend_from_slice(&multihash);

    Ok(format!("k{}", base36_encode(&cid)))
}

/// Codificação base36 "estilo base58" (mesma família de algoritmo do
/// `base-x`/multibase): trata os bytes como um inteiro big-endian e converte
/// pra base 36 via o algoritmo clássico de "multiply-add" sobre um vetor de
/// dígitos (evita depender de uma crate de bignum pra um valor usado uma vez
/// só aqui); bytes `0x00` à esquerda viram `'0'` à esquerda no resultado, em
/// vez de serem absorvidos pelo valor numérico — mesmo comportamento de
/// `_base36Encode` no Dart.
fn base36_encode(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    let mut digits: Vec<u8> = vec![0];
    for &byte in bytes {
        let mut carry = byte as u32;
        for d in digits.iter_mut() {
            let val = (*d as u32) * 256 + carry;
            *d = (val % 36) as u8;
            carry = val / 36;
        }
        while carry > 0 {
            digits.push((carry % 36) as u8);
            carry /= 36;
        }
    }

    let leading_zero_bytes = bytes.iter().take_while(|&&b| b == 0).count();
    let mut result = String::with_capacity(leading_zero_bytes + digits.len());
    result.extend(std::iter::repeat_n('0', leading_zero_bytes));
    for &d in digits.iter().rev() {
        result.push(BASE36_ALPHABET[d as usize] as char);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mesmo vetor cruzado usado em `mobile/test/services/ipns_key_service_test.dart`
    // e `extension/src/session/ipnsKey.test.ts` — validado contra um Kubo
    // 0.42.0 real (a chave derivada foi importada de verdade via `POST
    // /api/v0/key/import?format=libp2p-protobuf-cleartext`, e o `Id` que o
    // Kubo devolveu bateu byte-a-byte com este valor). Fecha o loop de
    // interoperabilidade Rust↔Kubo↔Dart↔TS de forma determinística e offline.
    #[test]
    fn matches_the_fixture_validated_against_real_kubo() {
        let session_id_hex = "000102030405060708090a0b0c0d0e0f";
        let expected = "k51qzi5uqu5diyq5i3xkj8knjqw2jewheim4x3ghwm0a8bh7t6ty3zv9x5f3oh";

        assert_eq!(compute_ipns_name(session_id_hex).unwrap(), expected);
    }

    #[test]
    fn is_deterministic() {
        let session_id_hex = "000102030405060708090a0b0c0d0e0f";
        assert_eq!(
            compute_ipns_name(session_id_hex).unwrap(),
            compute_ipns_name(session_id_hex).unwrap()
        );
    }

    #[test]
    fn different_session_ids_derive_different_names() {
        let a = compute_ipns_name("000102030405060708090a0b0c0d0e0f").unwrap();
        let b = compute_ipns_name("0f0e0d0c0b0a09080706050403020100").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn always_starts_with_the_base36_multibase_prefix() {
        let name = compute_ipns_name("000102030405060708090a0b0c0d0e0f").unwrap();
        assert!(name.starts_with('k'));
    }

    #[test]
    fn rejects_invalid_hex() {
        assert!(compute_ipns_name("not-hex").is_err());
    }
}
