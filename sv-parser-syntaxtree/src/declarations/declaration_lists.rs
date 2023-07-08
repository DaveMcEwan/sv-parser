
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfDefparamAssignments {
    pub nodes: (List<Symbol, DefparamAssignment>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfGenvarIdentifiers {
    pub nodes: (List<Symbol, GenvarIdentifier>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfInterfaceIdentifiers {
    pub nodes: (List<Symbol, (InterfaceIdentifier, Vec<UnpackedDimension>)>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfNetDeclAssignments {
    pub nodes: (List<Symbol, NetDeclAssignment>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfParamAssignments {
    pub nodes: (List<Symbol, ParamAssignment>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfPortIdentifiers {
    pub nodes: (List<Symbol, (PortIdentifier, Vec<UnpackedDimension>)>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfUdpPortIdentifiers {
    pub nodes: (List<Symbol, PortIdentifier>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfSpecparamAssignments {
    pub nodes: (List<Symbol, SpecparamAssignment>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfTfVariableIdentifiers {
    pub nodes: (
        List<
            Symbol,
            (
                PortIdentifier,
                Vec<VariableDimension>,
                Option<(Symbol, Expression)>,
            ),
        >,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfTypeAssignments {
    pub nodes: (List<Symbol, TypeAssignment>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfVariableDeclAssignments {
    pub nodes: (List<Symbol, VariableDeclAssignment>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfVariableIdentifiers {
    pub nodes: (List<Symbol, (VariableIdentifier, Vec<VariableDimension>)>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfVariablePortIdentifiers {
    pub nodes: (
        List<
            Symbol,
            (
                PortIdentifier,
                Vec<VariableDimension>,
                Option<(Symbol, ConstantExpression)>,
            ),
        >,
    ),
}
