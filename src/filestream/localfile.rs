use std::fs::File;
use std::io::{Seek, Read};
use std::io::Result as StdResult;
use std::path::{PathBuf};

pub struct LocalFileChunker {
    file : File,
    pub chunk_size : u64,
}

impl LocalFileChunker {
    fn index_to_offset(&self, index: u64) -> u64 {
        self.chunk_size * index
    }

    fn seek_index(&mut self, index: u64) -> StdResult<u64> {
        let offset = self.index_to_offset(index);
        self.file.seek(std::io::SeekFrom::Start(offset))
    }
}

impl super::ChunkReader for LocalFileChunker {
    fn read_chunk(&mut self, index: u64, buf: &mut [u8]) -> StdResult<usize> {
        self.seek_index(index)?;
        let chunked_buf = &mut buf[0..self.chunk_size as usize]; 
        self.file.read(chunked_buf)
    }
}

pub fn new_local_file_chunker(path : PathBuf, chunk_size : u64) -> Result<LocalFileChunker, String> {
    if chunk_size == 0 {
        return Err("chunk size must not be zero".to_string());
    }

    let f = File::open(path).map_err(|e| e.to_string())?;

    Ok(LocalFileChunker{
        file: f,
        chunk_size,
    })
}