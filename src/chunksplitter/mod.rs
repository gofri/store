use self::default::{BufReaderIterItem, ChunkSplitter, _new};

use std::path;
mod default;

pub struct ChunkSplitterIter<'a> {
    index: u64,
    splitter: &'a ChunkSplitter<'a>,
}

pub trait BufReader: Send {
    fn read(&self) -> Result<Vec<u8>, String>;
}

pub trait BufReaderIntoIterator<'a>:
    IntoIterator<IntoIter = ChunkSplitterIter<'a>, Item = BufReaderIterItem<'a>>
{
}

pub fn new<'a, 'b, T: 'b>(path: &'a path::Path, chunk_size: u64) -> Result<T, String>
where
    'a: 'b,
    T: From<ChunkSplitter<'b>>,
    &'b T: IntoIterator,
{
    Ok(T::from(_new(path, chunk_size)?))
}
