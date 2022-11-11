use std::thread;

use chunksplitter::BufReaderIntoIterator;
use clap::Parser;

mod filestream;

mod config;
mod encdec;

use crate::config::get_config;

mod uploader;

mod chunksplitter;

use crate::encdec::EncDec;
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

    let key = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let c = encdec::new(key).unwrap();
    let mut buffer: Vec<u8> = vec![0; 16];
    buffer.extend_from_slice(b"super cool");
    c.enc(&mut buffer).unwrap();
    println!("my cipher: {:?}", buffer);
    match c.dec(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            println!("failed! {}", e)
        }
    }
    println!("my plain: {:?}", buffer);

    let config = get_config();
    let chunk_size = config.unwrap().get_int("default_chunk_size").unwrap() as u64;
    let splitter: _ = chunksplitter::new(args.path.as_path(), chunk_size).unwrap();
    run(&splitter);
}
