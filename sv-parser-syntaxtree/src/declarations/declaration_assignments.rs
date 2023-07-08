
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DefparamAssignment {
    pub nodes: (
        HierarchicalParameterIdentifier,
        Symbol,
        ConstantMintypmaxExpression,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NetDeclAssignment {
    pub nodes: (
        NetIdentifier,
        Vec<UnpackedDimension>,
        Option<(Symbol, Expression)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ParamAssignment {
    pub nodes: (
        ParameterIdentifier,
        Vec<UnpackedDimension>,
        Option<(Symbol, ConstantParamExpression)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum SpecparamAssignment {
    Mintypmax(Box<SpecparamAssignmentMintypmax>),
    PulseControlSpecparam(Box<PulseControlSpecparam>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SpecparamAssignmentMintypmax {
    pub nodes: (SpecparamIdentifier, Symbol, ConstantMintypmaxExpression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TypeAssignment {
    pub nodes: (TypeIdentifier, Option<(Symbol, DataType)>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PulseControlSpecparam {
    WithoutDescriptor(Box<PulseControlSpecparamWithoutDescriptor>),
    WithDescriptor(Box<PulseControlSpecparamWithDescriptor>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PulseControlSpecparamWithoutDescriptor {
    pub nodes: (
        Symbol,
        Symbol,
        Paren<(RejectLimitValue, Option<(Symbol, ErrorLimitValue)>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PulseControlSpecparamWithDescriptor {
    pub nodes: (
        Symbol,
        SpecifyInputTerminalDescriptor,
        Symbol,
        SpecifyOutputTerminalDescriptor,
        Symbol,
        Paren<(RejectLimitValue, Option<(Symbol, ErrorLimitValue)>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ErrorLimitValue {
    pub nodes: (LimitValue,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RejectLimitValue {
    pub nodes: (LimitValue,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LimitValue {
    pub nodes: (ConstantMintypmaxExpression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum VariableDeclAssignment {
    Variable(Box<VariableDeclAssignmentVariable>),
    DynamicArray(Box<VariableDeclAssignmentDynamicArray>),
    Class(Box<VariableDeclAssignmentClass>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VariableDeclAssignmentVariable {
    pub nodes: (
        VariableIdentifier,
        Vec<VariableDimension>,
        Option<(Symbol, Expression)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VariableDeclAssignmentDynamicArray {
    pub nodes: (
        DynamicArrayVariableIdentifier,
        UnsizedDimension,
        Vec<VariableDimension>,
        Option<(Symbol, DynamicArrayNew)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VariableDeclAssignmentClass {
    pub nodes: (ClassVariableIdentifier, (Symbol, ClassNew)),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ClassNew {
    Argument(Box<ClassNewArgument>),
    Expression(Box<ClassNewExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClassNewArgument {
    pub nodes: (Option<ClassScope>, Keyword, Option<Paren<ListOfArguments>>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClassNewExpression {
    pub nodes: (Keyword, Expression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DynamicArrayNew {
    pub nodes: (Keyword, Bracket<Expression>, Option<Paren<Expression>>),
}
