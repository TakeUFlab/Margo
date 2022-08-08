use crate::lst::*;
use nom::bytes::complete::tag;
use nom::error::{ErrorKind, ParseError};
use nom::{Err, Parser};
use thiserror::Error;

pub static NEWLINE: &str = "\n";
pub static NEWLINE_CHAR: char = '\n';

pub use nom::error::ErrorKind as EKind;
pub type NResult<'a> = Result<SynNode, Err<Error<&'a str>>>;
pub type IResult<'a> = nom::IResult<&'a str, Child, Error<&'a str>>;
pub trait IParser<'a, O = Child>: Parser<&'a str, O, Error<&'a str>> {}
impl<'a, T> IParser<'a> for T where T: Parser<&'a str, Child, Error<&'a str>> {}

#[derive(Debug, Clone, Error)]
pub enum Error<I> {
    #[error("nom parser error")]
    Nom(I, ErrorKind),

    #[error("errors")]
    Backtrack(Vec<Self>),
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Self::Nom(input, kind)
    }

    fn append(input: I, kind: ErrorKind, mut other: Self) -> Self {
        let err = Self::from_error_kind(input, kind);
        if let Self::Backtrack(ref mut v) = other {
            v.push(err);
            other
        } else {
            Self::Backtrack(vec![err, other])
        }
    }
}

impl<I> Error<I> {
    pub fn new(input: I, kind: ErrorKind) -> nom::Err<Error<I>> {
        nom::Err::Error(Self::from_error_kind(input, kind))
    }
}

pub fn c_token(kind: Syntax) -> impl Fn(&str) -> Child {
    move |text| Child::from(Token::new(kind.into(), text))
}

pub fn c_node(kind: Syntax) -> impl Fn(Vec<Child>) -> Child {
    move |child| Child::from(Node::new(kind.into(), child))
}

pub fn c_tag(kind: Syntax, t: &str) -> impl IParser {
    tag(t).map(c_token(kind))
}

pub fn c_newline(t: &str) -> IResult {
    tag(NEWLINE).map(c_token(Newline)).parse(t)
}

pub fn is_newline(s: char) -> bool {
    s == NEWLINE_CHAR
}
