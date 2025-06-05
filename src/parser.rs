use crate::{
    combi::{Map, Or, map, or},
    *,
};
use err::*;
use iter::*;

pub type ParseRes<'a, V> = Result<(PIter<'a>, V, Option<PErr<'a>>), PErr<'a>>;

pub trait Parser: Sized {
    type Out;

    fn parse<'a>(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out>;

    fn parse_s<'a>(&self, s: &'a str) -> Result<Self::Out, PErr<'a>> {
        self.parse(&PIter::new(s)).map(|(_, v, _)| v)
    }

    fn or<B: Parser<Out = Self::Out>>(self, b: B) -> Or<Self, B> {
        or(self, b)
    }

    fn map<F: Fn(Self::Out) -> V, V>(self, f: F) -> Map<Self, F, V> {
        map(self, f)
    }
}

impl<V, F> Parser for F
where
    F: for<'a> Fn(&PIter<'a>) -> ParseRes<'a, V>,
{
    type Out = V;
    fn parse<'b>(&self, i: &PIter<'b>) -> ParseRes<'b, V> {
        self(i)
    }
}

impl Parser for &'static str {
    type Out = &'static str;

    fn parse<'a>(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let mut it = i.clone();
        let s_it = self.chars();

        for n in s_it {
            match it.next() {
                Some(v) if v == n => {}
                _ => return Err(i.err_s(self)),
            }
        }

        Ok((it, self, None))
    }
}

impl Parser for char {
    type Out = char;

    fn parse<'a>(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let mut it = i.clone();

        match it.next() {
            Some(c) if c == *self => Ok((it, c, None)),
            _ => Err(i.err(Expected::Char(*self))),
        }
    }
}
