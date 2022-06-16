/// All possible types that rune can have
/// This structure is heavily based on syn
use crate::ast::prelude::*;

/// The possible types that a Rune value could have.
#[derive(Debug, Clone, PartialEq, Eq, ToTokens, Spanned)]
#[non_exhaustive]
pub enum Type {
    /// A path pattern like `Color::Red`
    Path(TypePath), // TODO: _ type
}

impl Parse for Type {
    fn parse(p: &mut Parser) -> Result<Self, ParseError> {
        let path: ast::Path = p.parse()?;
        Ok(Self::Path(TypePath { path }))
    }
}

/// A path pattern like `Color::Red`
#[derive(Debug, Clone, PartialEq, Eq, ToTokens, Spanned)]
#[non_exhaustive]
pub struct TypePath {
    /// The path of the type
    pub path: ast::Path,
}
