
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct AttributeInstance {
    pub nodes: (Symbol, List<Symbol, AttrSpec>, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct AttrSpec {
    pub nodes: (Identifier, Option<(Symbol, ConstantExpression)>),
}
