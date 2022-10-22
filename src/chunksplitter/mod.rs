use std::path;

mod default;
use self::default::new_default_chunk_splitter;

pub trait BufReader {
    fn read(&mut self) -> Result<bytes::Bytes, String>;
}

pub trait BufReaderIter: Iterator {
    fn next_reader<'a, 'b>(&'a self) -> std::option::Option<Box<dyn BufReader + 'b>>
    where
        'a: 'b;
}

type BufReaderIterItem<'a> = Box<dyn BufReader + 'a>;

pub trait ChunkSplitter {
    fn make_iter<'a, 'b>(&'a self) -> Box<dyn BufReaderIter<Item = BufReaderIterItem> + 'b>
    where
        'a: 'b;
    fn total_size(&self) -> u64;
}

pub fn new_chunk_splitter<'a, 'b>(
    path: &'a path::Path,
    chunk_size: u64,
) -> Result<Box<dyn ChunkSplitter + 'b>, String>
where
    'a: 'b,
{
    new_default_chunk_splitter(path, chunk_size)
}
