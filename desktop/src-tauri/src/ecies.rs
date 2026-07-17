use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key};
use k256::ecdsa::SigningKey;
use k256::elliptic_curve::ecdh::diffie_hellman;
use k256::elliptic_curve::sec1::FromEncodedPoint;
use k256::PublicKey;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

/// ECIES genérico (secp256k1 ECDH + SHA-256 + AES-256-GCM) — mesmo esquema que
/// `encrypt_bytes_for_device` no TruthID Desktop (`desktop/src-tauri/src/lib.rs`)
/// já usa e que os testes de lá (`dart_produced_blob_decrypts_correctly`) já
/// provam interoperar com Dart/TS. O Practice Valuation só precisa do lado
/// `decrypt` — quem cifra é sempre o celular, respondendo ao QR.
///
/// Formato do blob: ephemeral_pubkey(33 bytes comprimida) || nonce(12 bytes)
/// || ciphertext+tag.
const EPHEMERAL_PUBKEY_LEN: usize = 33;
const NONCE_LEN: usize = 12;

pub fn decrypt(blob: &[u8], recipient_priv_hex: &str) -> Result<Vec<u8>, String> {
    if blob.len() < EPHEMERAL_PUBKEY_LEN + NONCE_LEN {
        return Err("blob too short to be a valid ECIES payload".to_string());
    }

    let priv_bytes = hex::decode(recipient_priv_hex.trim_start_matches("0x"))
        .map_err(|e| e.to_string())?;
    let recipient_priv =
        SigningKey::from_bytes(priv_bytes.as_slice().into()).map_err(|e| e.to_string())?;

    let ephemeral_pub_bytes = &blob[0..EPHEMERAL_PUBKEY_LEN];
    let nonce_bytes = &blob[EPHEMERAL_PUBKEY_LEN..EPHEMERAL_PUBKEY_LEN + NONCE_LEN];
    let ciphertext = &blob[EPHEMERAL_PUBKEY_LEN + NONCE_LEN..];

    let point = k256::EncodedPoint::from_bytes(ephemeral_pub_bytes).map_err(|e| e.to_string())?;
    let ephemeral_pub = PublicKey::from_encoded_point(&point)
        .into_option()
        .ok_or_else(|| "invalid ephemeral public key".to_string())?;

    let shared = diffie_hellman(recipient_priv.as_nonzero_scalar(), ephemeral_pub.as_affine());
    let aes_key_bytes = Sha256::digest(shared.raw_secret_bytes());
    let aes_key = Key::<Aes256Gcm>::from_slice(&aes_key_bytes);
    let cipher = Aes256Gcm::new(aes_key);

    cipher
        .decrypt(nonce_bytes.into(), ciphertext)
        .map_err(|_| "ECIES decrypt failed".to_string())
}

/// Gera um par de chaves efêmero (secp256k1) para uma nova sessão cross-device
/// — a privada fica só com o requisitante (nunca vai pro QR), a pública
/// comprimida vai no QR pro celular cifrar a resposta contra ela.
pub fn generate_ephemeral_keypair() -> (String, String) {
    let priv_key = SigningKey::random(&mut OsRng);
    let pub_key = priv_key.verifying_key();
    let priv_hex = hex::encode(priv_key.to_bytes());
    let pub_hex = format!(
        "0x{}",
        hex::encode(pub_key.to_encoded_point(true).as_bytes())
    );
    (priv_hex, pub_hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mesmo vetor cruzado fixo do TruthID Desktop
    // (`desktop/src-tauri/src/lib.rs::dart_produced_blob_decrypts_correctly`),
    // gerado uma vez rodando o `EciesService.encrypt` real do Dart contra uma
    // chave privada de teste determinística. Prova que este `decrypt` bate
    // byte-a-byte com o que Dart/TS/Rust já provaram interoperar entre si,
    // sem precisar de um celular real pra este teste específico.
    #[test]
    fn decrypts_dart_produced_blob() {
        let recipient_priv_hex =
            "ebea44b99557c83965e6152a1393a5c6d74fe114f0a626f51bb2349e815136b2";
        let blob_b64 = "AqQAXxG3rw53DVihUXbTzqHcENoLZGbHFsnNHPFvZduk0FF00QwiZMLWLCs8q19CzAj4kYiWXr1jUTn0tUxh1ibNVbwPQiCSBZAJdH1eqE86qT1Na5ytsA==";
        let expected_plaintext_hex =
            "747275746869642d7661756c742d656e7472792d66697874757265";

        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let blob = STANDARD.decode(blob_b64).expect("valid base64");

        let plaintext =
            decrypt(&blob, recipient_priv_hex).expect("should decrypt the Dart-produced blob");

        assert_eq!(hex::encode(&plaintext), expected_plaintext_hex);
    }

    #[test]
    fn round_trips_with_generated_ephemeral_keypair() {
        // Gera um par "destinatário" fixo só pra este teste (não o par
        // efêmero da sessão — aqui simula o celular cifrando pra nós).
        let recipient_priv = SigningKey::random(&mut OsRng);
        let recipient_pub_hex = hex::encode(recipient_priv.verifying_key().to_encoded_point(true).as_bytes());
        let recipient_priv_hex = hex::encode(recipient_priv.to_bytes());

        let plaintext = b"cross-device sign-request result";
        let blob = encrypt_for_test(plaintext, &recipient_pub_hex);

        let decrypted = decrypt(&blob, &recipient_priv_hex).expect("should decrypt");
        assert_eq!(decrypted, plaintext);
    }

    // Réplica mínima do lado "cifra" (papel do celular), só pra este teste
    // poder montar um blob válido sem depender de um dispositivo real.
    fn encrypt_for_test(plaintext: &[u8], recipient_pub_hex: &str) -> Vec<u8> {
        use aes_gcm::aead::AeadCore;

        let recipient_pub_bytes = hex::decode(recipient_pub_hex).unwrap();
        let point = k256::EncodedPoint::from_bytes(&recipient_pub_bytes).unwrap();
        let recipient_pub = PublicKey::from_encoded_point(&point).unwrap();

        let ephemeral_priv = SigningKey::random(&mut OsRng);
        let ephemeral_pub = ephemeral_priv.verifying_key();

        let shared = diffie_hellman(ephemeral_priv.as_nonzero_scalar(), recipient_pub.as_affine());
        let aes_key_bytes = Sha256::digest(shared.raw_secret_bytes());
        let aes_key = Key::<Aes256Gcm>::from_slice(&aes_key_bytes);
        let cipher = Aes256Gcm::new(aes_key);

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();

        let ephemeral_bytes = ephemeral_pub.to_encoded_point(true);
        let mut blob = Vec::with_capacity(33 + 12 + ciphertext.len());
        blob.extend_from_slice(ephemeral_bytes.as_bytes());
        blob.extend_from_slice(&nonce);
        blob.extend_from_slice(&ciphertext);
        blob
    }
}
