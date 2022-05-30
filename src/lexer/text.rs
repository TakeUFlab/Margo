use chumsky::prelude::*;
use chumsky::primitive::custom;

use crate::token::Token;
use crate::types::{Block, BlockHeading, BlockParagraph, Inline, InlineBold, Text};

use super::error::ParseError;
use super::heading;
use super::utils::block_newline;

// pub fn register(r: Recursive<'_, char, Vec<Inline>, ParseError>) {
//     r.repeated().map(Inline::Text);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_parse() {
        dbg!(any::<char, ParseError>().parse("aaaa").unwrap());
    }
}
