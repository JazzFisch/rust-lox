use crate::{
    parser::parse_error::ParseError,
    token::{token_type::TokenType, token_value::TokenValue, Token},
};

pub struct ErrorBag {
    failed: bool,
    writer: Box<dyn std::io::Write>,
}

impl Default for ErrorBag {
    fn default() -> Self {
        Self {
            failed: false,
            writer: Box::new(std::io::stderr()),
        }
    }
}

impl ErrorBag {
    pub fn new(writer: Box<dyn std::io::Write>) -> Self {
        Self {
            failed: false,
            writer,
        }
    }

    pub fn has_error(&self) -> bool {
        self.failed
    }

    pub fn parse_error(&mut self, token: &Token, message: &str) -> ParseError {
        let report_result = match (&token.token_type, &token.value) {
            (TokenType::Eof, _) => self.report(token.line, " at end", message),
            (_, TokenValue::None) => self.report(
                token.line,
                format!(" at '{}'", token.token_type).as_str(),
                message,
            ),
            _ => self.report(
                token.line,
                format!(" at '{}'", token.value).as_str(),
                message,
            ),
        };

        report_result.expect("Failed to write to writer");

        self.failed = true;
        ParseError::Error(token.token_type, message.to_string())
    }

    pub fn report(
        &mut self,
        line: usize,
        location: &str,
        message: &str,
    ) -> Result<(), std::io::Error> {
        writeln!(
            self.writer.as_mut(),
            "[line {line}] Error{location}: {message}",
            line = line,
            location = location,
            message = message
        )
    }

    pub fn report_lex_error(&self, line: usize, message: &str) {
        eprintln!("[line {}] Error: {}", line, message);
    }
}
