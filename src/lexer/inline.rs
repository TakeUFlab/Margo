use chumsky::prelude::*;

use crate::token::Token;
use crate::types::{Block, BlockHeading, BlockParagraph, Inline, InlineBold, Text};

use super::error::{ParseError, ParseLabel};
use super::utils::block_newline;
use super::{bold, heading};

pub fn parser() -> impl Parser<char, Inline, Error = ParseError> {
    recursive(|r| {
        // let bold = r
        //     .clone()
        //     .repeated()
        //     .delimited_by(just(" *"), just("* "))
        //     .map_with_span(|content, span| InlineBold {
        //         span,
        //         content: Box::new(Inline::Inlines(content)),
        //     })
        //     .map(Inline::Bold);
        // let txt = take_until(until)

        let txt = none_of("*/~_")
            .repeated()
            .at_least(1)
            .collect()
            .map_with_span(|content, span| Inline::Text(Text { span, content }));

        let bold = r
            .clone()
            .map_with_span(|content, span| {
                Inline::Bold(InlineBold {
                    span,
                    content: Box::new(content),
                })
            })
            .delimited_by(just(" *"), just("* "));

        bold.or(txt)
            .or(r.repeated().at_least(1).map(Inline::Inlines))
    })
    .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_parse() {
        // let txt = take_until(just::<_, _, ParseError>("* "))
        //     .map(|(chars, _)| chars)
        //     .collect()
        //     .map_with_span(|content, span| Text { span, content })
        //     .map(Inline::Text);

        // dbg!(txt.parse("sdfghj* ").unwrap());
        dbg!(parser().parse_recovery_verbose("AAA *fghjkl* ",));
    }
}
