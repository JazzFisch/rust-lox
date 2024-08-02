use crate::{
    token::{token_type::TokenType, token_value::TokenValue, Token},
    visitor::expression_visitor::ExpressionVisitor,
};

use super::object::Object;

pub enum Expression {
    Assignment {
        name: Token,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Literal {
        value: Object,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Variable {
        name: Token,
    },
}

impl Expression {
    pub fn new_assignment(name: Token, expression: Expression) -> Self {
        Expression::Assignment {
            name,
            expression: Box::new(expression),
        }
    }

    pub fn new_binary(left: Expression, operand: Token, right: Expression) -> Self {
        Expression::Binary {
            left: Box::new(left),
            operator: operand,
            right: Box::new(right),
        }
    }

    pub fn new_grouping(expression: Expression) -> Self {
        Expression::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn new_literal(token: &Token) -> Self {
        let value = match (&token.token_type, &token.value) {
            (TokenType::Nil, _) => Object::Nil,
            (TokenType::False, _) => Object::Boolean(false),
            (TokenType::True, _) => Object::Boolean(true),
            (_, TokenValue::Number(num)) => Object::Number(*num),
            (_, TokenValue::String(str)) => Object::String(str.clone()),
            _ => unreachable!("Invalid token value for literal expression {:?}", token),
        };

        Expression::Literal { value }
    }

    pub fn new_logical(left: Expression, operator: Token, right: Expression) -> Self {
        Expression::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_unary(operator: Token, right: Expression) -> Self {
        Expression::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn new_variable(name: Token) -> Self {
        Expression::Variable { name }
    }

    pub fn accept<T, E>(&self, visitor: &mut dyn ExpressionVisitor<T, E>) -> Result<T, E> {
        match self {
            Expression::Assignment { name, expression } => {
                visitor.visit_assignment(name, expression)
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expression::Grouping { expression } => visitor.visit_grouping(expression),
            Expression::Literal { value } => visitor.visit_literal(value),
            Expression::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical(left, operator, right),
            Expression::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expression::Variable { name } => visitor.visit_variable(name),
        }
    }
}
