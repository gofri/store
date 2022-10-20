use std::fs::File;
use std::io::Result as StdResult;
use std::io::{Read, Seek};
use std::path::{self, PathBuf};

pub struct LocalFileChunker {
    path: path::PathBuf,
    pub chunk_size: u64,
}

impl LocalFileChunker {
    fn index_to_offset(&self, index: u64) -> u64 {
        self.chunk_size * index
    }

    fn seek_index(&self, index: u64) -> StdResult<u64> {
        let offset = self.index_to_offset(index);
        match self.open_file() {
            Ok(mut f) => f.seek(std::io::SeekFrom::Start(offset)),
            Err(e) => Err(e),
        }
    }

    fn open_file(&self) -> StdResult<File> {
        File::open(self.path.as_path())
    }
}

impl super::ChunkReader for LocalFileChunker {
    fn read_chunk(&self, index: u64, buf: &mut [u8]) -> StdResult<usize> {
        self.seek_index(index)?;
        let chunked_buf = &mut buf[0..self.chunk_size as usize];
        self.open_file()?.read(chunked_buf)
    }
}

pub fn new_local_file_chunker(path: PathBuf, chunk_size: u64) -> Result<LocalFileChunker, String> {
    if chunk_size == 0 {
        return Err("chunk size must not be zero".to_string());
    }

    Ok(LocalFileChunker { path, chunk_size })
}
