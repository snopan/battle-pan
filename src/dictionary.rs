extern crate phf;
use rand::{thread_rng, Rng};

include!(concat!(env!("OUT_DIR"), "/", env!("WORD_TABLE_FILENAME")));
include!(concat!(env!("OUT_DIR"), "/", env!("SEGMENT_TABLE_FILENAME")));

pub fn is_word(word: &str) -> bool {
    WORDS.contains(word)
}

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBoundsDifficulty(f32),
    OutOfBoundsDifficultyIndex(usize)
}

pub fn translate_difficulty(difficulty: f32) -> Result<usize, Error> {
    if difficulty < 0.0 || difficulty > 1.0 {
        return Err(Error::OutOfBoundsDifficulty(difficulty))
    }

    if difficulty == 1.0 {
        return Ok(SEGMENTS.len() - 1)
    }

    let translated = (difficulty * SEGMENTS.len() as f32).floor();
    Ok(translated as usize)
} 

pub fn get_random_segment(difficulty_index: usize) -> Result<String, Error> {
    if difficulty_index >= SEGMENTS.len() {
        return Err(Error::OutOfBoundsDifficultyIndex(difficulty_index))
    }

    let segments: &phf::OrderedSet<&'static str> = &SEGMENTS[difficulty_index];

    let random_index = thread_rng().gen_range(0..segments.len());
    let segment = segments.index(random_index).unwrap();

    Ok(String::from(*segment))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_word() {
        assert!(is_word("hello"), "returns true for real word");
        assert!(!is_word("abcdefg"), "returns false for fake word");
    }

    #[test]
    fn test_translate_difficulty() {
        assert_eq!(
            translate_difficulty(1.0),
            Ok(SEGMENTS.len()-1),
            "returns last difficulty index when given difficulty 1.0"
        );
        assert_eq!(
            translate_difficulty(0.0),
            Ok(0),
            "returns first difficulty index when given difficulty 0.0"
        );
        assert_eq!(
            translate_difficulty(-0.1),
            Err(Error::OutOfBoundsDifficulty(-0.1)),
            "returns error when given a negative difficulty"
        );
        assert_eq!(
            translate_difficulty(1.1),
            Err(Error::OutOfBoundsDifficulty(1.1)),
            "returns error when given a number larger than 1"
        );
    }

    #[test]
    fn test_get_random_segment() {
        assert!(SEGMENTS.len() > 0, "There should be at least one difficulty group of segments");

        let output = get_random_segment(0);
        let segment_size = env!("SEGMENT_SIZE").parse::<usize>();
        assert!(output.is_ok(), "retrieving segment with valid index should be ok");
        assert!(segment_size.is_ok(), "retrieving env SEGMENT_SIZE should be ok");
        assert_eq!(
            output.unwrap().len(),
            segment_size.unwrap(),
            "the length of the segment should equal to config"
        );

        assert_eq!(
            get_random_segment(SEGMENTS.len()),
            Err(Error::OutOfBoundsDifficultyIndex(SEGMENTS.len())),
            "returns error when given a difficulty index larger than SEGMENTS"
        )
    }
}