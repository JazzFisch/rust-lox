pub mod environment;
pub mod interpreter_error;

mod helpers;

use environment::Environment;
use helpers::{check_number_operand, check_number_operands};
use interpreter_error::InterpreterError;

use crate::{
    parser::{callable::Clock, expression::Expression, object::Object, statement::Statement},
    token::{token_type::TokenType, token_value::TokenValue, Token},
    visitor::{expression_visitor::ExpressionVisitor, statement_visitor::StatementVisitor},
};

pub struct Interpreter {
    environment: Environment,
    globals: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        globals.define("clock", Object::Callable(Box::new(Clock {})));
        let environment = Environment::new_from_parent(&mut globals.clone());

        Self {
            environment,
            globals,
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Statement>) -> Result<(), InterpreterError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expression) -> Result<Object, InterpreterError> {
        let expr = expr.accept(self)?;
        Ok(expr)
    }

    fn execute(&mut self, stmt: &Statement) -> Result<(), InterpreterError> {
        stmt.accept(self)
    }

    fn execute_block(&mut self, statements: &[Statement]) -> Result<(), InterpreterError> {
        self.environment.push_scope();

        let mut result: Result<(), InterpreterError> = Ok(());
        for statement in statements {
            if let Err(err) = self.execute(statement) {
                result = Err(err);
                break;
            }
        }

        self.environment.pop_scope();
        result
    }
}

impl ExpressionVisitor<Object, InterpreterError> for Interpreter {
    fn visit_assignment(
        &mut self,
        name: &Token,
        expression: &Expression,
    ) -> Result<Object, InterpreterError> {
        let value = self.evaluate(expression)?;
        self.environment.assign(name, value.clone())?;
        Ok(value)
    }

    fn visit_binary(
        &mut self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Result<Object, InterpreterError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        // special case for string concatenation
        if let (Object::String(left), TokenType::Plus, Object::Number(right)) =
            (&left, operator.token_type, &right)
        {
            return Ok(Object::String(format!("{}{}", left, right)));
        }

        match operator.token_type {
            TokenType::Minus => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(Object::Number(left - right))
            }
            TokenType::Slash => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(Object::Number(left / right))
            }
            TokenType::Star => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(Object::Number(left * right))
            }
            TokenType::Plus => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(Object::Number(left + right))
            }
            TokenType::Greater => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(Object::Boolean(left > right))
            }
            TokenType::GreaterEqual => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(Object::Boolean(left >= right))
            }
            TokenType::Less => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(Object::Boolean(left < right))
            }
            TokenType::LessEqual => {
                let (left, right) = check_number_operands(&left, &operator.token_type, &right)?;
                Ok(Object::Boolean(left <= right))
            }
            TokenType::BangEqual => Ok(Object::Boolean(left != right)),
            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
            _ => unreachable!(
                "Invalid binary expression ({} {} {})",
                left, operator.token_type, right
            ),
        }
    }

    fn visit_call(
        &mut self,
        callee: &Expression,
        _: &Token,
        arguments: &[Expression],
    ) -> Result<Object, InterpreterError> {
        let callee = self.evaluate(callee)?;

        let mut args = Vec::new();
        for argument in arguments {
            let arg = self.evaluate(argument)?;
            args.push(arg);
        }

        if let Object::Callable(callable) = callee {
            if args.len() != callable.arity() {
                return Err(InterpreterError::RuntimeError(format!(
                    "Expected {} arguments but got {}.",
                    callable.arity(),
                    args.len()
                )));
            }
            return callable.call(self, args);
        }

        Err(InterpreterError::RuntimeError(
            "Can only call functions and classes.".to_string(),
        ))
    }

    fn visit_grouping(&mut self, expression: &Expression) -> Result<Object, InterpreterError> {
        let value = self.evaluate(expression)?;
        Ok(value)
    }

    fn visit_literal(&mut self, value: &Object) -> Result<Object, InterpreterError> {
        Ok(value.clone())
    }

    fn visit_logical(
        &mut self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Result<Object, InterpreterError> {
        let left = self.evaluate(left)?;

        if operator.token_type == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else if !left.is_truthy() {
            return Ok(left);
        }

        self.evaluate(right)
    }

    fn visit_unary(
        &mut self,
        operator: &Token,
        right: &Expression,
    ) -> Result<Object, InterpreterError> {
        let right = self.evaluate(right)?;

        match (operator.token_type, &right) {
            (TokenType::Minus, Object::Number(num)) => {
                check_number_operand(&operator.token_type, &right)?;
                Ok(Object::Number(-num))
            }
            (TokenType::Bang, _) => Ok(Object::Boolean(!right.is_truthy())),
            _ => Err(InterpreterError::RuntimeError(format!(
                "Invalid unary expression ({} {})",
                operator.token_type, right
            ))),
        }
    }

    fn visit_variable(&mut self, name: &Token) -> Result<Object, InterpreterError> {
        if let TokenValue::Identifier(name) = &name.value {
            let value = self.environment.get(name)?;
            return Ok(value.clone());
        }
        unreachable!("Variable expression must have a string name");
    }
}

impl StatementVisitor for Interpreter {
    fn visit_block_statement(&mut self, statements: &[Statement]) -> Result<(), InterpreterError> {
        self.execute_block(statements)
    }

    fn visit_expression_statement(&mut self, expr: &Expression) -> Result<(), InterpreterError> {
        let _ = self.evaluate(expr)?;
        Ok(())
    }

    fn visit_if_statement(
        &mut self,
        condition: &Expression,
        then_branch: &Statement,
        else_branch: &Option<Box<Statement>>,
    ) -> Result<(), InterpreterError> {
        let condition = self.evaluate(condition)?;
        if condition.is_truthy() {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_print_statement(&mut self, expr: &Expression) -> Result<(), InterpreterError> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_variable_statement(
        &mut self,
        name: &Token,
        initializer: &Option<Expression>,
    ) -> Result<(), InterpreterError> {
        let mut value = Object::Nil;
        if initializer.is_some() {
            value = self.evaluate(initializer.as_ref().unwrap())?;
        }

        if let TokenValue::Identifier(name) = &name.value {
            let name = name.clone();
            self.environment.define(name.as_str(), value);
        }

        Ok(())
    }

    fn visit_while_statement(
        &mut self,
        condition: &Expression,
        body: &Statement,
    ) -> Result<(), InterpreterError> {
        let mut value = self.evaluate(condition)?;
        loop {
            if !value.is_truthy() {
                break;
            }

            self.execute(body)?;
            value = self.evaluate(condition)?;
        }

        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
