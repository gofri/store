use std::{cell::RefCell, path, rc::Rc};

use crate::filestream::{new_chunk_reader, ChunkReader};

use super::BufReader;

struct DefaultChunkSplitter {
    index: RefCell<u64>,
    total_size: u64,
    chunk_size: u64,
    chunk_reader: Rc<Box<dyn ChunkReader>>,
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

pub fn new_default_chunk_splitter(
    path: &path::Path,
    chunk_size: u64,
) -> Result<Box<dyn super::ChunkSplitter>, String> {
    let reader = Box::new(new_chunk_reader(path::PathBuf::from(path), chunk_size)?);
    let total_size = get_file_size(path)?;
    Ok(Box::new(DefaultChunkSplitter {
        index: RefCell::new(0),
        total_size,
        chunk_size,
        chunk_reader: Rc::new(reader),
    }))
}

impl DefaultChunkSplitter {
    fn done_reading(&self) -> bool {
        return *self.index.borrow() * self.chunk_size > self.total_size;
    }
}

impl super::ChunkSplitter for DefaultChunkSplitter {
    fn next_reader(&self) -> Result<Box<dyn BufReader>, super::ReadRes> {
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
