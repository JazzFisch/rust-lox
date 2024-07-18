use std::env;
use std::fs;
use std::io::{self, Write};
use anyhow::Result;

mod lexer;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum InterpreterError {
    #[error("Invalid command. Usage: {0} tokenize <filename>")]
    InvalidCommand(String),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("Lexical failure")]
    LexicalFailure,

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
        InterpreterCommand::Tokenize(filename) => tokenize_file(&filename),
    };

    if result.is_err() {
        let err = result.err().unwrap();
        let exit_code = match err {
            InterpreterError::InvalidCommand(_) => 64,
            InterpreterError::InvalidFile(_) => 64,
            InterpreterError::LexicalFailure => 65,
            _ => 1,
        };

        if err != InterpreterError::LexicalFailure {
            writeln!(io::stderr(), "{}", err)?;
        }
        std::process::exit(exit_code);
    }

    Ok(())
}

fn handle_args() -> Result<InterpreterCommand, InterpreterError> {
    let args: Vec<String> = env::args().collect();

    //let args: Vec<String> = vec!["".into(), "tokenize".into(), "test.lox".into()];

    if args.len() < 3 {
        return Err(InterpreterError::InvalidCommand(args[0].clone()));
    }

    return match args[1].as_str() {
        "tokenize" => Ok(InterpreterCommand::Tokenize(args[2].clone())),
        _ => Err(InterpreterError::UnknownCommand(args[1].clone())),
    };
}

fn tokenize_file(filename: &String) -> Result<(), InterpreterError> {
    let file_contents = fs::read_to_string(filename);
    if file_contents.is_err() {
        return Err(InterpreterError::InvalidFile(filename.into()));
    }

    let file_contents = file_contents.ok().unwrap_or("".into());
    let mut lexer = lexer::Lexer::new(&file_contents);
    lexer.tokenize()
}
