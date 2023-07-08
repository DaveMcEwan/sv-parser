
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProgramInstantiation {
    pub nodes: (
        ProgramIdentifier,
        Option<ParameterValueAssignment>,
        List<Symbol, HierarchicalInstance>,
        Symbol,
    ),
}
