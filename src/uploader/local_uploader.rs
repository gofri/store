use serde::{Deserialize, Serialize};
use std::path::{self, PathBuf};
use std::str;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct LocalUploader {
    path: PathBuf,
}

impl super::ChunkUploader for LocalUploader {
    fn upload(&self, buf: &[u8]) -> Result<serde_yaml::Value, String> {
        println!("read bytes: {}", str::from_utf8(buf).unwrap());
        serde_yaml::to_value(self).map_err(|e| e.to_string())
    }
}

pub fn new(index: u64) -> impl super::ChunkUploader {
    return LocalUploader {
        path: path::Path::new(
            std::fmt::format(format_args!("/tmp/local_upload/{}", index)).as_str(),
        )
        .to_path_buf(),
    };
}
