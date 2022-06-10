use std::collections::HashMap;

use margo::Render;
use vue::H;
use wasm_bindgen::prelude::*;
mod vue;

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct Builder {
    components: HashMap<Types, JsValue>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[wasm_bindgen]
pub enum Types {
    File,
    Block,
    BlockHeading,
    BlockParagraph,
    BlockCode,
    Inline,
    InlineBold,
    InlineCode,
    InlineItalic,
    InlineLinethrough,
    InlineMath,
    InlineUnderline,
    Text,
}

#[wasm_bindgen]
impl Builder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    #[wasm_bindgen(js_name = setComponent)]
    pub fn set_component(mut self, key: Types, value: JsValue) -> Self {
        self.components.insert(key, value);
        self
    }

    #[wasm_bindgen(js_name = getComponent)]
    pub fn get_component(&self, t: Types) -> Result<JsValue, JsError> {
        Ok(self
            .components
            .get(&t)
            .ok_or(JsError::new("Not Find Component"))?
            .clone())
    }

    pub fn parse(&mut self, c: String) -> Result<JsValue, JsError> {
        let ast = margo::parse_string(c).map_err(|_| JsError::new("Parse Error"))?;
        self.render_file(ast)
    }
}

impl Render for Builder {
    type Output = JsValue;

    type Error = JsError;

    fn render_file(&mut self, c: margo::tokenstream::File) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::File)?)
            .add_property("key", js_hash(c.hash))
            .add_child(self.render_block(c.content)?)
            .h())
    }

    fn render_block(&mut self, c: margo::tokenstream::Block) -> Result<Self::Output, Self::Error> {
        match c {
            margo::tokenstream::Block::Blocks(v) => {
                let mut f = vec![];
                for i in v {
                    f.push(self.render_block(i)?);
                }
                Ok(js_sys::Array::from_iter(f).into())
            }
            margo::tokenstream::Block::Paragraph(v) => self.render_block_paragraph(v),
            margo::tokenstream::Block::Heading(v) => self.render_block_heading(v),
            margo::tokenstream::Block::Code(v) => self.render_block_code(v),
            _ => todo!(),
        }
    }

    fn render_block_heading(
        &mut self,
        c: margo::tokenstream::BlockHeading,
    ) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::BlockHeading)?)
            .add_property("key", js_hash(c.hash))
            .add_property("level", c.level)
            .add_child(self.render_text(c.content)?)
            .h())
    }

    fn render_block_paragraph(
        &mut self,
        c: margo::tokenstream::BlockParagraph,
    ) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::BlockParagraph)?)
            .add_property("key", js_hash(c.hash))
            .add_child(self.render_inline(c.content)?)
            .h())
    }

    fn render_block_code(
        &mut self,
        c: margo::tokenstream::BlockCode,
    ) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::BlockCode)?)
            .add_property("key", js_hash(c.hash))
            .add_property("lang", c.lang.map(|x| x.content))
            .add_child(self.render_text(c.content)?)
            .h())
    }

    fn render_inline(
        &mut self,
        c: margo::tokenstream::Inline,
    ) -> Result<Self::Output, Self::Error> {
        match c {
            margo::tokenstream::Inline::Inlines(v) => {
                let mut f = vec![];
                for i in v {
                    f.push(self.render_inline(i)?);
                }
                Ok(js_sys::Array::from_iter(f).into())
            }
            margo::tokenstream::Inline::Bold(v) => self.render_inline_bold(v),
            margo::tokenstream::Inline::Italic(v) => self.render_inline_italic(v),
            margo::tokenstream::Inline::Underline(v) => self.render_inline_underline(v),
            margo::tokenstream::Inline::Linethrough(v) => self.render_inline_linethrough(v),
            margo::tokenstream::Inline::Code(v) => self.render_inline_code(v),
            margo::tokenstream::Inline::Math(v) => self.render_inline_math(v),
            margo::tokenstream::Inline::Text(v) => self.render_text(v),
            _ => todo!(),
        }
    }

    fn render_inline_bold(
        &mut self,
        c: margo::tokenstream::InlineBold,
    ) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::InlineBold)?)
            .add_property("key", js_hash(c.hash))
            .add_child(self.render_inline(*c.content)?)
            .h())
    }

    fn render_inline_code(
        &mut self,
        c: margo::tokenstream::InlineCode,
    ) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::InlineCode)?)
            .add_property("key", js_hash(c.hash))
            .add_child(self.render_text(c.content)?)
            .h())
    }

    fn render_inline_italic(
        &mut self,
        c: margo::tokenstream::InlineItalic,
    ) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::InlineItalic)?)
            .add_property("key", js_hash(c.hash))
            .add_child(self.render_inline(*c.content)?)
            .h())
    }

    fn render_inline_linethrough(
        &mut self,
        c: margo::tokenstream::InlineLinethrough,
    ) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::InlineLinethrough)?)
            .add_property("key", js_hash(c.hash))
            .add_child(self.render_inline(*c.content)?)
            .h())
    }

    fn render_inline_math(
        &mut self,
        c: margo::tokenstream::InlineMath,
    ) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::InlineMath)?)
            .add_property("key", js_hash(c.hash))
            .add_child(self.render_text(c.content)?)
            .h())
    }

    fn render_inline_underline(
        &mut self,
        c: margo::tokenstream::InlineUnderline,
    ) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::InlineUnderline)?)
            .add_property("key", js_hash(c.hash))
            .add_child(self.render_inline(*c.content)?)
            .h())
    }

    fn render_text(&mut self, c: margo::tokenstream::Text) -> Result<Self::Output, Self::Error> {
        Ok(H::new()
            .component(self.get_component(Types::Text)?)
            .add_property("key", js_hash(c.hash))
            .add_child(c.content)
            .h())
    }
}

fn js_hash(h: u64) -> usize {
    (h % usize::MAX as u64) as usize
}
