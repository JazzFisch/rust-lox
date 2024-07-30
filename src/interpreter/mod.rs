pub mod environment;
pub mod interpreter_error;

use environment::Environment;
use interpreter_error::InterpreterError;

use crate::{
    parser::{
        binary_expression::BinaryExpression, expression::Expression,
        expression_value::ExpressionValue, grouping_expression::GroupingExpression,
        literal_expression::LiteralExpression, statement::Statement,
        unary_expression::UnaryExpression, variable_expression::VariableExpression,
    },
    token::{token_type::TokenType, token_value::TokenValue, Token},
    visitor::{expression_visitor::ExpressionVisitor, statement_visitor::StatementVisitor},
};

pub struct Interpreter {
    environment: Environment,
}

fn is_truthy(value: &ExpressionValue) -> bool {
    match value {
        ExpressionValue::Nil => false,
        ExpressionValue::Boolean(val) => *val,
        _ => true,
    }
}

fn is_equal(left: &ExpressionValue, right: &ExpressionValue) -> bool {
    match (left, right) {
        (ExpressionValue::Nil, ExpressionValue::Nil) => true,
        (ExpressionValue::Nil, _) => false,
        (_, ExpressionValue::Nil) => false,
        (ExpressionValue::Number(left), ExpressionValue::Number(right)) => left == right,
        (ExpressionValue::String(left), ExpressionValue::String(right)) => left == right,
        (ExpressionValue::Boolean(left), ExpressionValue::Boolean(right)) => left == right,
        _ => false,
    }
}

fn check_number_operand(
    operator: &TokenType,
    operand: &ExpressionValue,
) -> Result<(), InterpreterError> {
    if let ExpressionValue::Number(_) = operand {
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(format!(
            "Operand must be a number for operator ({} {})",
            operator, operand
        )))
    }
}

fn check_number_operands<'a>(
    left: &'a ExpressionValue,
    operator: &TokenType,
    right: &'a ExpressionValue,
) -> Result<(f64, f64), InterpreterError> {
    if let (ExpressionValue::Number(left), ExpressionValue::Number(right)) = (left, right) {
        Ok((*left, *right))
    } else {
        Err(InterpreterError::RuntimeError(format!(
            "Operands must be numbers for operator ({} {} {})",
            left, operator, right
        )))
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Statement>) -> Result<(), InterpreterError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn evaluate(&self, expr: &Expression) -> Result<ExpressionValue, InterpreterError> {
        let expr = expr.accept(self)?;
        Ok(expr)
    }

    fn execute(&mut self, stmt: &Statement) -> Result<(), InterpreterError> {
        stmt.accept(self)
    }
}

impl ExpressionVisitor<ExpressionValue, InterpreterError> for Interpreter {
    fn visit_binary(&self, expr: &BinaryExpression) -> Result<ExpressionValue, InterpreterError> {
        let left = self.evaluate(expr.left())?;
        let operator = expr.operator();
        let right = self.evaluate(expr.right())?;

        // special case for string concatenation
        if let (ExpressionValue::String(left), TokenType::Plus, ExpressionValue::Number(right)) =
            (&left, operator.token_type, &right)
        {
            return Ok(ExpressionValue::String(format!("{}{}", left, right)));
        }

        match operator.token_type {
            TokenType::Minus => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(ExpressionValue::Number(left - right))
            }
            TokenType::Slash => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(ExpressionValue::Number(left / right))
            }
            TokenType::Star => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(ExpressionValue::Number(left * right))
            }
            TokenType::Plus => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(ExpressionValue::Number(left + right))
            }
            TokenType::Greater => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(ExpressionValue::Boolean(left > right))
            }
            TokenType::GreaterEqual => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(ExpressionValue::Boolean(left >= right))
            }
            TokenType::Less => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(ExpressionValue::Boolean(left < right))
            }
            TokenType::LessEqual => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(ExpressionValue::Boolean(left <= right))
            }
            TokenType::BangEqual => Ok(ExpressionValue::Boolean(!is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(ExpressionValue::Boolean(is_equal(&left, &right))),
            _ => unreachable!(
                "Invalid binary expression ({} {} {})",
                left,
                expr.operator().token_type,
                right
            ),
        }
    }

    fn visit_grouping(
        &self,
        expr: &GroupingExpression,
    ) -> Result<ExpressionValue, InterpreterError> {
        let value = self.evaluate(expr.expression())?;
        Ok(value)
    }

    fn visit_literal(&self, expr: &LiteralExpression) -> Result<ExpressionValue, InterpreterError> {
        Ok(expr.value().clone())
    }

    fn visit_unary(&self, expr: &UnaryExpression) -> Result<ExpressionValue, InterpreterError> {
        let operator = expr.operator();
        let right = self.evaluate(expr.right())?;

        match (operator.token_type, &right) {
            (TokenType::Minus, ExpressionValue::Number(num)) => {
                check_number_operand(&operator.token_type, &right)?;
                Ok(ExpressionValue::Number(-num))
            }
            (TokenType::Bang, _) => Ok(ExpressionValue::Boolean(!is_truthy(&right))),
            _ => Err(InterpreterError::RuntimeError(format!(
                "Invalid unary expression ({} {})",
                operator.token_type, right
            ))),
            // _ => unreachable!(
            //     "Invalid unary expression ({} {})",
            //     expr.operator().token_type,
            //     right
            // ),
        }
    }

    fn visit_variable(
        &self,
        var: &VariableExpression,
    ) -> Result<ExpressionValue, InterpreterError> {
        if let TokenValue::Identifier(name) = &var.name().value {
            let value = self.environment.get(name)?;
            return Ok(value.clone());
        }
        unreachable!("Variable expression must have a string name");
    }
}

impl StatementVisitor for Interpreter {
    fn visit_expression_statement(&self, expr: &Expression) -> Result<(), InterpreterError> {
        let _ = self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print_statement(&self, expr: &Expression) -> Result<(), InterpreterError> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_variable_statement(
        &mut self,
        name: &Token,
        initializer: &Option<Expression>,
    ) -> Result<(), InterpreterError> {
        let mut value = ExpressionValue::Nil;
        if initializer.is_some() {
            value = self.evaluate(initializer.as_ref().unwrap())?;
        }

        if let TokenValue::Identifier(name) = &name.value {
            let name = name.clone();
            self.environment.define(name, value);
        }

        Ok(())
    }
}
