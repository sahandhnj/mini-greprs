use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn get_file_content(file_path: &str) -> Result<String, io::Error>{
    let mut file = File::open(file_path)?;

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    Ok(file_content)
}