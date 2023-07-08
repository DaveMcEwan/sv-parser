
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct UdpPortList {
    pub nodes: (
        OutputPortIdentifier,
        Symbol,
        List<Symbol, InputPortIdentifier>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct UdpDeclarationPortList {
    pub nodes: (
        UdpOutputDeclaration,
        Symbol,
        List<Symbol, UdpInputDeclaration>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum UdpPortDeclaration {
    UdpOutputDeclaration(Box<(UdpOutputDeclaration, Symbol)>),
    UdpInputDeclaration(Box<(UdpInputDeclaration, Symbol)>),
    UdpRegDeclaration(Box<(UdpRegDeclaration, Symbol)>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum UdpOutputDeclaration {
    Nonreg(Box<UdpOutputDeclarationNonreg>),
    Reg(Box<UdpOutputDeclarationReg>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct UdpOutputDeclarationNonreg {
    pub nodes: (Vec<AttributeInstance>, Keyword, PortIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct UdpOutputDeclarationReg {
    pub nodes: (
        Vec<AttributeInstance>,
        Keyword,
        Keyword,
        PortIdentifier,
        Option<(Symbol, ConstantExpression)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct UdpInputDeclaration {
    pub nodes: (Vec<AttributeInstance>, Keyword, ListOfUdpPortIdentifiers),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct UdpRegDeclaration {
    pub nodes: (Vec<AttributeInstance>, Keyword, VariableIdentifier),
}
