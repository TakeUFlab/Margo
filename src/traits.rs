use crate::token::*;
use crate::types::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait Spanned {
    fn span(&self) -> Span;
}

pub trait Hashing
where
    Self: Hash,
{
    fn hashing(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

pub trait Render {
    type Output;
    type Error;

    fn render_file(&mut self, c: File) -> Result<Self::Output, Self::Error>;

    fn render_block(&mut self, c: Block) -> Result<Self::Output, Self::Error>;

    fn render_block_heading(&mut self, c: BlockHeading) -> Result<Self::Output, Self::Error>;

    fn render_block_paragraph(&mut self, c: BlockParagraph) -> Result<Self::Output, Self::Error>;

    fn render_block_code(&mut self, c: BlockCode) -> Result<Self::Output, Self::Error>;

    fn render_inline(&mut self, c: Inline) -> Result<Self::Output, Self::Error>;

    fn render_inline_bold(&mut self, c: InlineBold) -> Result<Self::Output, Self::Error>;

    fn render_inline_code(&mut self, c: InlineCode) -> Result<Self::Output, Self::Error>;

    fn render_inline_italic(&mut self, c: InlineItalic) -> Result<Self::Output, Self::Error>;

    fn render_inline_strikethrough(
        &mut self,
        c: InlineStrikethrough,
    ) -> Result<Self::Output, Self::Error>;

    fn render_inline_math(&mut self, c: InlineMath) -> Result<Self::Output, Self::Error>;

    fn render_inline_underline(&mut self, c: InlineUnderline) -> Result<Self::Output, Self::Error>;

    fn render_text(&mut self, c: Text) -> Result<Self::Output, Self::Error>;

    // fn render_ident(&mut self, c: Ident) -> Result<Self::Output, Self::Error>;

    // fn render_span(&mut self, c: Span) -> Result<Self::Output, Self::Error>;

    // fn render_(&mut self, c: ) -> Result<Self::Output,Self::Error> {
    // }
}

impl<T> Spanned for (T, Span) {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl<T> Hashing for T where T: Hash {}
