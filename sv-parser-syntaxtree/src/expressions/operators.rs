
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct UnaryOperator {
    pub nodes: (Symbol,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct BinaryOperator {
    pub nodes: (Symbol,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct IncOrDecOperator {
    pub nodes: (Symbol,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct UnaryModulePathOperator {
    pub nodes: (Symbol,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct BinaryModulePathOperator {
    pub nodes: (Symbol,),
}
