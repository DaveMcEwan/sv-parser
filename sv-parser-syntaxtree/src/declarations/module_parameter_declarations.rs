
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum LocalParameterDeclaration {
    Param(Box<LocalParameterDeclarationParam>),
    Type(Box<LocalParameterDeclarationType>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LocalParameterDeclarationParam {
    pub nodes: (Keyword, DataTypeOrImplicit, ListOfParamAssignments),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LocalParameterDeclarationType {
    pub nodes: (Keyword, Keyword, ListOfTypeAssignments),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ParameterDeclaration {
    Param(Box<ParameterDeclarationParam>),
    Type(Box<ParameterDeclarationType>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ParameterDeclarationParam {
    pub nodes: (Keyword, DataTypeOrImplicit, ListOfParamAssignments),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ParameterDeclarationType {
    pub nodes: (Keyword, Keyword, ListOfTypeAssignments),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SpecparamDeclaration {
    pub nodes: (
        Keyword,
        Option<PackedDimension>,
        ListOfSpecparamAssignments,
        Symbol,
    ),
}
