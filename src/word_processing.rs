use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::fs::{read_to_string, File};
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct CandidateWords {
    eight_letter_words: Vec<String>,
}

pub fn clean_words() -> Result<(), Box<dyn std::error::Error>> {
    let raw_file_contents = read_to_string("words_alpha.txt")?; // Use ? for error handling

    let eight_letter_words: Vec<String> = raw_file_contents
        .lines()
        .filter(|line| line.len() == 8)
        .map(String::from)
        .collect();

    let candidate_words = CandidateWords { eight_letter_words };

    let json_data = to_string_pretty(&candidate_words)?;
    let mut file = File::create("custom_urls.json")?;
    file.write_all(json_data.as_bytes())?;
    println!("\nCustom word list created.");

    Ok(())
}

pub fn load_words() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let json_data = read_to_string("custom_urls.json")?;
    let candidate_words: CandidateWords = from_str(&json_data)?;
    Ok(candidate_words.eight_letter_words)
}

pub fn save_words(eight_letter_words: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let candidate_words = CandidateWords { eight_letter_words };
    let json_data = to_string_pretty(&candidate_words)?;
    let mut file = File::create("custom_urls.json")?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}
