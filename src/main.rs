use std::thread;

use chunksplitter::ChunkSplitter;
use clap::Parser;

mod filestream;

mod config;
use crate::config::get_config;

mod uploader;

mod chunksplitter;

use crate::uploader::ChunkUploader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn run<'a>(splitter: &'a ChunkSplitter<'a>) {
    thread::scope(|scope| {
        for (s, i) in splitter.into_iter().zip(0u64..) {
            scope.spawn(move || {
                let b = s.read().unwrap();
                let u = uploader::new(i);
                println!("uploaded: {:?}", u.upload(b.as_ref()).unwrap());
            });
        }
    });
}

fn main() {
    env_logger::init();

    let args = Cli::parse();
    println!("got args: {}", args.path.display());

    let config = get_config();
    let chunk_size = config.unwrap().get_int("chunk_size").unwrap() as u64;
    let splitter = chunksplitter::new(args.path.as_path(), chunk_size).unwrap();
    run(&splitter);
}
