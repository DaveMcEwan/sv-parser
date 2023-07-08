
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum NetLvalue {
    Identifier(Box<NetLvalueIdentifier>),
    Lvalue(Box<NetLvalueLvalue>),
    Pattern(Box<NetLvaluePattern>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NetLvalueIdentifier {
    pub nodes: (PsOrHierarchicalNetIdentifier, ConstantSelect),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NetLvalueLvalue {
    pub nodes: (Brace<List<Symbol, NetLvalue>>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NetLvaluePattern {
    pub nodes: (
        Option<AssignmentPatternExpressionType>,
        AssignmentPatternNetLvalue,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum VariableLvalue {
    Identifier(Box<VariableLvalueIdentifier>),
    Lvalue(Box<VariableLvalueLvalue>),
    Pattern(Box<VariableLvaluePattern>),
    StreamingConcatenation(Box<StreamingConcatenation>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VariableLvalueIdentifier {
    pub nodes: (
        Option<ImplicitClassHandleOrPackageScope>,
        HierarchicalVariableIdentifier,
        Select,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VariableLvalueLvalue {
    pub nodes: (Brace<List<Symbol, VariableLvalue>>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VariableLvaluePattern {
    pub nodes: (
        Option<AssignmentPatternExpressionType>,
        AssignmentPatternVariableLvalue,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NonrangeVariableLvalue {
    pub nodes: (
        Option<ImplicitClassHandleOrPackageScope>,
        HierarchicalVariableIdentifier,
        NonrangeSelect,
    ),
}
