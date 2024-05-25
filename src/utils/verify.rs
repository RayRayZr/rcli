use std::path::{Path, PathBuf};

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists")
    }
}

pub fn verify_input(input: &str) -> Result<String, &'static str> {
    // - 代表标准输入
    if input == "-" {
        return Ok(input.into());
    }
    verify_input_file(input)
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    if Path::new(path).exists() {
        Ok(path.into())
    } else {
        Err("Path does not exists")
    }
}
