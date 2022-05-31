use chumsky::prelude::*;

use crate::types::InlineCode;

use super::{error::ParseError, txt};

pub fn parser() -> impl Parser<char, InlineCode, Error = ParseError> {
    just(" `")
        .ignore_then(txt::parser_until(just("` ")))
        .map_with_span(|content, span| InlineCode { span, content })
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
