use chumsky::prelude::*;

use crate::types::Block;

use super::error::ParseError;

use super::{code, heading, paragraph};

pub fn parser() -> impl Parser<char, Block, Error = ParseError> {
    choice((
        heading::parser().map(Block::Heading),
        code::parser().map(Block::Code),
        paragraph::parser().map(Block::Paragraph),
    ))
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {
        // parser().parse(stream)
    }
}
