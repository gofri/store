use std::path;

mod default;

use self::default::{ChunkSplitter, _new};

pub trait BufReader: Send {
    fn read(&self) -> Result<Vec<u8>, String>;
}

pub trait FileSizer {
    fn total_size(&self) -> u64;
}

type BufReaderIterItem<'a> = Box<dyn BufReader + 'a>;
pub trait BufReaderIter<'a>: Iterator<Item = BufReaderIterItem<'a>> {}

pub fn new<'a, 'b>(path: &'a path::Path, chunk_size: u64) -> Result<ChunkSplitter<'b>, String>
where
    'a: 'b,
{
    _new(path, chunk_size)
}
