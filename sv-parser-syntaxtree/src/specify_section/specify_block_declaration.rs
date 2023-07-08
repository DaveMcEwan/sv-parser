
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SpecifyBlock {
    pub nodes: (Keyword, Vec<SpecifyItem>, Keyword),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum SpecifyItem {
    SpecparamDeclaration(Box<SpecparamDeclaration>),
    PulsestyleDeclaration(Box<PulsestyleDeclaration>),
    ShowcancelledDeclaration(Box<ShowcancelledDeclaration>),
    PathDeclaration(Box<PathDeclaration>),
    SystemTimingCheck(Box<SystemTimingCheck>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PulsestyleDeclaration {
    pub nodes: (Keyword, ListOfPathOutputs, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ShowcancelledDeclaration {
    pub nodes: (Keyword, ListOfPathOutputs, Symbol),
}
