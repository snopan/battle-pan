use super::file;
use std::io::{self, BufRead};

pub fn build_word_list(file_path: &str) -> Result<Vec<String>, io::Error> {
        
    // Load the raw word list file as a read buffer
    let word_list_buffer = file::read_file(file_path)?;

    // Collect and convert to Result<Vec>
    let word_list_r: Result<Vec<String>, io::Error> = word_list_buffer.lines().collect();

    // Retrieve the Vec<String> if no error is found
    let word_list = word_list_r?;

    Ok(word_list)
}