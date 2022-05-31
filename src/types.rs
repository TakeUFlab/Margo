use crate::token::{Span, Token};

#[cfg(feature = "serde")]
use serde::Serialize;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Block {
    Paragraph(BlockParagraph),
    Heading(BlockHeading),
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Inline {
    Inlines(Vec<Inline>),
    Bold(InlineBold),
    Italic(InlineItalic),
    Underline(InlineUnderline),
    Linethrough(InlineLinethrough),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct File {
    pub content: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BlockParagraph {
    pub span: Span,
    pub content: Inline,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BlockHeading {
    pub span: Span,
    pub level: usize,
    pub content: Text,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct InlineBold {
    pub span: Span,
    pub content: Box<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct InlineItalic {
    pub span: Span,
    pub content: Box<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct InlineUnderline {
    pub span: Span,
    pub content: Box<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct InlineLinethrough {
    pub span: Span,
    pub content: Box<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Text {
    pub span: Span,
    pub content: String,
}
