use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use anyhow::Result;
use token::Token;

mod lexer;
mod parser;
mod token;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum InterpreterError {
    #[error("Invalid command. Usage: {0} <{1}> <filename>")]
    InvalidCommand(String, String),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("Lexical failure")]
    LexicalFailure,

    #[error("Parser failure")]
    ParserFailure,

    #[error("Failed to read file {0}")]
    InvalidFile(String),
}

enum InterpreterCommand {
    Tokenize(String),
    Parse(String),
}

fn main() -> Result<()> {
    let command = handle_args();
    if command.is_err() {
        let error = command.err().unwrap();
        writeln!(io::stderr(), "{}", error)?;
        return Err(error.into());
    }

    let error = match command.ok().unwrap() {
        InterpreterCommand::Tokenize(filename) => tokenize_file(&filename, true).err(),
        InterpreterCommand::Parse(filename) => parse_file(&filename).err(),
    };

    if error.is_some() {
        let error = error.unwrap();
        let exit_code = match error {
            InterpreterError::InvalidCommand(_, _) | InterpreterError::InvalidFile(_) => 64,
            InterpreterError::LexicalFailure | InterpreterError::ParserFailure => 65,
            _ => 1,
        };

        if error != InterpreterError::LexicalFailure {
            writeln!(io::stderr(), "{}", error)?;
        }
        std::process::exit(exit_code);
    }

    Ok(())
}

fn handle_args() -> Result<InterpreterCommand, InterpreterError> {
    let args: Vec<String> = env::args().collect();

    //let args: Vec<String> = vec!["".into(), "tokenize".into(), "test.lox".into()];
    //let args: Vec<String> = vec!["".into(), "parse".into(), "test.lox".into()];

    if args.len() < 3 {
        let path = Path::new(&args[0]);
        // what the rust, rust?!?!
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let empty = "command".to_string();
        let command = args.get(1).unwrap_or(&empty);

        return Err(InterpreterError::InvalidCommand(file_name, command.clone()));
    }

    return match args[1].as_str() {
        "tokenize" => Ok(InterpreterCommand::Tokenize(args[2].clone())),
        "parse" => Ok(InterpreterCommand::Parse(args[2].clone())),
        _ => Err(InterpreterError::UnknownCommand(args[1].clone())),
    };
}

fn parse_file(filename: &String) -> Result<(), InterpreterError> {
    let tokens = tokenize_file(filename, false)?;
    let mut parser = parser::Parser::new(tokens);
    let expression = parser.parse();
    let printer = parser::ast_printer::AstPrinter;

    match expression {
        Ok(expression) => println!("{}", printer.print(&expression)),
        Err(_) => {
            return Err(InterpreterError::ParserFailure);
        }
    }

    if parser.failed() {
        return Err(InterpreterError::ParserFailure);
    }

    Ok(())
}

fn tokenize_file(filename: &String, print_tokens: bool) -> Result<Vec<Token>, InterpreterError> {
    let file_contents = fs::read_to_string(filename);
    if file_contents.is_err() {
        return Err(InterpreterError::InvalidFile(filename.into()));
    }

    let file_contents = file_contents.ok().unwrap_or("".into());
    let mut lexer = lexer::Lexer::new(&file_contents);
    lexer.tokenize(print_tokens)
}
