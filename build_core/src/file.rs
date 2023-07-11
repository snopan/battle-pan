use std::fs::File;
use std::io::{self, BufWriter, BufReader};
use std::path::Path;

pub fn read_file(file_path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);
    Ok(buffer)
} 

pub fn write_file(file_path: &Path) -> io::Result<BufWriter<File>> {
    let buffer = BufWriter::new(File::create(file_path)?);
    Ok(buffer)
}