use chumsky::prelude::*;

use crate::token::Token;
use crate::types::{Block, BlockHeading, BlockParagraph, Text};

use super::error::ParseError;
use super::utils::block_newline;
use super::{heading, inline};

pub fn parser() -> impl Parser<char, BlockParagraph, Error = ParseError> {
    inline::parser().map_with_span(|content, span| BlockParagraph { span, content })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_parse() {
        // parser().parse(stream)
    }
}
