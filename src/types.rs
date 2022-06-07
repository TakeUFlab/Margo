use crate::token::{Ident, Span};

#[cfg(feature = "serde")]
use serde::Serialize;

#[non_exhaustive]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "hashing", derive(Hash))]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub enum Block {
    Paragraph(BlockParagraph),
    Heading(BlockHeading),
    Code(BlockCode),
}

#[non_exhaustive]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "hashing", derive(Hash))]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub enum Inline {
    Inlines(Vec<Inline>),
    Bold(InlineBold),
    Italic(InlineItalic),
    Underline(InlineUnderline),
    Linethrough(InlineLinethrough),
    Code(InlineCode),
    Math(InlineMath),
    Text(Text),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct File {
    pub content: Vec<Block>,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct BlockParagraph {
    pub span: Span,
    pub content: Inline,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct BlockHeading {
    pub span: Span,
    pub level: usize,
    pub content: Text,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct BlockCode {
    pub span: Span,
    pub lang: Option<Ident>,
    pub content: Text,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineBold {
    pub span: Span,
    pub content: Box<Inline>,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineItalic {
    pub span: Span,
    pub content: Box<Inline>,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineUnderline {
    pub span: Span,
    pub content: Box<Inline>,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineLinethrough {
    pub span: Span,
    pub content: Box<Inline>,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineCode {
    pub span: Span,
    pub content: Text,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct InlineMath {
    pub span: Span,
    pub content: Text,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub struct Text {
    pub span: Span,
    pub content: String,
    #[cfg(feature = "hashing")]
    pub hash: u64,
}
