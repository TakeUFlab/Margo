use chumsky::prelude::*;


use crate::types::{BlockParagraph};

use super::error::ParseError;

use super::{inline};

pub fn parser() -> impl Parser<char, BlockParagraph, Error = ParseError> {
    inline::parser().map_with_span(|content, span| BlockParagraph { span, content })
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn block_parse() {
        // parser().parse(stream)
    }
}
