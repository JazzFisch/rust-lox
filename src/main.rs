use anyhow::Result;
use error_bag::ErrorBag;
use lexer::Lexer;
use parser::statement::Statement;
use parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use token::Token;
use visitor::expression_printer::ExpressionPrinter;

mod error_bag;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod visitor;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum InterpreterError {
    #[error("Invalid command. Usage: {0} <{1}> <filename>")]
    InvalidCommand(String, String),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("Lexical failure")]
    LexicalFailure,

    #[error("Parser failure: {0}")]
    ParserFailure(#[from] crate::parser::parse_error::ParseError),

    #[error("Interpreter failure: {0}")]
    InterpreterFailure(#[from] crate::interpreter::interpreter_error::InterpreterError),

    #[error("Failed to read file {0}")]
    InvalidFile(String),
}

enum InterpreterCommand {
    Tokenize(String),
    Parse(String),
    Interpret(String),
}

fn main() -> Result<()> {
    let command = handle_args();
    if command.is_err() {
        let error = command.err().unwrap();
        let message = format!("{}", error);
        writeln!(io::stderr(), "{}", message)?;
        return Err(anyhow::Error::msg(message));
    }

    let mut errors = error_bag::ErrorBag::default();
    let error = match command.ok().unwrap() {
        InterpreterCommand::Tokenize(filename) => tokenize_file(&filename, &mut errors, true).err(),
        InterpreterCommand::Parse(filename) => parse_file(&filename, &mut errors, true).err(),
        InterpreterCommand::Interpret(filename) => interpret_file(&filename, &mut errors).err(),
    };

    if error.is_some() {
        let error = error.unwrap();
        let exit_code = match error {
            InterpreterError::InvalidCommand(_, _) | InterpreterError::InvalidFile(_) => 64,
            InterpreterError::LexicalFailure | InterpreterError::ParserFailure(_) => 65,
            InterpreterError::InterpreterFailure(_) => 70,
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
    //let args: Vec<String> = vec!["".into(), "interpret".into(), "test.lox".into()];

    if args.len() < 3 {
        let path = Path::new(&args[0]);
        let file_name = path.to_string_lossy().into_owned();
        let empty = "command".to_owned();
        let command = args.get(1).unwrap_or(&empty);

        return Err(InterpreterError::InvalidCommand(file_name, command.clone()));
    }

    return match args[1].as_str() {
        "tokenize" => Ok(InterpreterCommand::Tokenize(args[2].clone())),
        "parse" => Ok(InterpreterCommand::Parse(args[2].clone())),
        "interpret" => Ok(InterpreterCommand::Interpret(args[2].clone())),
        _ => Err(InterpreterError::UnknownCommand(args[1].clone())),
    };
}

fn interpret_file(filename: &String, errors: &mut ErrorBag) -> Result<(), InterpreterError> {
    let statements = parse_file(filename, errors, false)?;
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(&statements)?;

    Ok(())
}

fn parse_file(
    filename: &String,
    errors: &mut ErrorBag,
    print_tree: bool,
) -> Result<Vec<Statement>, InterpreterError> {
    let tokens = tokenize_file(filename, errors, false)?;
    let mut parser = Parser::new(errors, tokens);
    let statements = parser.parse()?;

    if print_tree {
        let mut printer = ExpressionPrinter;
        for statement in &statements {
            if let Statement::Expression(expr) = statement {
                printer.print(expr);
            }
        }
    }

    Ok(statements)
}

fn tokenize_file(
    filename: &String,
    errors: &mut ErrorBag,
    print_tokens: bool,
) -> Result<Vec<Token>, InterpreterError> {
    let file_contents = fs::read_to_string(filename);
    if file_contents.is_err() {
        return Err(InterpreterError::InvalidFile(filename.into()));
    }

    let file_contents = file_contents.ok().unwrap_or("".into());
    let mut lexer = Lexer::new(errors, &file_contents);
    lexer.tokenize(print_tokens)
}
