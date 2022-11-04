/*
pub trait StatefulTransaction {
    fn prepare();
    fn commit();
}

pub trait FileIndexStorage {
    fn store();
    fn remove();
}

pub trait FileIndexTransaction {
    fn do(); // e.g. prepare index storage, actually store, commit
    fn undo(); // e.g. prepare index to removal, actually remove, remove storage file
}
*/

pub enum IndexAction {
    UPLOAD = "upload",
}

type YamlResult = Result<serde_yaml::Value, serde_yaml::Error>;

pub trait ActionIndex {
    fn prepare(&self) -> YamlResult;
    fn complete(&self, success: bool) -> YamlResult;
}

struct UploadAction {
    path: path::Path,
    src: path::Path,
}

const REMOTE_FILE_NAME: &str = "data";
#[derive(Serialize, Deserialize)]
struct RemoteFileInfo {
    repo: String,
    branch: String,
}

#[derive(Serialize, Deserialize)]
struct UploadActionData {
    chunks: [RemoteFileInfo],
}

impl UploadAction {
    // TODO should come from a provider (like the entire upload procedure and info)
    fn translate(src: &path::Path) -> Result<UploadActionData, String> {
        // TODO return repo/branch
    }
}

impl ActionIndex for UploadAction {
    fn prepare(&self) -> YamlResult {
        serde_yaml::to_value().map_err(|e| e.to_string())
    }
}
