use std::path;

mod default;

use self::default::new_default_chunk_splitter;

pub trait BufReader: Send {
    fn read(&self) -> Result<Vec<u8>, String>;
}

type BufReaderIterItem<'a> = Box<dyn BufReader + 'a>;
pub trait BufReaderIter<'a>: Iterator<Item = BufReaderIterItem<'a>> {}

pub trait ChunkSplitter<'a>: BufReaderIter<'a> {
    fn total_size(&self) -> u64;
}

pub fn new<'a, 'b>(
    path: &'a path::Path,
    chunk_size: u64,
) -> Result<Box<dyn ChunkSplitter<'b> + 'b>, String>
where
    'a: 'b,
{
    new_default_chunk_splitter(path, chunk_size)
}
