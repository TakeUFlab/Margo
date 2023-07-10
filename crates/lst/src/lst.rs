use num_enum::{IntoPrimitive, TryFromPrimitive};
use rowan::Language;

pub type SynNode = rowan::SyntaxNode<Lang>;
pub type SynToken = rowan::SyntaxToken<Lang>;
pub type SynChild = rowan::NodeOrToken<SynNode, SynToken>;
pub type Node = rowan::GreenNode;
pub type Token = rowan::GreenToken;
pub type Child = rowan::NodeOrToken<Node, Token>;
pub use self::Syntax::*;

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive,
)]
#[repr(u16)]
pub enum Syntax {
    // char level
    Space,
    Newline,

    /// root level
    File,

    /// block level
    Block,

    Paragraph,

    Heading,
    LpHeading,

    Code,
    LRpCode,
    LangTag,

    Math,
    LpMath,
    RpMath,

    List,

    /// inline level
    Inline,

    Bold,
    LpBold,
    RpBold,

    Italic,
    LpItalic,
    RpItalic,

    Underline,
    LpUnderline,
    RpUnderline,

    Strikethrough,
    LpStrikethrough,
    RpStrikethrough,

    LineCode,
    LpLineCode,
    RpLineCode,

    LineMath,
    LpLineMath,
    RpLineMath,

    Text,
}

impl Into<rowan::SyntaxKind> for Syntax {
    fn into(self) -> rowan::SyntaxKind {
        rowan::SyntaxKind(self.into())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lang {}

impl Language for Lang {
    type Kind = Syntax;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        raw.0.try_into().unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
