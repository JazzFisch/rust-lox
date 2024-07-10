pub struct Input<'a> {
    text: &'a str,
    pos: usize,
    line: i32
}

impl<'a> Input<'a> {
    pub fn new(text: &'a str) -> Input<'a> {
        Input {
            text,
            pos: 0,
            line: 1
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let chr = self.text.chars().nth(self.pos);
        if chr.is_none() {
            return None;
        }

        self.pos += 1;
        if chr.unwrap() == '\n' {
            self.line += 1;
        }

        chr
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn peek(&self) -> Option<char> {
        self.text.chars().nth(self.pos)
    }
}
