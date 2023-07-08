
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SpecifyInputTerminalDescriptor {
    pub nodes: (InputIdentifier, Option<Bracket<ConstantRangeExpression>>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SpecifyOutputTerminalDescriptor {
    pub nodes: (OutputIdentifier, Option<Bracket<ConstantRangeExpression>>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum InputIdentifier {
    InputPortIdentifier(Box<InputPortIdentifier>),
    InoutPortIdentifier(Box<InoutPortIdentifier>),
    Interface(Box<InputIdentifierInterface>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InputIdentifierInterface {
    pub nodes: (InterfaceIdentifier, Symbol, PortIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum OutputIdentifier {
    OutputPortIdentifier(Box<OutputPortIdentifier>),
    InoutPortIdentifier(Box<InoutPortIdentifier>),
    Interface(Box<OutputIdentifierInterface>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct OutputIdentifierInterface {
    pub nodes: (InterfaceIdentifier, Symbol, PortIdentifier),
}
