use chumsky::prelude::*;

use crate::{token::Ident, types::InlineMath};

use super::{error::ParseError, txt};

pub fn parser() -> impl Parser<char, Ident, Error = ParseError> {
    text::ident().map_with_span(|content, span| Ident { content, span })
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
