use self::default::{BufReaderIterItem, ChunkSplitterIter};

use std::path;
mod default;

pub trait BufReader: Send {
    fn read(&self) -> Result<Vec<u8>, String>;
}

pub trait BufReaderIntoIterator<'a>:
    IntoIterator<IntoIter = ChunkSplitterIter<'a>, Item = BufReaderIterItem<'a>>
{
}

pub trait ChunkSplitter {
    fn num_chunks(&self) -> u64;
}

pub fn new<'a, 'b, T: 'b>(path: &'a path::Path, chunk_size: u64) -> Result<T, String>
where
    'a: 'b,
    T: ChunkSplitter + From<default::ChunkSplitter<'b>>,
    &'b T: BufReaderIntoIterator<'b>,
{
    Ok(T::from(default::ChunkSplitter::new(path, chunk_size)?))
}

pub trait ChunkSizeDictator {
    fn adjust_size(current_size: u64) -> Result<u64, String>;
}
