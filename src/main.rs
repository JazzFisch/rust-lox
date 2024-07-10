use std::env;
use std::fs;
use std::io::{self, Write};
use anyhow::Result;

#[derive(thiserror::Error, Debug)]
enum InterpreterError {
    #[error("Invalid command. Usage: {0} tokenize <filename>")]
    InvalidCommand(String),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("Failed to read file {0}")]
    InvalidFile(String),
}

enum InterpreterCommand {
    Tokenize(String),
}

fn main() -> Result<()> {
    let command = handle_args();
    if command.is_err() {
        let err = command.err().unwrap();
        writeln!(io::stderr(), "{}", err)?;
        return Err(err.into());
    }

    let result = match command.ok().unwrap() {
        InterpreterCommand::Tokenize(filename) => tokenize(&filename),
    };

    if result.is_err() {
        let err = result.err().unwrap();
        writeln!(io::stderr(), "{}", err)?;
        return Err(err.into());
    }

    Ok(())
}

fn handle_args() -> Result<InterpreterCommand, InterpreterError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(InterpreterError::InvalidCommand(args[0].clone()));
    }

    return match args[1].as_str() {
        "tokenize" => Ok(InterpreterCommand::Tokenize(args[2].clone())),
        _ => Err(InterpreterError::UnknownCommand(args[1].clone())),
    };
}

fn tokenize(filename: &String) -> Result<(), InterpreterError> {
    let file_contents = fs::read_to_string(filename);
    if file_contents.is_err() {
        return Err(InterpreterError::InvalidFile(filename.into()));
    }

    let file_contents = file_contents.ok().unwrap_or("".into());

    // ,, ., -, +, ;, *
    if !file_contents.is_empty() {
        for chr in file_contents.chars() {
            match chr {
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                '{' => println!("LEFT_BRACE {{ null"),
                '}' => println!("RIGHT_BRACE }} null"),
                ',' => println!("COMMA , null"),
                '.' => println!("DOT . null"),
                '-' => println!("MINUS - null"),
                '+' => println!("PLUS + null"),
                ';' => println!("SEMICOLON ; null"),
                '*' => println!("STAR * null"),
                _ => println!("CHAR {} null", chr),
            }
        }
    }

    println!("EOF  null");
    Ok(())
}