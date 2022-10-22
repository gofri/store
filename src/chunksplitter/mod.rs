use std::path;

mod default;

use self::default::new_default_chunk_splitter;

pub trait BufReader {
    fn read(&mut self) -> Result<bytes::Bytes, String>;
}

type BufReaderIterItem<'a> = Box<dyn BufReader + 'a>;
pub trait BufReaderIter<'a>: Iterator<Item = BufReaderIterItem<'a>> {}
type BufReaderIntoIterBound<'a> = Box<dyn BufReaderIter<'a> + 'a>;

pub trait BufReaderIntoIter<'a>:
    IntoIterator<Item = BufReaderIterItem<'a>, IntoIter = BufReaderIntoIterBound<'a>>
{
}
pub trait ChunkSplitter<'c> /* : BufReaderIntoIter<'c> */ {
    // TODO back to a
    fn make_iter<'a, 'b>(&'a self) -> BufReaderIntoIterBound<'b>
    where
        'a: 'b;
    fn total_size(&self) -> u64;
}

pub fn new<'a, 'b>(
    path: &'a path::Path,
    chunk_size: u64,
) -> Result<Box<dyn ChunkSplitter + 'b>, String>
where
    'a: 'b,
{
    new_default_chunk_splitter(path, chunk_size)
}
