use chumsky::prelude::*;

use crate::types::BlockCode;

use super::error::ParseError;

use super::{ident, txt};

pub fn parser() -> impl Parser<char, BlockCode, Error = ParseError> {
    let tag = just("```");
    tag.ignore_then(ident::parser().or_not())
        .then_ignore(text::newline())
        .then(txt::parser_until(tag))
        .map_with_span(|(lang, content), span| BlockCode {
            span,
            content,
            lang,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        dbg!(parser().parse_recovery_verbose("```\nHi\n```"));
        dbg!(parser().parse_recovery_verbose("```rust\nHi\n```"));
    }
}
