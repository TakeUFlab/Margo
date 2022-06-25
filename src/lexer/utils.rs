use super::error::{ParseError, ParseLabel};
use chumsky::prelude::*;

pub fn block_newline() -> impl Parser<char, (), Error = ParseError> {
    text::newline()
        .repeated()
        .at_least(2)
        .ignored()
        .labelled(ParseLabel::MissingBlockNewline)
}

pub fn is_newline(&c: &char) -> bool {
    "\n\r\n".contains(c)
}
