
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum SubroutineCallStatement {
    SubroutineCall(Box<(SubroutineCall, Symbol)>),
    Function(Box<SubroutineCallStatementFunction>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SubroutineCallStatementFunction {
    pub nodes: (Keyword, Symbol, Paren<FunctionSubroutineCall>, Symbol),
}
