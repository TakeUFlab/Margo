use chumsky::Parser;
mod lexer;
mod token;
mod traits;
mod types;

pub use traits::*;

pub mod tokenstream {
    pub use super::token::*;
    pub use super::types::*;
}

pub mod parse {
    pub use super::lexer::*;
}

pub fn parse_string(content: String) -> Result<types::File, Vec<lexer::error::ParseError>> {
    lexer::file::parser().parse(content)
}
