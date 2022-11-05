use std::{cell::RefCell, path, sync::Arc};

use crate::filestream::{new_chunk_reader, ChunkReader};

struct DefaultChunkSplitter<'a> {
    index: RefCell<u64>,
    total_size: u64,
    chunk_size: u64,
    chunk_reader: Arc<dyn ChunkReader + 'a>,
}

fn get_file_size(path: &path::Path) -> Result<u64, String> {
    match std::fs::metadata(path) {
        Ok(m) => Ok(m.len()),
        Err(e) => Err(std::fmt::format(format_args!(
            "failed to get file ({}) size: {}",
            path.display(),
            e
        ))),
    }
}

pub fn new_default_chunk_splitter<'a, 'b>(
    path: &'a path::Path,
    chunk_size: u64,
) -> Result<Box<dyn super::ChunkSplitter<'b> + 'b>, String>
where
    'a: 'b,
{
    let total_size = get_file_size(path)?;
    let chunk_reader = Arc::new(new_chunk_reader(path, chunk_size)?);
    Ok(Box::new(DefaultChunkSplitter {
        index: RefCell::new(0),
        total_size,
        chunk_size,
        chunk_reader,
    }))
}

impl DefaultChunkSplitter<'_> {
    fn is_valid_chunk(&self, chunk_index: u64) -> bool {
        chunk_index * self.chunk_size < self.total_size
    }
}

impl<'a> super::ChunkSplitter<'a> for DefaultChunkSplitter<'a> {
    fn total_size(&self) -> u64 {
        self.total_size
    }
}

impl<'a> super::BufReaderIter<'a> for DefaultChunkSplitter<'a> {}
impl<'a> Iterator for DefaultChunkSplitter<'a> {
    type Item = super::BufReaderIterItem<'a>;
    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        let index = self.index.replace_with(|old| *old + 1);
        if self.is_valid_chunk(index) {
            Some(Box::new(SingleChunkReader {
                chunk_reader: Arc::clone(&self.chunk_reader),
                index,
            }))
        } else {
            self.index.replace(0);
            None
        }
    }
}

struct SingleChunkReader<'a> {
    index: u64,
    chunk_reader: Arc<dyn ChunkReader + 'a>,
}

impl super::BufReader for SingleChunkReader<'_> {
    fn read(&self) -> Result<Vec<u8>, String> {
        match self.chunk_reader.read_chunk(self.index) {
            Ok(buf) => Ok(buf),
            Err(e) => Err(std::fmt::format(format_args!(
                "failed to read chunk: {}",
                e
            ))),
        }
    }
}
