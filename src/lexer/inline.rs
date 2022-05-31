use chumsky::prelude::*;

use crate::token::Token;
use crate::types::{
    Block, BlockHeading, BlockParagraph, Inline, InlineBold, InlineItalic, InlineLinethrough,
    InlineUnderline, Text,
};

use super::error::{ParseError, ParseLabel};
use super::utils::{block_newline, is_newline};
use super::{bold, heading, italic, linethrough, txt, underline};

pub fn parser() -> impl Parser<char, Inline, Error = ParseError> {
    recursive(|r| {
        let txt = txt::parser().map(Inline::Text);

        let bold = bold::parser(r.clone()).map(Inline::Bold);

        let italic = italic::parser(r.clone()).map(Inline::Italic);

        let linethrough = linethrough::parser(r.clone()).map(Inline::Linethrough);

        let underline = underline::parser(r.clone()).map(Inline::Underline);

        choice((bold, italic, linethrough, underline, txt))
            .repeated()
            .at_least(1)
            .map(Inline::Inlines)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_parse() {
        // let txt = take_until(justs::<_, _, ParseError>("* "))
        //     .map(|(chars, _)| chars)
        //     .collect()
        //     .map_with_span(|content, span| Text { span, content })
        //     .map(Inline::Text);

        // dbg!(txt.parse("sdfghj* ").unwrap());
        dbg!(parser().parse_recovery_verbose("AAA * fghjkl *  _ there ~ fghjk ~ _ fghjk",));
    }
}
