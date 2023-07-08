
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Symbol {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Keyword {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum WhiteSpace {
    Newline(Box<Locate>),
    Space(Box<Locate>),
    Comment(Box<Comment>),
    CompilerDirective(Box<CompilerDirective>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Paren<T> {
    pub nodes: (Symbol, T, Symbol),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brace<T> {
    pub nodes: (Symbol, T, Symbol),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bracket<T> {
    pub nodes: (Symbol, T, Symbol),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApostropheBrace<T> {
    pub nodes: (Symbol, T, Symbol),
}

#[derive(Clone, Debug, PartialEq)]
pub struct List<T, U> {
    pub nodes: (U, Vec<(T, U)>),
}

impl<T, U> List<T, U> {
    pub fn contents(&self) -> Vec<&U> {
        let mut ret = vec![];
        let (ref x, ref y) = self.nodes;
        ret.push(x);
        for (_, y) in y {
            ret.push(y)
        }
        ret
    }
}
