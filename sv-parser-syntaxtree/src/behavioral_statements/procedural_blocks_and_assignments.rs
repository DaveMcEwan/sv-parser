
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InitialConstruct {
    pub nodes: (Keyword, StatementOrNull),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct AlwaysConstruct {
    pub nodes: (AlwaysKeyword, Statement),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum AlwaysKeyword {
    Always(Box<Keyword>),
    AlwaysComb(Box<Keyword>),
    AlwaysLatch(Box<Keyword>),
    AlwaysFf(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FinalConstruct {
    pub nodes: (Keyword, FunctionStatement),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum BlockingAssignment {
    Variable(Box<BlockingAssignmentVariable>),
    NonrangeVariable(Box<BlockingAssignmentNonrangeVariable>),
    HierarchicalVariable(Box<BlockingAssignmentHierarchicalVariable>),
    OperatorAssignment(Box<OperatorAssignment>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct BlockingAssignmentVariable {
    pub nodes: (VariableLvalue, Symbol, DelayOrEventControl, Expression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct BlockingAssignmentNonrangeVariable {
    pub nodes: (NonrangeVariableLvalue, Symbol, DynamicArrayNew),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct BlockingAssignmentHierarchicalVariable {
    pub nodes: (
        Option<ImplicitClassHandleOrClassScopeOrPackageScope>,
        HierarchicalVariableIdentifier,
        Select,
        Symbol,
        ClassNew,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct OperatorAssignment {
    pub nodes: (VariableLvalue, AssignmentOperator, Expression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct AssignmentOperator {
    pub nodes: (Symbol,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NonblockingAssignment {
    pub nodes: (
        VariableLvalue,
        Symbol,
        Option<DelayOrEventControl>,
        Expression,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ProceduralContinuousAssignment {
    Assign(Box<ProceduralContinuousAssignmentAssign>),
    Deassign(Box<ProceduralContinuousAssignmentDeassign>),
    ForceVariable(Box<ProceduralContinuousAssignmentForceVariable>),
    ForceNet(Box<ProceduralContinuousAssignmentForceNet>),
    ReleaseVariable(Box<ProceduralContinuousAssignmentReleaseVariable>),
    ReleaseNet(Box<ProceduralContinuousAssignmentReleaseNet>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProceduralContinuousAssignmentAssign {
    pub nodes: (Keyword, VariableAssignment),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProceduralContinuousAssignmentDeassign {
    pub nodes: (Keyword, VariableLvalue),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProceduralContinuousAssignmentForceVariable {
    pub nodes: (Keyword, VariableAssignment),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProceduralContinuousAssignmentForceNet {
    pub nodes: (Keyword, NetAssignment),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProceduralContinuousAssignmentReleaseVariable {
    pub nodes: (Keyword, VariableLvalue),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProceduralContinuousAssignmentReleaseNet {
    pub nodes: (Keyword, NetLvalue),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VariableAssignment {
    pub nodes: (VariableLvalue, Symbol, Expression),
}
