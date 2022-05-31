use chumsky::prelude::*;

use crate::types::Text;

use super::error::{ParseError, ParseLabel};

pub fn parser_not<T, K>(t: T) -> impl Parser<char, Text, Error = ParseError>
where
    T: Parser<char, K, Error = ParseError>,
{
    take_until(t.rewind())
        .try_map(|(chars, _), span| {
            (!chars.is_empty())
                .then(|| chars)
                .ok_or_else(|| ParseError::new(span).with_label(ParseLabel::CannotEmpty))
        })
        .collect()
        .map_with_span(|content, span| Text { span, content })
}

pub fn parser_until<T, K>(t: T) -> impl Parser<char, Text, Error = ParseError>
where
    T: Parser<char, K, Error = ParseError>,
{
    take_until(t)
        .map(|(chars, _)| chars)
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
