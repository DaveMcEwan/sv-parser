
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InoutDeclaration {
    pub nodes: (Keyword, NetPortType, ListOfPortIdentifiers),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum InputDeclaration {
    Net(Box<InputDeclarationNet>),
    Variable(Box<InputDeclarationVariable>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InputDeclarationNet {
    pub nodes: (Keyword, NetPortType, ListOfPortIdentifiers),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InputDeclarationVariable {
    pub nodes: (Keyword, VariablePortType, ListOfVariableIdentifiers),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum OutputDeclaration {
    Net(Box<OutputDeclarationNet>),
    Variable(Box<OutputDeclarationVariable>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct OutputDeclarationNet {
    pub nodes: (Keyword, NetPortType, ListOfPortIdentifiers),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct OutputDeclarationVariable {
    pub nodes: (Keyword, VariablePortType, ListOfVariablePortIdentifiers),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InterfacePortDeclaration {
    pub nodes: (
        InterfaceIdentifier,
        Option<(Symbol, ModportIdentifier)>,
        ListOfInterfaceIdentifiers,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RefDeclaration {
    pub nodes: (Keyword, VariablePortType, ListOfVariableIdentifiers),
}
