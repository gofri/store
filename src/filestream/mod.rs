use std::io::Result as StdResult;
use std::path::Path;

mod localfile;
use self::localfile::new_local_file_chunker;

pub trait ChunkReader {
    fn read_chunk(&self, index: u64, buf: &mut [u8]) -> StdResult<usize>;
}

pub fn new_chunk_reader(path: &Path, chunk_size: u64) -> Result<impl ChunkReader + '_, String> {
    new_local_file_chunker(path, chunk_size)
}
