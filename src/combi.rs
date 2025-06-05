use crate::iter::*;
use crate::parser::*;

impl<A, B> Parser for (A, B)
where
    A: Parser,
    B: Parser,
{
    type Out = (A::Out, B::Out);

    fn parse<'a>(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it, a, _) = self.0.parse(i)?;
        let (it, b, nx) = self.1.parse(&it)?;
        Ok((it, (a, b), nx))
    }
}

impl<A, B, C> Parser for (A, B, C)
where
    A: Parser,
    B: Parser,
    C: Parser,
{
    type Out = (A::Out, B::Out, C::Out);

    fn parse<'a>(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it, a, _) = self.0.parse(i)?;
        let (it, b, _) = self.1.parse(&it)?; // TODO Allow expectation carry on fail
        let (it, c, nx) = self.2.parse(&it)?;
        Ok((it, (a, b, c), nx))
    }
}

pub fn or<A, B, O>(a: A, b: B) -> Or<A, B>
where
    A: Parser<Out = O>,
    B: Parser<Out = O>,
{
    Or { a, b }
}

pub struct Or<A, B> {
    a: A,
    b: B,
}

impl<A, B, O> Parser for Or<A, B>
where
    A: Parser<Out = O>,
    B: Parser<Out = O>,
{
    type Out = O;

    fn parse<'a>(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        match self.a.parse(i) {
            Ok(v) => Ok(v),
            Err(e) => {
                if e.is_break {
                    return Err(e);
                }
                match self.b.parse(i) {
                    Ok(v) => Ok(v),
                    Err(e2) => Err(e.join(e2)),
                }
            }
        }
    }
}

pub fn map<A: Parser, F: Fn(A::Out) -> V, V>(a: A, f: F) -> Map<A, F, V> {
    Map { a, f }
}

pub struct Map<A: Parser, F: Fn(A::Out) -> V, V> {
    a: A,
    f: F,
}

impl<A: Parser, F: Fn(A::Out) -> V, V> Parser for Map<A, F, V> {
    type Out = V;

    fn parse<'a>(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        self.a.parse(i).map(|(i, v, e)| (i, (self.f)(v), e))
    }
}
