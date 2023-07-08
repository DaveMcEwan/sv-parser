
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EnableTerminal {
    pub nodes: (Expression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InoutTerminal {
    pub nodes: (NetLvalue,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InputTerminal {
    pub nodes: (Expression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NcontrolTerminal {
    pub nodes: (Expression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct OutputTerminal {
    pub nodes: (NetLvalue,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PcontrolTerminal {
    pub nodes: (Expression,),
}
