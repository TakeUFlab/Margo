use chumsky::prelude::*;

use crate::token::Token;
use crate::types::{Block, BlockHeading, Text};

use super::error::ParseError;
use super::utils::block_newline;
use super::{heading, paragraph};

pub fn parser() -> impl Parser<char, Block, Error = ParseError> {
    choice((
        heading::parser().map(Block::Heading),
        paragraph::parser().map(Block::Paragraph),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_parse() {
        // parser().parse(stream)
    }
}
