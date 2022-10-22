use std::{cell::RefCell, path, rc::Rc};

use crate::filestream::{new_chunk_reader, ChunkReader};

struct DefaultChunkSplitter<'a> {
    total_size: u64,
    chunk_size: u64,
    chunk_reader: Rc<dyn ChunkReader + 'a>,
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
) -> Result<Box<dyn super::ChunkSplitter + 'b>, String>
where
    'a: 'b,
{
    let chunk_reader = Rc::new(new_chunk_reader(path, chunk_size)?);
    let total_size = get_file_size(path)?;
    Ok(Box::new(DefaultChunkSplitter {
        total_size,
        chunk_size,
        chunk_reader,
    }))
}

impl<'c> super::ChunkSplitter<'c> for DefaultChunkSplitter<'c> {
    // TODO c
    fn total_size(&self) -> u64 {
        self.total_size
    }

    fn make_iter<'a, 'b>(&'a self) -> super::BufReaderIntoIterBound<'b>
    where
        'a: 'b,
    {
        Box::new(ChunkReaderIter {
            index: RefCell::new(0),
            total_size: self.total_size,
            chunk_size: self.chunk_size,
            chunk_reader: Rc::clone(&self.chunk_reader),
        })
    }
}
impl<'a> super::BufReaderIntoIter<'a> for DefaultChunkSplitter<'a> {}
impl<'a> IntoIterator for DefaultChunkSplitter<'a> {
    type Item = super::BufReaderIterItem<'a>;
    type IntoIter = super::BufReaderIntoIterBound<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(ChunkReaderIter {
            index: RefCell::new(0),
            total_size: self.total_size,
            chunk_size: self.chunk_size,
            chunk_reader: Rc::clone(&self.chunk_reader),
        })
    }
}

//

struct ChunkReaderIter<'a> {
    index: RefCell<u64>,
    total_size: u64,
    chunk_size: u64,
    chunk_reader: Rc<dyn ChunkReader + 'a>,
}
impl<'a> ChunkReaderIter<'a> {
    fn done_reading(&self) -> bool {
        return *self.index.borrow() * self.chunk_size > self.total_size;
    }
}

impl<'a> super::BufReaderIter<'a> for ChunkReaderIter<'a> {}
impl<'a> Iterator for ChunkReaderIter<'a> {
    type Item = super::BufReaderIterItem<'a>;
    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        let index = self.index.replace_with(|old| *old + 1);
        match self.done_reading() {
            true => None,
            false => Some(Box::new(SingleChunkReader {
                chunk_reader: Rc::clone(&self.chunk_reader),
                index,
                chunk_size: self.chunk_size,
            })),
        }
    }
}
//

struct SingleChunkReader<'a> {
    index: u64,
    chunk_size: u64,
    chunk_reader: Rc<dyn ChunkReader + 'a>,
}

impl super::BufReader for SingleChunkReader<'_> {
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
