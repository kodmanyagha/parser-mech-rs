use crate::*;
use err::*;
use iter::*;
use parser::*;

pub trait CharBool: Sized {
    fn char_bool(&self, c: char) -> bool;
    fn expected(&self) -> Expected {
        Expected::Str(std::any::type_name::<Self>())
    }

    fn plus(self) -> Plus<Self> {
        Plus { cb: self }
    }

    fn star(self) -> Star<Self> {
        Star { cb: self }
    }
}

impl<F: Fn(char) -> bool> CharBool for F {
    fn char_bool(&self, c: char) -> bool {
        // self being a function of course (Lesson 10 2:02)
        self(c)
    }
}

impl CharBool for char {
    fn char_bool(&self, c: char) -> bool {
        *self == c
    }

    fn expected(&self) -> Expected {
        Expected::Char(*self)
    }
}

impl CharBool for &'static str {
    fn char_bool(&self, c: char) -> bool {
        self.contains(c)
    }

    fn expected(&self) -> Expected {
        Expected::CharIn(self)
    }
}

pub fn do_char_read<'a, CB: CharBool>(
    cb: &CB,
    i: &PIter<'a>,
    min: usize,
    exact: bool,
) -> ParseRes<'a, String> {
    let mut res = String::new();
    let mut it = i.clone();

    loop {
        let i2 = it.clone();
        match it.next() {
            Some(c) if cb.char_bool(c) => {
                res.push(c);
                if res.len() == min && exact {
                    return Ok((it, res, None));
                }
            }
            _ => {
                if res.len() >= min {
                    let e_op = Some(it.err(cb.expected()));
                    return Ok((i2, res, e_op));
                }
                return Err(i2.err(cb.expected()));
            }
        }
    }
}

pub struct Plus<CB: CharBool> {
    cb: CB,
}

impl<CB: CharBool> Parser for Plus<CB> {
    type Out = String;

    fn parse<'a>(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_char_read(&self.cb, i, 1, false)
    }
}

pub struct Star<CB: CharBool> {
    cb: CB,
}

impl<CB: CharBool> Parser for Star<CB> {
    type Out = String;

    fn parse<'a>(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_char_read(&self.cb, i, 0, false)
    }
}
