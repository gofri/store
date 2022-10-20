use std::path;

mod chunksplitter;
use self::chunksplitter::new_default_chunk_splitter;

pub trait BufReader {
    fn read(&mut self) -> Result<bytes::Bytes, String>;
}

pub trait ChunkSplitter {
    fn next_reader(&self) -> Box<dyn BufReader>;
}

pub fn new_chunk_splitter(
    path: &path::Path,
    chunk_size: u64,
) -> Result<Box<dyn ChunkSplitter>, String> {
    new_default_chunk_splitter(path, chunk_size)
}
