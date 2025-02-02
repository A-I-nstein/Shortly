use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::fs::{read_to_string, File};
use std::io::{stdin, Write};
use std::process::exit;
use url::{ParseError, Url};

use crate::db_ops::{insert_record, ShortlyRecord};
use crate::word_processing::{load_words, save_words};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    last_base: String,
}

pub fn shorten(custom: bool) {
    let long_url = get_user_url().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });

    let short_url_base = get_new_base(custom);

    let new_record = ShortlyRecord {
        short_url_base: short_url_base.clone(),
        long_url: long_url.to_string(),
    };

    insert_record(&new_record);
    update_config(&short_url_base, custom);

    println!(
        "Your unique short URL: http://127.0.0.1:3000/{}",
        short_url_base
    );
}

fn get_user_url() -> Result<Url, String> {
    let mut user_input = String::new();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("\nEnter the URL that you want to shorten: ");
    stdin()
        .read_line(&mut user_input)
        .map_err(|e| e.to_string())?;

    let trimmed_input = user_input.trim();
    if trimmed_input.is_empty() {
        return Err("No URL entered. Please try again.".to_string());
    }

    Url::parse(trimmed_input).map_err(|e: ParseError| format!("Invalid URL: {}", e))
}

fn get_new_base(custom: bool) -> String {
    if custom {
        get_custom_url()
    } else {
        let last_base = get_last_base();
        increment_string(&last_base)
    }
}

fn get_last_base() -> String {
    let json_data = read_to_string("config.json").unwrap_or_else(|err| {
        eprintln!("\nError reading config file: {}", err);
        exit(1);
    });
    let config: Config = from_str(&json_data).unwrap_or_else(|err| {
        eprintln!("\nError parsing config file: {}", err);
        exit(1);
    });
    config.last_base
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

fn update_config(short_url: &str, custom: bool) {
    if custom {
        return;
    }

    let config = Config {
        last_base: short_url.to_string(),
    };

    let json_data = to_string_pretty(&config).unwrap_or_else(|err| {
        eprintln!("\nError serializing config: {}", err);
        exit(1);
    });

    let mut file = File::create("config.json").unwrap_or_else(|err| {
        eprintln!("\nError creating config file: {}", err);
        exit(1);
    });

    file.write_all(json_data.as_bytes()).unwrap_or_else(|err| {
        eprintln!("\nError writing to config file: {}", err);
        exit(1);
    });
}

fn get_custom_url() -> String {
    loop {
        println!("\n1. Choose from a predefined list");
        println!("2. Provide your own (8 letters)");
        println!("\nHow would you like to customise your URL?");

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap_or_else(|err| {
            eprintln!("\nError reading input: {}", err);
            exit(1);
        });

        let trimmed_input = user_input.trim();
        if trimmed_input.is_empty() {
            eprint!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            eprintln!("\nNo choice has been made. Please try again.");
            continue;
        }

        match trimmed_input.parse::<u32>() {
            Ok(1) => return get_random_word(),
            Ok(2) => return get_custom_base(),
            _ => {
                eprint!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                eprintln!("\nInvalid choice. Please try again.")
            }
        }
    }
}

fn get_random_word() -> String {
    let mut rng = rand::rng();
    let mut eight_letter_words = load_words().unwrap();

    loop {
        let random_word = eight_letter_words.choose(&mut rng).unwrap().to_string();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("\nCustom URL Base: {:?}", random_word);
        println!("Press 'y' to select this URL base.");
        println!("Press 'Enter' to choose a different base.");

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap_or_else(|err| {
            eprintln!("\nError reading input: {}", err);
            exit(1);
        });

        let trimmed_input = user_input.trim();
        if !trimmed_input.is_empty() && trimmed_input.to_lowercase() == "y" {
            eight_letter_words.remove(
                eight_letter_words
                    .iter()
                    .position(|x| *x == random_word)
                    .expect("Element not found."),
            );
            let _ = save_words(eight_letter_words);
            return random_word;
        }
    }
}

fn get_custom_base() -> String {
    loop {
        println!("\nProvide your custom URL base (8-letters): ");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap_or_else(|err| {
            eprintln!("\nError reading input: {}", err);
            exit(1);
        });

        let trimmed_input = user_input.trim();
        if trimmed_input.is_empty() {
            eprint!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            eprintln!("\nNo input provided. Please try again.");
            continue;
        }

        if trimmed_input.len() == 8 {
            return trimmed_input.to_string();
        } else {
            eprint!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            eprintln!("Please provide an 8-letter word.");
        }
    }
}
