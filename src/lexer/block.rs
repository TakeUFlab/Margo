use chumsky::prelude::*;

use crate::types::Block;

use super::error::{ParseError, ParseLabel};

use super::{code, heading, paragraph};

pub fn parser() -> impl Parser<char, Block, Error = ParseError> {
    choice((
        heading::parser().map(Block::Heading),
        code::parser().map(Block::Code),
        paragraph::parser().map(Block::Paragraph),
    ))
    .labelled(ParseLabel::NotABlock)
}

#[cfg(test)]
mod tests {

    #[test]
    fn block_parse() {
        // parser().parse(stream)
    }
}
