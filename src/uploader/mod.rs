pub trait ChunkUploader {
    fn upload(&self, buf: &[u8]) -> Result<serde_yaml::Value, String>;
}

mod local_uploader;

pub fn new(index: u64) -> impl ChunkUploader {
    return local_uploader::new(index);
}
