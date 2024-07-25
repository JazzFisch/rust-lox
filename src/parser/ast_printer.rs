use crate::token::token_type::TokenType;

use super::Expression;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expression) -> String {
        match expr {
            Expression::Binary(left, operator, right) => {
                format!("({} {} {})", operator.token_type, self.print(left), self.print(right))
            },
            Expression::Grouping(expr) => {
                format!("(group {})", self.print(expr))
            },
            Expression::Literal(token) => {
                match token.token_type {
                    TokenType::Number => format!("{}", token.value),
                    TokenType::String => format!("{}", token.value),
                    TokenType::Keyword => format!("{}", token.value),
                    _ => "".to_string(),
                }
            },
            Expression::Unary(operator, right) => {
                format!("({} {})", operator.token_type, self.print(right))
            },
        }
    }
}