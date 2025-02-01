use std::fs::File;
use std::io::Write;
use serde_json::from_str;
use url::{Url, ParseError};
use std::io::{Stdin, stdin};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, process::exit};

mod db_ops;
use db_ops::{create_db, insert_record, show_records, clear_db, ShortlyRecord};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    last_base: String
}

fn main() {
    let long_url: Result<Url, String> = get_user_url();
    // create_db();
    match long_url {
        Ok(in_url) => {
            let new_record: ShortlyRecord = ShortlyRecord {
                short_url_base: get_new_base(),
                long_url: in_url.to_string()
            };
            insert_record(&new_record);
            update_config(&new_record.short_url_base);
            show_records();
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
    // clear_db();
}

fn get_user_url() -> Result<Url, String> {
    let mut user_input: String = String::new();

    println!("Enter the URL that you want to shorten: ");
    let stdin: Stdin = stdin();
    let _n: usize = stdin.read_line(&mut user_input).map_err(|e| e.to_string())?;

    let trimmed_input: &str = user_input.trim();
    if trimmed_input.is_empty() {
        return Err("No URL entered. Please try again.".to_string());
    }
    let long_url: Result<Url, String> = Url::parse(trimmed_input).map_err(|e: ParseError| format!("Invalid URL: {}", e));
    return long_url;
}

fn get_new_base() -> String {
    let last_base: String = get_last_base();
    increment_string(&last_base)
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

fn update_config(short_url: &String) {
    let config: Config = Config {
        last_base: short_url.to_string(),
    };
    let json_data: String = serde_json::to_string_pretty(&config).unwrap();
    let mut file: File = File::create("config.json").unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}