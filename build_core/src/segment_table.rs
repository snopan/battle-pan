use super::file;
use std::io::{self, Write};
use std::collections::HashMap;
use std::path::Path;


pub fn make_segments(word_list: Vec<String>, segment_size: usize) -> HashMap::<String, usize> {
    
    let mut segments = HashMap::<String, usize>::new();
    for word in word_list {

        // Only find segments if the word is big enough
        if word.len() >= segment_size {

            // Split the string into segments of the given size
            for start in 0..=(word.len() - segment_size) {
                let end = start + segment_size;
                let s = &word[start..end];
                let count = segments.get(s);

                // For each segment count increase by one or initialize to one if it doesn't exist
                let new_count = match count {
                    Some(c) => c + 1,
                    None => 1
                };
                segments.insert(s.to_string(), new_count);
            }
        }   
    }

    segments
}

pub fn parse_difficulty_limits(difficulty_limits_str: String) -> Vec<usize> {
    let difficulty_limits = difficulty_limits_str.split(",");
    difficulty_limits.filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

pub fn group_segments_by_difficulty(segments: HashMap<String, usize>, difficulty_limits: Vec<usize>) -> Vec<Vec<String>> {
    
    // Generate the vec that would store vec of segments based by difficulty
    // there would be n+1 vec generated where n is difficulty_limits length
    // because it is assumed that any segment that don't fit these limits
    // is the easiest difficulty, it is explained more below where it's being filled
    let mut difficulty_segments: Vec<Vec<String>> = vec![];
    for _ in 0..=difficulty_limits.len() {
        difficulty_segments.push(vec![]);
    }

    for (segment, count) in segments {

        // Iterate through difficulty limits to determine if the current segment belong
        // in this difficulty where if the count is within the limit it means it belongs there
        // we're including an extra index so if we loop to it means that this count was
        // larger than all limits so it will be placed at the end (easiest difficulty)
        for i in 0..=difficulty_limits.len() {

            // When the difficulty is found add it to difficulty_segments and break the loop
            if i == difficulty_limits.len() || difficulty_limits[i] >= count {
                difficulty_segments[i].push(segment);
                break;
            }
        }
    }

    difficulty_segments
}

pub fn build_segment_table(difficulty_segments: Vec<Vec<String>>) -> String {
    
    // Build each difficulty segment to String and combine them to be written later
    let segment_builds: Vec<String> = difficulty_segments.iter()
        .map(|segments| -> String {
            let mut builder = phf_codegen::OrderedSet::new();
            for s in segments {
                builder.entry(s);
            }
            builder.build().to_string()
        })
        .collect();
    
    segment_builds.join(", ")
}

pub fn save_segment_table(file_path: &Path, variable_name: &str, segment_table_build: String) -> Result<(), io::Error> {
    
    // Create the segment table file
    let mut segment_table_file = file::write_file(file_path)?;

    // Write the complete segment builds to the segment table file
    write!(
        &mut segment_table_file,
        "static {}: &'static [phf::OrderedSet<&'static str>] = &[{}];\n",
        variable_name,
        segment_table_build
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_segments() {
        let input = vec![String::from("hello"), String::from("tell")];
        assert_eq!(
            make_segments(input, 3), HashMap::from([
                (String::from("ell"), 2),
                (String::from("hel"), 1),
                (String::from("llo"), 1),
                (String::from("tel"), 1),
            ]),
            "create segments of length 3"
        );

        let input = vec![String::from("test"), String::from("tests")];
        assert_eq!(
            make_segments(input, 4),
            HashMap::from([
                (String::from("test"), 2),
                (String::from("ests"), 1),
            ]),
            "create segments of length 4"
        );
    }

    #[test]
    fn test_parse_difficulty_limits() {
        assert_eq!(
            parse_difficulty_limits(String::from("2,4,5")),
            vec![2, 4, 5],
            "Parse difficulty limits string"
        );
        assert_eq!(
            parse_difficulty_limits(String::from("2,4,,5,")),
            vec![2, 4, 5],
            "Parsing missing value should ignore it"
        );
    }

    #[test]
    fn test_group_segments_by_difficulty() {
        let segments = HashMap::from([
            (String::from("abc"), 1),
            (String::from("bcd"), 2),
            (String::from("cde"), 5),
            (String::from("def"), 5),
            (String::from("efg"), 10)
        ]);
        let difficulty_limits = vec![2, 7, 9];
        
        let mut output = group_segments_by_difficulty(segments, difficulty_limits);
        output[0].sort();
        output[1].sort();

        assert_eq!(
            output,
            vec![
                vec![String::from("abc"), String::from("bcd")],
                vec![String::from("cde"), String::from("def")],
                vec![],
                vec![String::from("efg")]
            ]
        );
    }

    #[test]
    fn test_build_segment_table() {
        let input = vec![
            vec![String::from("abc"), String::from("bcd")],
            vec![String::from("cde")],
        ];

        let mut builder_0 = phf_codegen::OrderedSet::new();
        builder_0.entry(input[0][0].to_owned());
        builder_0.entry(input[0][1].to_owned());

        let mut builder_1 = phf_codegen::OrderedSet::new();
        builder_1.entry(input[1][0].to_owned());
        
        let expected = format!(
            "{}, {}", 
            builder_0.build().to_string(),
            builder_1.build().to_string()
        );
        assert_eq!(build_segment_table(input), expected);
    }
}