use crate::error::LoxError;
use std::fs;
use std::io::{self, BufRead};
use std::{env, process};

mod token_types;
mod error;

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
    run(contents.as_str());
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
            run(&line.as_str())
        } else {
            break;
        }
    }
    Ok(())
}

struct Scanner<T> {
    source: T,
}
impl<T: std::fmt::Display> Scanner<T> {
    fn new(source: T) -> Self {
        Scanner { source }
    }

    fn scan_tokens(&self) -> &T {
        println!("scanning tokens");
        println!("source: {}", &self.source);
        &self.source
    }
}

fn run<T: std::fmt::Display>(source: T) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
}

// struct Lox {
//     had_error: bool,
// }

// impl Lox {
//     fn report(&self) {}
// }
