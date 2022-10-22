use std::path;

mod default;
use self::default::new_default_chunk_splitter;

pub trait BufReader {
    fn read(&mut self) -> Result<bytes::Bytes, String>;
}

pub trait BufReaderIter {
    fn next_reader<'a, 'b>(&'a self) -> std::option::Option<Box<dyn BufReader + 'b>>
    where
        'a: 'b;
}

pub trait ChunkSplitter {
    fn make_iter<'a, 'b>(&'a self) -> Box<dyn BufReaderIter + 'b>
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
