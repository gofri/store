use std::thread;

use chunksplitter::BufReaderIntoIterator;
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

fn run<'a>(splitter: impl BufReaderIntoIterator<'a>) {
    thread::scope(|scope| {
        let mut children = vec![];
        for (s, i) in splitter.into_iter().zip(0u64..) {
            let handler = scope.spawn(move || -> Result<u64, String> {
                let u = uploader::new(i);
                let buf = s.read()?;
                let yml = u.upload(buf.as_ref())?;
                println!("uploaded: {:?}", yml);
                Ok(i)
            });
            children.push(handler);
        }
        for c in children {
            match c.join().expect("thread panic!") {
                Ok(i) => {
                    println!("thread finished: {}", i)
                }
                Err(e) => {
                    println!("thread failed: {}", e)
                }
            }
        }
    });
}

fn main() {
    env_logger::init();

    let args = Cli::parse();
    println!("got args: {}", args.path.display());

    let config = get_config();
    let chunk_size = config.unwrap().get_int("default_chunk_size").unwrap() as u64;
    let splitter = chunksplitter::new(args.path.as_path(), chunk_size).unwrap();
    run(&splitter);
}
