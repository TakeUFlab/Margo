use chumsky::prelude::*;



use crate::types::{Text};

use super::error::ParseError;



pub fn parser() -> impl Parser<char, Text, Error = ParseError> {
    none_of("\n\r\n*/~_")
        .repeated()
        .at_least(1)
        .collect()
        .map_with_span(|content, span| Text { span, content })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_parse() {
        dbg!(any::<char, ParseError>().parse("aaaa").unwrap());
    }
}
