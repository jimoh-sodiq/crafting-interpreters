use crate::scanner::Scanner;
use std::fs;
use std::io::{self, stdout, BufRead, Write};
use std::{env, process};

pub mod error;
pub mod scanner;
pub mod token;
pub mod token_types;
pub mod ast_print;
pub mod expr;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt().expect("Failed to run the prompt"),
        2 => run_file(&args[1]).expect("failed to run the file"),
        _ => {
            println!("Usage: lox [script]");
            process::exit(64);
        }
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let contents = fs::read_to_string(&path)?;
    for line in contents.lines() {
        println!("{}", line);
    }
    run(contents.to_string());
    // write!()
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
    }
    Ok(())
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let _tokens = scanner.scan_tokens();
}
