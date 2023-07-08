
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LibraryText {
    pub nodes: (Vec<WhiteSpace>, Vec<LibraryDescription>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum LibraryDescription {
    LibraryDeclaration(Box<LibraryDeclaration>),
    IncludeStatement(Box<IncludeStatement>),
    ConfigDeclaration(Box<ConfigDeclaration>),
    Null(Box<Symbol>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LibraryDeclaration {
    pub nodes: (
        Keyword,
        LibraryIdentifier,
        List<Symbol, FilePathSpec>,
        Option<(Keyword, List<Symbol, FilePathSpec>)>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct IncludeStatement {
    pub nodes: (Keyword, FilePathSpec, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum FilePathSpec {
    Literal(StringLiteral),
    NonLiteral(FilePathSpecNonLiteral),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FilePathSpecNonLiteral {
    pub nodes: (Locate, Vec<WhiteSpace>),
}
