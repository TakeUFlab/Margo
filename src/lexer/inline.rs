use chumsky::prelude::*;

use crate::types::Inline;

use super::error::ParseError;

use super::{bold, inline_code, inline_math, italic, linethrough, txt, underline};

pub fn parser() -> impl Parser<char, Inline, Error = ParseError> {
    recursive(|r| {
        let txt = txt::parser_until(
            choice((
                just(" ")
                    .then(choice((
                        just("*"),
                        just("/"),
                        just("~"),
                        just("_"),
                        just("$"),
                        just("`"),
                        just("="),
                    )))
                    .ignored(),
                choice((
                    just("*"),
                    just("/"),
                    just("~"),
                    just("_"),
                    just("$"),
                    just("`"),
                    just("="),
                ))
                .then(just(" "))
                .ignored(),
                text::newline(),
            ))
            .rewind(),
        )
        .map(Inline::Text);

        let code = inline_code::parser().map(Inline::Code);

        let math = inline_math::parser().map(Inline::Math);

        let bold = bold::parser(r.clone()).map(Inline::Bold);

        let italic = italic::parser(r.clone()).map(Inline::Italic);

        let linethrough = linethrough::parser(r.clone()).map(Inline::Linethrough);

        let underline = underline::parser(r.clone()).map(Inline::Underline);

        choice((code, math, bold, italic, linethrough, underline, txt))
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
        dbg!(parser()
            .parse_recovery_verbose("AAAfghjk _dfgh *bold* jkl_  ` *Hi* `  $\\frac{10}{11}$ \n",));
    }
}
