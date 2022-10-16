use std::str;

use clap::Parser;

mod filestream;
use crate::filestream::{ChunkReader, new_chunk_reader};

mod config;
use crate::config::get_config;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    env_logger::init();

    let args = Cli::parse();
    println!("got args: {}", args.path.display());

    let config = get_config();
    let chunk_size = config.unwrap().get_int("chunk_size").unwrap() as u64;
    let mut cr = new_chunk_reader(args.path, chunk_size).unwrap();
    for i in 1..10 {
        let mut buf = [0u8; 1000];
        let s = cr.read_chunk(i, &mut buf).unwrap();
        println!("read {} bytes: {}", s, str::from_utf8(&buf).unwrap());
    }
}