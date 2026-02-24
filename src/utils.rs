use std::{fs, path::PathBuf};

pub fn file_exists(path: &PathBuf) -> bool {
    fs::metadata(path).map_or(false, |s| s.is_file())
}
