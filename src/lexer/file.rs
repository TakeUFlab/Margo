use super::block;
use super::error::ParseError;
use super::utils::block_newline;
use crate::traits::Hashing;
use crate::types::{Block, File};
use chumsky::prelude::*;
use std::hash::Hash;

impl File {
    #[cfg(not(feature = "hashing"))]
    pub fn new(content: Block) -> Self {
        Self { content }
    }

    #[cfg(feature = "hashing")]
    pub fn new(content: Block) -> Self {
        let hash = ("file", &content).hashing();
        Self { content, hash }
    }
}

#[cfg(feature = "hashing")]
impl Hash for File {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub fn parser() -> impl Parser<char, File, Error = ParseError> {
    block::parser()
        .separated_by(block_newline())
        .map(Block::Blocks)
        .map(File::new)
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
