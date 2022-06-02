use crate::token::Span;
use crate::traits::Spanned;
use chumsky::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseLabel {
    UnExpected,
    NotABlock,
    SpaceRequire,
    CannotEmpty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub span: Span,
    pub label: Option<ParseLabel>,
}

impl ParseError {
    pub fn new(span: Span) -> Self {
        ParseError { span, label: None }
    }
}

impl Error<char> for ParseError {
    type Span = Span;
    type Label = ParseLabel;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        _: Iter,
        _: Option<char>,
    ) -> Self {
        Self { span, label: None }
    }

    fn with_label(mut self, label: Self::Label) -> Self {
        self.label.get_or_insert(label);
        self
    }

    fn merge(mut self, other: Self) -> Self {
        if let Some(label) = other.label {
            self.label.get_or_insert(label);
        }
        self
    }
}

impl Spanned for ParseError {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
