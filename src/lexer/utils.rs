use super::error::ParseError;
use chumsky::prelude::*;

pub fn block_newline() -> impl Parser<char, (), Error = ParseError> {
    text::newline().repeated().at_least(1).ignored()
}

pub fn is_newline(&c: &char) -> bool {
    "\n\r\n".contains(c)
}
