use std::{cell::RefCell, ops::Add, path, rc::Rc};

use crate::filestream::{new_chunk_reader, ChunkReader};

use super::BufReader;

struct DefaultChunkSplitter {
    index: RefCell<u64>,
    chunk_size: u64,
    chunk_reader: Rc<Box<dyn ChunkReader>>,
}

pub fn new_default_chunk_splitter(
    path: &path::Path,
    chunk_size: u64,
) -> Result<Box<dyn super::ChunkSplitter>, String> {
    let reader = Box::new(new_chunk_reader(path::PathBuf::from(path), chunk_size)?);
    Ok(Box::new(DefaultChunkSplitter {
        index: RefCell::new(0),
        chunk_size,
        chunk_reader: Rc::new(reader),
    }))
}

impl super::ChunkSplitter for DefaultChunkSplitter {
    fn next_reader(&self) -> Box<dyn BufReader> {
        let index = *self.index.borrow();
        _ = self.index.borrow_mut().add(1);
        Box::new(SingleChunkReader {
            chunk_reader: Rc::clone(&self.chunk_reader),
            index,
            chunk_size: self.chunk_size,
        })
    }
}

//

struct SingleChunkReader {
    chunk_reader: Rc<Box<dyn ChunkReader>>,
    index: u64,
    chunk_size: u64,
}

impl super::BufReader for SingleChunkReader {
    fn read(&mut self) -> Result<bytes::Bytes, String> {
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
