use super::file;
use std::io::{self, Write};
use std::path::Path;

pub fn build_word_table(word_list: Vec<String>) -> String {
    
    // Create a phf table builder and add each word parsed to the builder
    let mut builder = phf_codegen::Set::new();
    for word in word_list {
        builder.entry(word);
    }

    builder.build().to_string()
}

pub fn save_word_table(file_path: &Path, variable_name: &str, word_table_build: String) -> Result<(), io::Error> {
    let mut word_table_file = file::write_file(file_path)?;
    write!(
        &mut word_table_file,
        "static {}: phf::Set<&'static str> = {};\n",
        variable_name,
        word_table_build
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_word_table() {
        let input = vec![String::from("hello"), String::from("world")];
        let mut builder = phf_codegen::Set::new();
        builder.entry(input[0].to_owned());
        builder.entry(input[1].to_owned());
        assert_eq!(build_word_table(input), builder.build().to_string());
    }
}