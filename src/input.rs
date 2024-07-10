use std::{iter::Peekable, str::Chars};

pub struct Input<'a> {
    text: &'a str,
    line: i32,
    pos: usize,
    iter: Peekable<Chars<'a>>,
}

impl<'a> Input<'a> {
    pub fn new(text: &str) -> Input {
        Input {
            text,
            line: 1,
            pos: 0,
            iter: text.chars().peekable(),
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let chr = self.iter.next();
        if chr.is_none() {
            return None;
        }

        // for now, we don't support line spanning lexemes
        if chr.unwrap() == '\n' {
            self.line += 1;
            self.pos = 0;
        }
        else {
            self.pos += 1;
        }

        chr
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    pub fn get_lexeme(&self, start: usize, end: usize) -> String {
        self.text.chars().skip(start).take(end - start).collect()
    }
}
