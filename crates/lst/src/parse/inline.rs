use crate::lst::*;
use crate::nom::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::anychar;
use nom::combinator::all_consuming;
use nom::combinator::{cut, eof, peek};
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::Parser;

static LP_BOLD: &str = " *";
static RP_BOLD: &str = "* ";

static LP_ITALIC: &str = " /";
static RP_ITALIC: &str = "/ ";

static LP_STRIKETHROUGH: &str = " ~";
static RP_STRIKETHROUGH: &str = "~ ";

static LP_LINECODE: &str = " `";
static RP_LINECODE: &str = "` ";

static LP_LINEMATH: &str = " $";
static RP_LINEMATH: &str = "$ ";

pub fn inline(t: &str) -> IResult {
    let (r, (n, _)) = many_till(
        alt((bold, italic, strikethrough, linecode, linemath, text)),
        eof,
    )(t)?;

    Ok((r, c_node(Inline)(n)))
}

pub fn bold(t: &str) -> IResult {
    let (r, n) = tuple((
        c_tag(LpBold, LP_BOLD),
        cut(take_until1(RP_BOLD)),
        cut(c_tag(RpBold, RP_BOLD)),
    ))(t)?;

    let (_, ni) = all_consuming(inline)(n.1)?;
    Ok((r, c_node(Bold)(vec![n.0, ni, n.2])))
}

pub fn italic(t: &str) -> IResult {
    let (r, n) = tuple((
        c_tag(LpItalic, LP_ITALIC),
        cut(take_until1(RP_ITALIC)),
        cut(c_tag(LpItalic, RP_ITALIC)),
    ))(t)?;

    let (_, ni) = all_consuming(inline)(n.1)?;
    Ok((r, c_node(Italic)(vec![n.0, ni, n.2])))
}

pub fn strikethrough(t: &str) -> IResult {
    let (r, n) = tuple((
        c_tag(LpStrikethrough, LP_STRIKETHROUGH),
        cut(take_until1(RP_STRIKETHROUGH)),
        cut(c_tag(RpStrikethrough, RP_STRIKETHROUGH)),
    ))(t)?;

    let (_, ni) = all_consuming(inline)(n.1)?;
    Ok((r, c_node(Strikethrough)(vec![n.0, ni, n.2])))
}

pub fn linecode(t: &str) -> IResult {
    let (r, n) = tuple((
        c_tag(LpLineCode, LP_LINECODE),
        cut(take_until1(RP_LINECODE).map(c_token(Text))),
        cut(c_tag(RpLineCode, RP_LINECODE)),
    ))(t)?;

    Ok((r, c_node(LineCode)(vec![n.0, n.1, n.2])))
}

pub fn linemath(t: &str) -> IResult {
    let (r, n) = tuple((
        c_tag(LpLineMath, LP_LINEMATH),
        cut(take_until1(RP_LINEMATH).map(c_token(Text))),
        cut(c_tag(RpLineMath, RP_LINEMATH)),
    ))(t)?;

    Ok((r, c_node(LineMath)(vec![n.0, n.1, n.2])))
}

pub fn text(t: &str) -> IResult {
    if t.is_empty() {
        return Err(Error::new(t, EKind::NonEmpty));
    }
    let (r, (s, _)) = many_till(
        anychar,
        peek(alt((
            tag(LP_BOLD),
            tag(LP_ITALIC),
            tag(LP_STRIKETHROUGH),
            tag(LP_LINECODE),
            tag(LP_LINEMATH),
            eof,
        ))),
    )(t)?;
    Ok((r, c_token(Text)(&s.into_iter().collect::<String>())))
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::*;

    #[test]
    fn test_text() {
        let l = SynNode::new_root(Node::new(
            File.into(),
            [text("this is a long text about Hello World").unwrap().1],
        ));
        let e = expect![[r#"
            File@0..37
              Text@0..37 "this is a long text a ..."

        "#]];
        e.assert_debug_eq(&l);
    }

    #[test]
    fn test_bold() {
        let l = SynNode::new_root(Node::new(
            File.into(),
            [bold(" *this is a long text about Hello World* ").unwrap().1],
        ));
        let e = expect![[r#"
            File@0..41
              Bold@0..41
                LpBold@0..2 " *"
                Inline@2..39
                  Text@2..39 "this is a long text a ..."
                RpBold@39..41 "* "

        "#]];
        e.assert_debug_eq(&l);
    }

    #[test]
    fn test_italic() {
        let l = SynNode::new_root(Node::new(
            File.into(),
            [italic(" /this is a long text about Hello World/ ")
                .unwrap()
                .1],
        ));
        let e = expect![[r#"
            File@0..41
              Italic@0..41
                LpItalic@0..2 " /"
                Inline@2..39
                  Text@2..39 "this is a long text a ..."
                LpItalic@39..41 "/ "

        "#]];
        e.assert_debug_eq(&l);
    }

    #[test]
    fn test_strikethrough() {
        let l = SynNode::new_root(Node::new(
            File.into(),
            [strikethrough(" ~this is a long text about Hello World~ ")
                .unwrap()
                .1],
        ));
        let e = expect![[r#"
            File@0..41
              Strikethrough@0..41
                LpStrikethrough@0..2 " ~"
                Inline@2..39
                  Text@2..39 "this is a long text a ..."
                RpStrikethrough@39..41 "~ "

        "#]];
        e.assert_debug_eq(&l);
    }

    #[test]
    fn test_linecode() {
        let l = SynNode::new_root(Node::new(
            File.into(),
            [linecode(" `fn rust() -> RS { ... }` ").unwrap().1],
        ));
        let e = expect![[r#"
            File@0..27
              LineCode@0..27
                LpLineCode@0..2 " `"
                Text@2..25 "fn rust() -> RS { ... }"
                RpLineCode@25..27 "` "

        "#]];
        e.assert_debug_eq(&l);
    }

    #[test]
    fn test_linemath() {
        let l = SynNode::new_root(Node::new(
            File.into(),
            [linemath(r" $\frac{e^{i\pi}}{-1} = 1$ ").unwrap().1],
        ));
        let e = expect![[r#"
            File@0..27
              LineMath@0..27
                LpLineMath@0..2 " $"
                Text@2..25 "\\frac{e^{i\\pi}}{-1} = 1"
                RpLineMath@25..27 "$ "

        "#]];
        e.assert_debug_eq(&l);
    }

    #[test]
    fn test_inline() {
        let l = SynNode::new_root(Node::new(
            File.into(),
            [inline(
                "some text *this* is a  ~long~  text  /about/  Hello World `code` and $math$ ",
            )
            .unwrap()
            .1],
        ));
        let e = expect![[r#"
            File@0..76
              Inline@0..76
                Text@0..9 "some text"
                Bold@9..17
                  LpBold@9..11 " *"
                  Inline@11..15
                    Text@11..15 "this"
                  RpBold@15..17 "* "
                Text@17..22 "is a "
                Strikethrough@22..30
                  LpStrikethrough@22..24 " ~"
                  Inline@24..28
                    Text@24..28 "long"
                  RpStrikethrough@28..30 "~ "
                Text@30..36 " text "
                Italic@36..45
                  LpItalic@36..38 " /"
                  Inline@38..43
                    Text@38..43 "about"
                  LpItalic@43..45 "/ "
                Text@45..57 " Hello World"
                LineCode@57..65
                  LpLineCode@57..59 " `"
                  Text@59..63 "code"
                  RpLineCode@63..65 "` "
                Text@65..68 "and"
                LineMath@68..76
                  LpLineMath@68..70 " $"
                  Text@70..74 "math"
                  RpLineMath@74..76 "$ "

        "#]];
        e.assert_debug_eq(&l);
    }
}

// let e = expect![r#""#];
// e.assert_debug_eq(&l);

// #[test]
// fn test_italic() {
//     let l = SynNode::new_root(Node::new(
//         File.into(),
//         [italic(" /this is a long text about Hello World/ ")
//             .unwrap()
//             .1],
//     ));
//     let e = expect![[r#""#]];
//     e.assert_debug_eq(&l);
// }
