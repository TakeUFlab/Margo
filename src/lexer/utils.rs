use super::error::ParseError;
use chumsky::prelude::*;

pub fn block_newline() -> impl Parser<char, (), Error = ParseError> {
    text::newline().repeated().exactly(2).ignored()
}
