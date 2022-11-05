use std::{path, sync::Arc};

use crate::filestream::{new_chunk_reader, ChunkReader};

pub fn _new<'a, 'b>(
    path: &'a path::Path,
    chunk_size: u64,
) -> Result<super::ChunkSplitter<'b>, String>
where
    'a: 'b,
{
    if chunk_size == 0 {
        return Err(String::from("Invalid chunk size (0)"));
    }
    let total_size = get_file_size(path)?;
    let chunk_reader = Arc::new(new_chunk_reader(path, chunk_size)?);
    Ok(super::ChunkSplitter {
        num_chunks: total_size / chunk_size,
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

impl super::ChunkSplitter<'_> {
    fn is_valid_chunk(&self, chunk_index: u64) -> bool {
        chunk_index <= self.num_chunks
    }
}

impl<'a> super::BufReaderIntoIterator<'a> for &'a super::ChunkSplitter<'a> {}
impl<'a> IntoIterator for &'a super::ChunkSplitter<'a> {
    type Item = BufReaderIterItem<'a>;
    type IntoIter = super::ChunkSplitterIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        super::ChunkSplitterIter {
            index: 0,
            splitter: self,
        }
    }
}

pub type BufReaderIterItem<'a> = Box<dyn super::BufReader + 'a>;

impl<'a> Iterator for super::ChunkSplitterIter<'a> {
    type Item = BufReaderIterItem<'a>;
    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        let index = self.index;
        self.index += 1;
        if self.splitter.is_valid_chunk(index) {
            Some(Box::new(SingleChunkReader {
                chunk_reader: Arc::clone(&self.splitter.chunk_reader),
                index,
            }))
        } else {
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
        self.chunk_reader
            .read_chunk(self.index)
            .map_err(|e| std::fmt::format(format_args!("failed to read chunk: {}", e)))
    }
}
