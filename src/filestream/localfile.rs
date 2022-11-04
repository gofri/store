use std::fs::File;
use std::io::Result as StdResult;
use std::io::{Read, Seek};
use std::path::Path;

pub struct LocalFileChunker<'a> {
    path: &'a Path,
    pub chunk_size: u64,
}

impl LocalFileChunker<'_> {
    fn index_to_offset(&self, index: u64) -> u64 {
        self.chunk_size * index
    }

    fn open_file_at_index(&self, index: u64) -> StdResult<File> {
        let offset = self.index_to_offset(index);
        match File::open(self.path) {
            Ok(mut f) => {
                f.seek(std::io::SeekFrom::Start(offset))?;
                Ok(f)
            }
            Err(e) => Err(e),
        }
    }
}

impl super::ChunkReader for LocalFileChunker<'_> {
    fn read_chunk(&self, index: u64, buf: &mut [u8]) -> StdResult<usize> {
        let chunked_buf = &mut buf[0..self.chunk_size as usize];
        self.open_file_at_index(index)?.read(chunked_buf)
    }
}

pub fn new_local_file_chunker(path: &Path, chunk_size: u64) -> Result<LocalFileChunker, String> {
    if chunk_size == 0 {
        return Err("chunk size must not be zero".to_string());
    }

    Ok(LocalFileChunker { path, chunk_size })
}
