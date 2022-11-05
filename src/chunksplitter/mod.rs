use self::default::{BufReaderIterItem, _new};
use crate::filestream::ChunkReader;
use std::{path, sync::Arc};
mod default;

pub struct ChunkSplitter<'a> {
    num_chunks: u64,
    chunk_reader: Arc<dyn ChunkReader + 'a>,
}

pub struct ChunkSplitterIter<'a> {
    index: u64,
    splitter: &'a super::ChunkSplitter<'a>,
}

pub trait BufReader: Send {
    fn read(&self) -> Result<Vec<u8>, String>;
}

pub trait BufReaderIntoIterator<'a>:
    IntoIterator<IntoIter = ChunkSplitterIter<'a>, Item = BufReaderIterItem<'a>>
{
}

pub fn new<'a, 'b>(path: &'a path::Path, chunk_size: u64) -> Result<ChunkSplitter<'b>, String>
where
    'a: 'b,
{
    _new(path, chunk_size)
}
