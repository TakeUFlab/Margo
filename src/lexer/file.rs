use chumsky::prelude::*;

use crate::token::Token;
use crate::types::{BlockHeading, File, Text};

use super::error::ParseError;
use super::utils::block_newline;
use super::{block, heading};

pub fn parser() -> impl Parser<char, File, Error = ParseError> {
    block::parser()
        .separated_by(block_newline())
        .map(|content| File { content })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_file() {
        dbg!(parser().parse("## Hi\n\n").unwrap());
    }
}
