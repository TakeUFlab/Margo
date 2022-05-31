use chumsky::prelude::*;

use crate::types::{Inline, InlineBold, InlineCode, InlineMath, Text};

use super::{error::ParseError, txt};

pub fn parser() -> impl Parser<char, InlineMath, Error = ParseError> {
    just(" $")
        .ignore_then(txt::parser_until(just("$ ")))
        .map_with_span(|content, span| InlineMath { span, content })
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {}
}
