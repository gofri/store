use std::{path, sync::Arc};

mod default;

pub struct ChunkSplitter<'a> {
    num_chunks: u64,
    pub chunk_reader: Arc<dyn ChunkReader + 'a>, // TODO ugly pub
}

use crate::filestream::ChunkReader;

use self::default::_new;

pub trait BufReader: Send {
    fn read(&self) -> Result<Vec<u8>, String>;
}

type BufReaderIterItem<'a> = Box<dyn BufReader + 'a>;
pub trait BufReaderIter<'a>: Iterator<Item = BufReaderIterItem<'a>> {}

pub fn new<'a, 'b>(path: &'a path::Path, chunk_size: u64) -> Result<ChunkSplitter<'b>, String>
where
    'a: 'b,
{
    _new(path, chunk_size)
}
