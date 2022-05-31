use chumsky::prelude::*;

use crate::types::BlockCode;

use super::error::ParseError;

use super::txt;

pub fn parser() -> impl Parser<char, BlockCode, Error = ParseError> {
    let tag = just("```");
    tag.then(text::newline())
        .ignore_then(txt::parser_until(tag))
        .map_with_span(|content, span| BlockCode { span, content })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        dbg!(parser().parse_recovery_verbose("```\nHi\n```"));
    }
}
