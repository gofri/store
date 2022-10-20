use std::path;

mod default;
use self::default::new_default_chunk_splitter;

pub trait BufReader<'a> {
    fn read(&'a mut self) -> Result<bytes::Bytes, String>;
}

#[derive(Debug)]
pub enum ReadRes {
    Eof,
}

pub trait ChunkSplitter<'a> {
    fn next_reader(&'a self) -> Result<Box<dyn BufReader + 'a>, ReadRes>;
    fn total_size(&self) -> u64;
}

pub fn new_chunk_splitter<'a>(
    path: &'a path::Path,
    chunk_size: u64,
) -> Result<Box<dyn ChunkSplitter + 'a>, String> {
    new_default_chunk_splitter(path, chunk_size)
}
