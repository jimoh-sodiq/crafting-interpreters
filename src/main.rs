use crate::scanner::Scanner;
use std::fs;
use std::io::{self, stdout, BufRead, Write};
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
    print!("> ");
    stdout().flush()?;
    for line in stdin.lock().lines() {
        match line {
            Ok(val) => {
                if val.is_empty() {
                    break;
                } else {
                    run(val)
                }
            }
            Err(e) => eprintln!("Error: {}", e.to_string()),
        }
        // if let Ok(line) = line {
        //     if line.is_empty() {
        //         break;
        //     }
        //     run(line.to_string())
        // } else {
        //     break;
        // }
    }
    Ok(())
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let _tokens = scanner.scan_tokens();
}

// struct Lox {
//     had_error: bool,
// }

// impl Lox {
//     fn report(&self) {}
// }
