use std::fs::File;
use std::io::{Result, Seek, Read};
use std::path::{Path, PathBuf};
use std::ptr::read;
use std::string;

pub trait ChunkReader {
    fn read_chunk(&self, index: u64, buf: &mut [u8]) -> Result<usize>;
}

pub struct FileChunker {
    pub path : PathBuf,
    pub chunk_size : u64,
}

impl ChunkReader for FileChunker {
    fn read_chunk(&self, index: u64, buf: &mut [u8]) -> Result<usize> {
        let offset = index * self.chunk_size;
        let my_buf = &mut buf[0..5];
        read_some(self.path.as_path(), offset, my_buf)
    }
}

fn open_at(path: &Path, offset: u64) -> Result<File> {
    let mut f = File::open(path)?;
    f.seek(std::io::SeekFrom::Start(offset));
    Ok(f)
}

pub fn read_some(path: &Path, offset: u64, buf: &mut [u8]) -> Result<usize> {
    let mut f = open_at(path, offset)?;
    f.read(buf)
}