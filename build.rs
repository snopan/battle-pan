use build_core::{word_list, word_table, segment_table};
use std::{env};
use std::path::Path;

fn main() {

    let build_dir = env::var("OUT_DIR").unwrap();
    
    let word_list_path = env::var("WORD_LIST_PATH").unwrap();
    let word_table_filename = env::var("WORD_TABLE_FILENAME").unwrap();
    let word_table_path = Path::new(&build_dir).join(&word_table_filename);

    let word_list = word_list::build_word_list(&word_list_path).unwrap();
    let word_table_build = word_table::build_word_table(word_list);
    word_table::save_word_table(
        &word_table_path, 
        "WORDS",
        word_table_build
    ).unwrap();

    let segment_table_filename = env::var("SEGMENT_TABLE_FILENAME").unwrap();
    let segment_table_path = Path::new(&build_dir).join(&segment_table_filename);
    let segment_size = env::var("SEGMENT_SIZE").unwrap().parse::<usize>().unwrap();
    let difficulty_limits_str = env::var("DIFFICULTY_LIMITS").unwrap();

    let word_list = word_list::build_word_list(&word_list_path).unwrap();
    let segments = segment_table::make_segments(word_list, segment_size);
    let difficulty_limits = segment_table::parse_difficulty_limits(difficulty_limits_str);
    let difficulty_segments = segment_table::group_segments_by_difficulty(segments, difficulty_limits);
    let segment_table_build = segment_table::build_segment_table(difficulty_segments);
    segment_table::save_segment_table(
        &segment_table_path, 
        "SEGMENTS",
        segment_table_build
    ).unwrap();

    println!("cargo:rerun-if-changed={}", word_list_path.as_str());
}
