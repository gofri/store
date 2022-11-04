use std::io::Result as StdResult;
use std::path::{Path};
use std::sync::Arc;

mod localfile;
use self::localfile::new_local_file_chunker;

pub trait ChunkReader: Send + Sync {
    fn read_chunk(&self, index: u64, buf: &mut [u8]) -> StdResult<usize>;
}

pub fn new_chunk_reader(path: Arc<Path>, chunk_size: u64) -> Result<impl ChunkReader, String> {
    new_local_file_chunker(path, chunk_size)
}
