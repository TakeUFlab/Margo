use crate::token::Span;
use crate::traits::Spanned;
use chumsky::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseLabel {
    UnExpected,
    NotABlock,
    NotAInline,
    MissingNewline,
    MissingBlockNewline,
    MissingSpace,
    MissingEnd,
    CannotEmpty,
    Unclosed(Span, char),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub span: Span,
    pub expect: HashMap<char, usize>,
    pub found: Option<char>,
    pub label: ParseLabel,
}

impl ParseError {
    pub fn new(span: Span, label: ParseLabel) -> Self {
        ParseError {
            span,
            label,
            expect: Default::default(),
            found: Default::default(),
        }
    }
}

impl ParseLabel {
    pub fn level(&self) -> usize {
        match self {
            ParseLabel::UnExpected => 0,
            ParseLabel::NotABlock => 1,
            ParseLabel::NotAInline => 1,
            ParseLabel::CannotEmpty => 1,
            ParseLabel::MissingNewline => 2,
            ParseLabel::MissingBlockNewline => 2,
            ParseLabel::MissingSpace => 2,
            ParseLabel::MissingEnd => 2,
            ParseLabel::Unclosed(..) => usize::MAX,
        }
    }
}

impl Error<char> for ParseError {
    type Span = Span;
    type Label = ParseLabel;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expect: Iter,
        found: Option<char>,
    ) -> Self {
        Self {
            span,
            label: ParseLabel::UnExpected,
            expect: expect
                .into_iter()
                .filter_map(|x| x.map(|y| (y, 0)))
                .collect(),
            found,
        }
    }

    fn unclosed_delimiter(
        unclosed_span: Self::Span,
        unclosed: char,
        span: Self::Span,
        expected: char,
        found: Option<char>,
    ) -> Self {
        Self {
            span,
            expect: HashMap::from_iter([(expected, 0)]),
            found,
            label: ParseLabel::Unclosed(unclosed_span, unclosed),
        }
    }

    fn with_label(mut self, label: Self::Label) -> Self {
        self.label = label;
        self
    }

    fn merge(mut self, other: Self) -> Self {
        if self.label.level() < other.label.level() {
            self.label = other.label;
        }

        for (k, c) in other.expect {
            if let Some(x) = self.expect.get_mut(&k) {
                *x += c + 1;
            } else {
                self.expect.insert(k, c + 1);
            }
        }
        other.found.map(|x| self.found.get_or_insert(x));
        self
    }
}

impl Spanned for ParseError {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
