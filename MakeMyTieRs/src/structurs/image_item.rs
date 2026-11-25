use std::path::PathBuf;

#[derive(Default)]
pub struct ImageItem {
    pub id: i32,
    pub relative_path: PathBuf,
}
