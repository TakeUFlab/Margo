use chumsky::prelude::*;

use crate::token::Token;
use crate::types::{Block, BlockHeading, BlockParagraph, Text};

use super::error::ParseError;
use super::heading;
use super::utils::block_newline;

// pub fn parser() -> impl Parser<char, BlockParagraph, Error = ParseError> {
    
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_parse() {
        // parser().parse(stream)
    }
}
