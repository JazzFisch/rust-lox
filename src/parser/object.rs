use std::{cmp::Ordering, fmt::Display};

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

    pub fn is_callable(&self) -> bool {
        matches!(self, Object::Callable(_))
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

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Nil, Self::Nil) => Some(Ordering::Equal),
            (Self::Number(left), Self::Number(right)) => {
                if left < right {
                    Some(Ordering::Less)
                } else if left > right {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Self::String(left), Self::String(right)) => match left.cmp(right) {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Greater => Some(Ordering::Greater),
                Ordering::Equal => Some(Ordering::Equal),
            },
            (Self::Boolean(left), Self::Boolean(right)) => match left.cmp(right) {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Greater => Some(Ordering::Greater),
                Ordering::Equal => Some(Ordering::Equal),
            },
            // this might need to just be false
            _ => None,
        }
    }
}

// TODO: is there a better way to display a callable
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(num) => {
                if num.fract() == 0.0 {
                    write!(f, "{}", *num as i64)
                } else {
                    write!(f, "{}", num)
                }
            }
            Object::String(str) => write!(f, "{}", str),
            Object::Boolean(bool) => write!(f, "{}", bool),
            Object::Callable(callable) => write!(f, "{}", callable),
            Object::Nil => write!(f, "nil"),
        }
    }
}
