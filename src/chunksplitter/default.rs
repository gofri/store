use std::{cell::RefCell, path, rc::Rc};

use crate::filestream::{new_chunk_reader, ChunkReader};

struct DefaultChunkSplitter<'a> {
    index: RefCell<u64>,
    total_size: u64,
    chunk_size: u64,
    chunk_reader: Rc<Box<dyn ChunkReader + 'a>>,
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

pub fn new_default_chunk_splitter<'a>(
    path: &'a path::Path,
    chunk_size: u64,
) -> Result<Box<dyn super::ChunkSplitter + 'a>, String> {
    let reader = Box::new(new_chunk_reader(path, chunk_size)?);
    let total_size = get_file_size(path)?;
    Ok(Box::new(DefaultChunkSplitter {
        index: RefCell::new(0),
        total_size,
        chunk_size,
        chunk_reader: Rc::new(reader),
    }))
}

impl<'a> DefaultChunkSplitter<'a> {
    fn done_reading(&self) -> bool {
        return *self.index.borrow() * self.chunk_size > self.total_size;
    }
}

impl<'a> super::ChunkSplitter<'a> for DefaultChunkSplitter<'a> {
    fn next_reader(&'a self) -> Result<Box<dyn super::BufReader + 'a>, super::ReadRes> {
        let index = self.index.replace_with(|old| *old + 1);
        match self.done_reading() {
            true => Err(super::ReadRes::Eof),
            false => Ok(Box::new(SingleChunkReader {
                chunk_reader: Rc::clone(&self.chunk_reader),
                index,
                chunk_size: self.chunk_size,
            })),
        }
    }

    fn total_size(&self) -> u64 {
        self.total_size
    }
}

//

struct SingleChunkReader<'a> {
    chunk_reader: Rc<Box<dyn ChunkReader + 'a>>,
    index: u64,
    chunk_size: u64,
}

impl<'a> super::BufReader<'a> for SingleChunkReader<'a> {
    fn read(&'a mut self) -> Result<bytes::Bytes, String> {
        let mut buf = vec![0u8; self.chunk_size as usize];
        let read_bytes = match self.chunk_reader.read_chunk(self.index, &mut buf) {
            Ok(x) => x,
            Err(e) => {
                return Err(std::fmt::format(format_args!(
                    "failed to read chunk: {}",
                    e
                )))
            }
        };

        Ok(bytes::Bytes::from(buf).slice(0..read_bytes))
    }
}
