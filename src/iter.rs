use crate::err::*;

use std::str::CharIndices;

#[derive(Clone)]
pub struct PIter<'a> {
    it: CharIndices<'a>,
    line: usize,
    col: usize,
}

impl<'a> Iterator for PIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.it.next() {
            Some((_, '\n')) => {
                self.line += 1;
                self.col = 0;
                Some('\n')
            }
            Some((_, v)) => {
                self.col += 1;
                Some(v)
            }
            None => None,
        }
    }
}

impl<'a> PIter<'a> {
    pub fn new(s: &'a str) -> Self {
        PIter {
            it: s.char_indices(),
            line: 0,
            col: 0,
        }
    }

    pub fn line_col(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    pub fn index(&self) -> Option<usize> {
        self.it.clone().next().map(|(i, _)| i)
    }

    pub fn err(&self, exp: Expected) -> PErr<'a> {
        let s = self.it.as_str();
        let c = s.char_indices().take(10).last();
        let found = match c {
            Some((n, _)) => &s[..n],
            None => s,
        };

        PErr {
            exp,
            found,
            col: self.col,
            line: self.line,
            index: self.index(),
            child: None,
            is_break: false,
        }
    }

    pub fn err_s(&self, s: &'static str) -> PErr<'a> {
        self.err(Expected::Str(s))
    }
}
