use crate::error::LoxError;
use crate::scanner::Scanner;
use std::fs;
use std::io::{self, BufRead};
use std::{env, process};

pub mod error;
mod scanner;
pub mod token;
pub mod token_types;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("failed to run the file");
    } else {
        run_prompt().expect("Failed to run the prompt");
    }

    let mut scanner_error = LoxError::new();
    scanner_error.set_error(true);
}

fn run_file(path: &String) -> io::Result<()> {
    let contents = fs::read_to_string(&path)?;
    for line in contents.lines() {
        println!("{}", line);
    }
    run(contents.to_string());
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        print!("> ");
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            run(line.to_string())
        } else {
            break;
        }
    }
    Ok(())
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
}

// struct Lox {
//     had_error: bool,
// }

// impl Lox {
//     fn report(&self) {}
// }
