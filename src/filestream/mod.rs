use std::fs::File;
use std::io::{Result, Seek, Read};
use std::path::{Path, PathBuf};
use std::ptr::read;
use std::string;

pub trait ChunkReader {
    fn read_chunk(&mut self, index: u64, buf: &mut [u8]) -> Result<usize>;
}

pub struct FileChunker {
    file : File,
    pub chunk_size : u64,
}

pub fn new_chunk_reader(path : PathBuf, chunk_size : u64) -> Result<FileChunker> {
    let mut f = File::open(path)?;
    if chunk_size == 0 {
        // return Err("chunk size must not be zero")
        // TODO how to return err
    }
    Ok(FileChunker{
        file: f,
        chunk_size: chunk_size,
    })
}

impl FileChunker {
    fn index_to_offset(&self, index: u64) -> u64 {
        self.chunk_size * index
    }

    fn seek_index(&mut self, index: u64) -> Result<u64> {
        let offset = self.index_to_offset(index);
        self.file.seek(std::io::SeekFrom::Start(offset))
    }
}

impl ChunkReader for FileChunker {
    fn read_chunk(&mut self, index: u64, buf: &mut [u8]) -> Result<usize> {
        self.seek_index(index)?;
        let chunked_buf = &mut buf[0..self.chunk_size as usize]; 
        self.file.read(chunked_buf)
    }
}