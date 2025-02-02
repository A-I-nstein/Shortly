use std::{
    io::{stdin, Stdin},
    num::ParseIntError,
    process::exit,
};

mod db_ops;
mod shortly_ops;
mod shortly_server;
mod word_processing;

use db_ops::{clear_db, create_db, show_records};
use shortly_ops::shorten;
use shortly_server::start_server;
use word_processing::clean_words;

fn main() {
    let choice: Result<i64, String> = get_user_input();
    match choice {
        Ok(in_choice) => {
            if in_choice == 1 {
                shorten(false);
            } else if in_choice == 2 {
                shorten(true);
            } else if in_choice == 3 {
                show_records();
            } else if in_choice == 4 {
                create_db();
            } else if in_choice == 5 {
                clear_db();
            } else if in_choice == 6 {
                clean_words();
            } else if in_choice == 7 {
                start_server();
            } else {
                eprintln!("Invalid choice. Please try again.");
                exit(1)
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}

fn get_user_input() -> Result<i64, String> {
    let mut user_input: String = String::new();

    println!("\n1. Shorten URL: press '1'");
    println!("2. Create custom URL: press '2'");
    println!("3. Show all records: press '3'");
    println!("4. Create Database: press '4'");
    println!("5. Clear Database: press '5'");
    println!("6. Create custom URL list: press '6'");
    println!("7. Start Server: press '7'");
    println!("\nWelcome to Shortly. What would you like to do? ");

    let stdin: Stdin = stdin();
    let _n: usize = stdin
        .read_line(&mut user_input)
        .map_err(|e| e.to_string())?;

    let trimmed_input: &str = user_input.trim();
    if trimmed_input.is_empty() {
        return Err("No choice has been made. Please try again.".to_string());
    }
    let choice: Result<i64, String> = trimmed_input
        .parse::<i64>()
        .map_err(|e: ParseIntError| format!("Invalid Input: {}", e));
    return choice;
}
