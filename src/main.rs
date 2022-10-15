#![allow(unused)]

use std::path::Path;
use std::str;

use clap::Parser;

use crate::filestream::{FileChunker, ChunkReader, new_chunk_reader};
mod filestream;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("got args: {}", args.path.display());
    let chunk_size = 5u64;
    let mut cr = new_chunk_reader(args.path, chunk_size).unwrap();
    for i in 1..10 {
        let mut buf = [0u8; 1000];
        let s = cr.read_chunk(i, &mut buf).unwrap();
        println!("read {} bytes: {}", s, str::from_utf8(&buf).unwrap());
    }
}