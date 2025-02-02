use std::fs::File;
use std::io::Write;
use rand::seq::IndexedRandom;
use url::{Url, ParseError};
use std::io::{Stdin, stdin};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, process::exit};
use serde_json::{from_str, to_string_pretty};

use crate::word_processing::load_words;
use crate::db_ops::{insert_record, ShortlyRecord};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    last_base: String
}

pub fn shorten(custom:bool) {
    let long_url: Result<Url, String> = get_user_url();
    match long_url {
        Ok(in_url) => {
            let new_record: ShortlyRecord = ShortlyRecord {
                short_url_base: get_new_base(custom),
                long_url: in_url.to_string()
            };
            insert_record(&new_record);
            update_config(&new_record.short_url_base, custom);
            println!("{}", format!("Your unique short URL: http://127.0.0.1:3000/{}", new_record.short_url_base));
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}

fn get_user_url() -> Result<Url, String> {
    let mut user_input: String = String::new();

    println!("\nEnter the URL that you want to shorten: ");
    let stdin: Stdin = stdin();
    let _n: usize = stdin.read_line(&mut user_input).map_err(|e| e.to_string())?;

    let trimmed_input: &str = user_input.trim();
    if trimmed_input.is_empty() {
        return Err("No URL entered. Please try again.".to_string());
    }
    let long_url: Result<Url, String> = Url::parse(trimmed_input).map_err(|e: ParseError| format!("Invalid URL: {}", e));
    return long_url;
}

fn get_new_base(custom:bool) -> String {
    if custom {
        return get_custom_url();
    } else {
        let last_base: String = get_last_base();
        return increment_string(&last_base);
    }
}

fn get_last_base() -> String {
    let json_data: String = read_to_string("config.json").unwrap();
    let config: Config = from_str(&json_data).unwrap();
    return config.last_base;
}

fn increment_string(s: &str) -> String {
    let mut result: String = s.to_string();
    let mut chars: Vec<char> = result.chars().collect::<Vec<char>>();

    for i in (0..chars.len()).rev() {
        if chars[i] < 'z' {
            chars[i] = (chars[i] as u8 + 1) as char;
            result = chars.iter().collect();
            return result;
        } else {
            chars[i] = 'a';
        }
    }
    "a".to_string() + &result
}

fn update_config(short_url: &String, custom:bool) {
    if custom {
        return;
    }
    let config: Config = Config {
        last_base: short_url.to_string(),
    };
    let json_data: String = to_string_pretty(&config).unwrap();
    let mut file: File = File::create("config.json").unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}

fn get_custom_url() -> String {

    
    println!("\n1. Choose from a predefined list: press '1'");
    println!("2. Provide your own (8 letters): press '2'");
    println!("\nHow would you like to customise your URL?");

    let mut user_input: String = String::new();

    let stdin: Stdin = stdin();
    let _n: usize = stdin.read_line(&mut user_input).unwrap();

    let trimmed_input: &str = user_input.trim();
    if trimmed_input.is_empty() {
        eprintln!("No choice has been made. Please try again.");
        exit(1)
    }
    let choice: i64 = trimmed_input.parse::<i64>().unwrap();

    if choice == 1 {
        return get_random_word();
    } else if choice == 2 {
        return get_custom_base();
    } else {
        eprintln!("Invalid choice. Please try again.");
        exit(1)
    }
}

fn get_random_word() -> String {

    let mut rng = rand::rng();
    let eight_letter_words: Vec<String> = load_words();

    loop {
        let random_word: String = eight_letter_words.choose(&mut rng).unwrap().to_string();
        println!("\nCustom URL Base: {:?}", random_word);
        println!("Press 'y' to select this URL base.");
        println!("Click 'Enter' to choose a different base.");

        let mut user_input: String = String::new();
        let stdin: Stdin = stdin();
        let _n: usize = stdin.read_line(&mut user_input).unwrap();
        let trimmed_input: &str = user_input.trim();
        if trimmed_input.is_empty() == false {
            return random_word;
        }
    }
}

fn get_custom_base () -> String {
    loop {
        println!("\nProvide your custom URL base: ");
        let mut user_input: String = String::new();
        let stdin: Stdin = stdin();
        let _n: usize = stdin.read_line(&mut user_input).unwrap();
        let trimmed_input: &str = user_input.trim();
        if trimmed_input.is_empty() {
            eprintln!("No input provided. Please try again.");
            exit(1)
        }
        if trimmed_input.len() == 8 {
            return trimmed_input.to_string();
        } else {
            println!("Please provide a 8 letter word.");
        }
    }
}