use std::path;

mod default;
use self::default::new_default_chunk_splitter;

pub trait BufReader {
    fn read(&mut self) -> Result<bytes::Bytes, String>;
}

pub trait ChunkSplitter {
    fn next_reader(&self) -> std::option::Option<Box<dyn BufReader>>;
    fn total_size(&self) -> u64;
}

pub fn new_chunk_splitter(
    path: &path::Path,
    chunk_size: u64,
) -> Result<Box<dyn ChunkSplitter>, String> {
    new_default_chunk_splitter(path, chunk_size)
}
