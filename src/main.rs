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

    let mut i = 0;
    loop {
        match splitter.next_reader() {
            Ok(mut r) => {
                let s = r.read().unwrap();
                let u = uploader::new(i);
                println!("uploaded: {:?}", u.upload(s.as_ref()).unwrap());
                i += 1;
            }
            Err(_) => {
                println!("done!");
                break;
            }
        }
    }
    /*
    let mut cr = new_chunk_reader(args.path, chunk_size).unwrap();

    let mut i = 0;
    loop {
        let mut buf = vec![0u8; chunk_size as usize];
        let s = cr.read_chunk(i, &mut buf).unwrap();
        if s == 0 {
            break;
        }
        let u = uploader::new(i);
        println!("uploaded: {:?}", u.upload(&buf).unwrap());

        i += 1;
    }
    */
}
