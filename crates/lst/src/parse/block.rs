use super::inline::inline;
use crate::lst::*;
use crate::nom::*;
use nom::branch::alt;
use nom::bytes::complete::take_till1;
use nom::bytes::complete::take_until1;
use nom::combinator::cut;
use nom::combinator::opt;
use nom::combinator::{all_consuming, eof};
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::Parser;

static LP_CODE: &str = "```";
static RP_CODE: &str = "\n```\n";

static LP_MATH: &str = "$$$";
static RP_MATH: &str = "\n$$$\n";

// static LP_: &str = " *";
// static RP_: &str = "* ";

pub fn block(t: &str) -> IResult {
    let (r, (n, _)) = many_till(alt((code, paragraph, c_newline)), eof)(t)?;

    Ok((r, c_node(Block)(n)))
}

pub fn paragraph(t: &str) -> IResult {
    let (r, n) = take_till1(is_newline)(t)?;
    let (_, n) = all_consuming(inline)(n)?;
    Ok((r, c_node(Paragraph)(vec![n])))
}

pub fn code(t: &str) -> IResult {
    let (r, n) = tuple((
        c_tag(LRpCode, LP_CODE),
        opt(take_until1(NEWLINE).map(c_token(LangTag))),
        cut(take_until1(RP_CODE).map(c_token(Text))),
        cut(c_newline),
        cut(c_tag(LRpCode, RP_CODE)),
        cut(c_newline),
    ))(t)?;

    let mut v = vec![n.0];
    n.1.map(|x| v.push(x));
    v.extend_from_slice(&[n.2, n.3]);
    Ok((r, c_node(Code)(v)))
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::*;

    #[test]
    fn test_paragraph() {
        let l = SynNode::new_root(Node::new(
            File.into(),
            [
                paragraph("this is a  *long*  text about /Hello World/ :)\n")
                    .unwrap()
                    .1,
            ],
        ));
        let e = expect![[r#"
            File@0..46
              Paragraph@0..46
                Inline@0..46
                  Text@0..10 "this is a "
                  Bold@10..18
                    LpBold@10..12 " *"
                    Inline@12..16
                      Text@12..16 "long"
                    RpBold@16..18 "* "
                  Text@18..29 " text about"
                  Italic@29..44
                    LpItalic@29..31 " /"
                    Inline@31..42
                      Text@31..42 "Hello World"
                    LpItalic@42..44 "/ "
                  Text@44..46 ":)"

        "#]];
        e.assert_debug_eq(&l);
    }
}
