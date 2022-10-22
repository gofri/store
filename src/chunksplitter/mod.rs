use std::path;

mod default;

use self::default::new_default_chunk_splitter;

pub trait BufReader: Send {
    fn read(&mut self) -> Result<bytes::Bytes, String>;
}

type BufReaderIterItem<'a> = Box<dyn BufReader + 'a>;
pub trait BufReaderIter<'a>: Iterator<Item = BufReaderIterItem<'a>> {}

pub trait ChunkSplitter<'a>: BufReaderIter<'a> {
    fn total_size(&self) -> u64;
}

pub fn new<'b>(
    path: path::PathBuf,
    chunk_size: u64,
) -> Result<Box<dyn ChunkSplitter<'b> + 'b>, String> {
    new_default_chunk_splitter(path, chunk_size)
}
