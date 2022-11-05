use std::{cell::RefCell, path, sync::Arc};

use crate::filestream::{new_chunk_reader, ChunkReader};

pub struct ChunkSplitter<'a> {
    index: RefCell<u64>,
    total_size: u64,
    chunk_size: u64,
    chunk_reader: Arc<dyn ChunkReader + 'a>,
}

pub fn _new<'a, 'b>(path: &'a path::Path, chunk_size: u64) -> Result<ChunkSplitter<'b>, String>
where
    'a: 'b,
{
    let total_size = get_file_size(path)?;
    let chunk_reader = Arc::new(new_chunk_reader(path, chunk_size)?);
    Ok(ChunkSplitter {
        index: RefCell::new(0),
        total_size,
        chunk_size,
        chunk_reader,
    })
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

impl super::FileSizer for ChunkSplitter<'_> {
    fn total_size(&self) -> u64 {
        self.total_size
    }
}

impl ChunkSplitter<'_> {
    fn is_valid_chunk(&self, chunk_index: u64) -> bool {
        chunk_index * self.chunk_size < self.total_size
    }
}

impl<'a> super::BufReaderIter<'a> for ChunkSplitter<'a> {}
impl<'a> Iterator for ChunkSplitter<'a> {
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

struct Helper<'a> {
    index: u64,
    chunk_reader: Arc<dyn ChunkReader + 'a>,
}

struct SingleChunkReader<'a> {
    index: u64,
    chunk_reader: Arc<dyn ChunkReader + 'a>,
}

impl super::BufReader for SingleChunkReader<'_> {
    fn read(&self) -> Result<Vec<u8>, String> {
        self.chunk_reader
            .read_chunk(self.index)
            .map_err(|e| std::fmt::format(format_args!("failed to read chunk: {}", e)))
    }
}
