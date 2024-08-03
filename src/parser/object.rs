use std::fmt::Display;

use super::callable::Callable;

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Box<dyn Callable>),
    Nil,
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(val) => *val,
            Object::Nil => false,
            _ => true,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Nil, _) => false,
            (_, Self::Nil) => false,
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Boolean(left), Self::Boolean(right)) => left == right,
            // this might need to just be false
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

// TODO: is there a better way to display a callable
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(num) => {
                if f64::trunc(*num) == *num {
                    write!(f, "{:.1}", num)
                } else {
                    write!(f, "{}", num)
                }
            }
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Callable(_) => write!(f, "<callable>"),
            Object::Nil => write!(f, "nil"),
        }
    }
}
