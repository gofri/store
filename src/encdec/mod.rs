use crate::chunksplitter;

mod default;

pub trait EncDec: chunksplitter::ChunkSizeDictator {
    fn enc(&self, plaintext: &mut Vec<u8>) -> Result<(), String>;
    fn dec(&self, ciphertext: &mut Vec<u8>) -> Result<(), String>;
}

pub fn new(key: &[u8]) -> Result<impl EncDec, String> {
    default::new(key)
}
