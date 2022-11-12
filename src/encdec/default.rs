use aes_gcm_siv::{
    aead::{AeadInPlace, KeyInit},
    Aes256GcmSiv,
    Nonce, // Or `Aes128GcmSiv`
};

use crate::chunksplitter;

// As per the aes-gcm-siv crate documentation, the auth-tag takes extra 16 bytes
const ENCRYPTION_OVERHEAD: u64 = 16;

// The constant nonce we will be used for all encryption operations.
// Using aes-gcm-siv, repeating nonce means that we get the same cipher for the same plain.
// This is not a problem for this use case.
// Actually, it would allow git to reduce the repo size in such cases.
// Using a per-chunk nonce would imply keeping a running counter and storing it securely,
// which would just complicate things too much without real benefit.
const NONCE: &[u8; 12] = b"123456789012";

// Associated data explained: https://security.stackexchange.com/a/179279
// In principle, we could use a non-constant associated data.
// This couldn't be e.g. file/path, because that would complicate operations on the metadata.
// We could use e.g. [digest(whole-file) x chunk-index] as the associated data,
// but that would mean:
// 1. Important: We need to calculate digest for the whole file before cutting to chunk (slowing).
// 2. Less important: git won't be able to save space by incidental de-dup.
const ASSOCIATED_DATA: &[u8] = b"simple-store";

pub fn new(key: &[u8]) -> Result<AesGcmSivWrapper, String> {
    let cipher = Aes256GcmSiv::new_from_slice(key).map_err(|e| e.to_string())?;
    Ok(AesGcmSivWrapper { cipher })
}

pub struct AesGcmSivWrapper {
    cipher: Aes256GcmSiv,
}

impl chunksplitter::ChunkSizeDictator for AesGcmSivWrapper {
    fn adjust_size(current_size: u64) -> Result<u64, String> {
        if current_size <= ENCRYPTION_OVERHEAD {
            Err(std::fmt::format(format_args!(
                "not enough size for encryption overhead: {} <= {}",
                current_size, ENCRYPTION_OVERHEAD
            )))
        } else {
            Ok(current_size - ENCRYPTION_OVERHEAD)
        }
    }
}

impl super::EncDec for AesGcmSivWrapper {
    fn enc(&self, plaintext: &mut Vec<u8>) -> Result<(), String> {
        let nonce = Nonce::from_slice(NONCE);
        self.cipher
            .encrypt_in_place(nonce, ASSOCIATED_DATA, plaintext)
            .map_err(|e| format!("failed to ecnrypt: {}", e))
    }
    fn dec(&self, ciphertext: &mut Vec<u8>) -> Result<(), String> {
        let nonce = Nonce::from_slice(NONCE);
        self.cipher
            .decrypt_in_place(nonce, ASSOCIATED_DATA, ciphertext)
            .map_err(|e| format!("failed to decrypt: {:?}", e))
    }
}
