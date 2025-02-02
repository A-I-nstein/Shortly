use std::fs::File;
use std::io::Write;
use std::fs::read_to_string;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize)]
struct CandidateWords {
    eight_letter_words: Vec<String>,
    four_letter_words: Vec<String>
}

pub fn clean_words() {

    let mut file_contents_vector: Vec<String> = vec![];
    let raw_file_contents: String = read_to_string("words_alpha.txt").unwrap();
    let mut file_contents_iter = raw_file_contents.lines();
    let mut line_str = file_contents_iter.next();
    while line_str != None {
        file_contents_vector.push(line_str.unwrap().to_string());
        line_str = file_contents_iter.next();
    }

    let mut four_letter_words: Vec<String> = vec![];
    let mut eight_letter_words: Vec<String> = vec![];

    for line in file_contents_vector {
        if line.len() == 4 {
            four_letter_words.push(line);
        } else if line.len() == 8 {
            eight_letter_words.push(line);
        }
    }

    // println!("{:?} {:?}", four_letter_words, four_letter_words.len());
    // println!("{:?} {:?}", eight_letter_words, eight_letter_words.len());

    let candidate_words: CandidateWords = CandidateWords {
        four_letter_words,
        eight_letter_words
    };

    let json_data: String = to_string_pretty(&candidate_words).unwrap();
    let mut file: File = File::create("custom_urls.json").unwrap();
    file.write_all(json_data.as_bytes()).unwrap();

}

pub fn load_words () -> Vec<String> {
    let json_data: String = read_to_string("custom_urls.json").unwrap();
    let candidate_words: CandidateWords = from_str(&json_data).unwrap();
    return candidate_words.eight_letter_words;
}