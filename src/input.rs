use std::{iter::Peekable, str::Chars};

pub struct Input<'a> {
    line: i32,
    iter: Peekable<Chars<'a>>,
}

impl<'a> Input<'a> {
    pub fn new(text: &str) -> Input {
        Input {
            iter: text.chars().peekable(),
            line: 1
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let chr = self.iter.next();
        if chr.is_none() {
            return None;
        }

        if chr.unwrap() == '\n' {
            self.line += 1;
        }

        chr
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }
}
