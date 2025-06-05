use std::cmp::Ordering;

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Expected {
    #[error("{}", .0)]
    Char(char),

    #[error("Char in {}", .0)]
    CharIn(&'static str),

    #[error("{}", .0)]
    Str(&'static str),

    #[error("{:?}", .0)]
    OneOf(Vec<Expected>),
}

#[derive(Debug, Error, PartialEq)]
#[error("Expected {}, Found {}, at ({:?}, {}, {}), \n\t {:?}", .exp, .found, .index, .line, .col, .child)]
pub struct PErr<'a> {
    pub exp: Expected,
    pub found: &'a str,
    pub index: Option<usize>,
    pub line: usize,
    pub col: usize,
    pub is_break: bool,
    pub child: Option<Box<PErr<'a>>>,
}

// We need to be able to join these errors in order of string position, preferring the furthest
// along in the string.
pub fn compare_index(a: &Option<usize>, b: &Option<usize>) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => a.cmp(b),
        // None happens at end of iterator
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

pub fn join_children<'a>(
    a: Option<Box<PErr<'a>>>,
    b: Option<Box<PErr<'a>>>,
) -> Option<Box<PErr<'a>>> {
    match (a, b) {
        (Some(ac), Some(bc)) => Some(Box::new((*ac).join(*bc))),
        (a, None) => a,
        (None, b) => b,
    }
}

impl<'a> PErr<'a> {
    pub fn join(mut self, mut b: Self) -> Self {
        match compare_index(&self.index, &b.index) {
            Ordering::Greater => {
                self.child = join_children(self.child, Some(Box::new(b)));
                self
            }
            Ordering::Less => {
                b.child = join_children(b.child, Some(Box::new(self)));
                b
            }
            _ => {
                self.child = join_children(self.child, b.child);
                self.exp = match (self.exp, b.exp) {
                    (Expected::OneOf(mut ae), Expected::OneOf(be)) => {
                        ae.extend(be);
                        Expected::OneOf(ae)
                    }
                    (Expected::OneOf(mut ae), b) | (b, Expected::OneOf(mut ae)) => {
                        ae.push(b);
                        Expected::OneOf(ae)
                    }
                    (a, b) => Expected::OneOf(vec![a, b]),
                };
                self
            }
        }
    }

    pub fn brk(mut self, b: bool) -> Self {
        self.is_break = b;
        self
    }
}
