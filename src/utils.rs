use std::{fs, path::PathBuf};

pub fn file_exists(path: &PathBuf) -> bool {
    fs::metadata(path).is_ok_and(|s| s.is_file())
}
