use std::{io::stdin, process::exit, str::FromStr};

mod db_ops;
mod shortly_ops;
mod shortly_server;
mod word_processing;

use db_ops::{clear_db, create_db, show_records};
use shortly_ops::shorten;
use shortly_server::start_server;
use word_processing::clean_words;

#[derive(Debug, PartialEq)]
enum Choice {
    Shorten,
    CustomShorten,
    ShowRecords,
    CreateDb,
    ClearDb,
    CustomWordList,
    StartServer,
    Invalid,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Choice::Shorten),
            "2" => Ok(Choice::CustomShorten),
            "3" => Ok(Choice::ShowRecords),
            "4" => Ok(Choice::CreateDb),
            "5" => Ok(Choice::ClearDb),
            "6" => Ok(Choice::CustomWordList),
            "7" => Ok(Choice::StartServer),
            _ => Ok(Choice::Invalid),
        }
    }
}

fn main() {
    loop {
        match get_user_input() {
            Ok(choice) => match choice {
                Choice::Shorten => shorten(false),
                Choice::CustomShorten => shorten(true),
                Choice::ShowRecords => show_records(),
                Choice::CreateDb => create_db(),
                Choice::ClearDb => clear_db(),
                Choice::CustomWordList => clean_words().unwrap(),
                Choice::StartServer => start_server().unwrap(),
                Choice::Invalid => {
                    eprint!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    eprintln!("\nInvalid choice. Please try again.");
                    continue;
                }
            },
            Err(err) => {
                eprintln!("Error: {}", err);
                exit(1);
            }
        }
        break;
    }
}

fn get_user_input() -> Result<Choice, String> {
    let mut user_input = String::new();

    println!("\n1. Create short URL");
    println!("2. Create custom short URL");
    println!("\nWelcome to Shortly. What would you like to do?");

    let stdin = stdin();
    stdin
        .read_line(&mut user_input)
        .map_err(|e| e.to_string())?;

    let trimmed_input = user_input.trim();
    if trimmed_input.is_empty() {
        return Err("No choice has been made. Please try again.".to_string());
    }

    trimmed_input
        .parse::<Choice>()
        .map_err(|_| "Invalid input. Please enter a number.".to_string())
}
