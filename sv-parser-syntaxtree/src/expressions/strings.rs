
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct StringLiteral {
    pub nodes: (Locate, Vec<WhiteSpace>),
}
