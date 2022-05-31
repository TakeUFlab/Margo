use chumsky::prelude::*;

use crate::types::File;

use super::block;
use super::error::ParseError;
use super::utils::{block_newline, is_newline};

pub fn parser() -> impl Parser<char, File, Error = ParseError> {
    block::parser()
        .separated_by(block_newline())
        .map(|content| File { content })
        .padded_by(text::newline().repeated())
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_file() {
        dbg!(parser().parse("## Hi\n\n").unwrap());
    }
}
