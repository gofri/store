use clap::Parser;

mod filestream;

mod config;
use crate::config::get_config;

mod uploader;

mod chunksplitter;
use crate::chunksplitter::new_chunk_splitter;

use crate::uploader::ChunkUploader;

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
    let splitter = new_chunk_splitter(args.path.as_path(), chunk_size).unwrap();

    println!("start reading {} bytes", splitter.total_size());
    for (mut s, i) in splitter.make_iter().zip(0u64..) {
        let s = s.read().unwrap();
        let u = uploader::new(i);
        println!("uploaded: {:?}", u.upload(s.as_ref()).unwrap());
    }
}
