use crate::token::Span;

#[cfg(feature = "serde")]
use serde::Serialize;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub enum Block {
    Paragraph(BlockParagraph),
    Heading(BlockHeading),
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub enum Inline {
    Inlines(Vec<Inline>),
    Bold(InlineBold),
    Italic(InlineItalic),
    Underline(InlineUnderline),
    Linethrough(InlineLinethrough),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct File {
    pub content: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct BlockParagraph {
    pub span: Span,
    pub content: Inline,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct BlockHeading {
    pub span: Span,
    pub level: usize,
    pub content: Text,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineBold {
    pub span: Span,
    pub content: Box<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineItalic {
    pub span: Span,
    pub content: Box<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineUnderline {
    pub span: Span,
    pub content: Box<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineLinethrough {
    pub span: Span,
    pub content: Box<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct Text {
    pub span: Span,
    pub content: String,
}
