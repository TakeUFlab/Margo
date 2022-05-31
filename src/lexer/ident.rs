use chumsky::prelude::*;

use crate::token::Ident;

use super::error::ParseError;

pub fn parser() -> impl Parser<char, Ident, Error = ParseError> {
    text::ident().map_with_span(|content, span| Ident { content, span })
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
