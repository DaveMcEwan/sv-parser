
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportDeclaration {
    pub nodes: (Keyword, List<Symbol, ModportItem>, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportItem {
    pub nodes: (
        ModportIdentifier,
        Paren<List<Symbol, ModportPortsDeclaration>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ModportPortsDeclaration {
    Simple(Box<ModportPortsDeclarationSimple>),
    Tf(Box<ModportPortsDeclarationTf>),
    Clocking(Box<ModportPortsDeclarationClocking>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportPortsDeclarationSimple {
    pub nodes: (Vec<AttributeInstance>, ModportSimplePortsDeclaration),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportPortsDeclarationTf {
    pub nodes: (Vec<AttributeInstance>, ModportTfPortsDeclaration),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportPortsDeclarationClocking {
    pub nodes: (Vec<AttributeInstance>, ModportClockingDeclaration),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportClockingDeclaration {
    pub nodes: (Keyword, ClockingIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportSimplePortsDeclaration {
    pub nodes: (PortDirection, List<Symbol, ModportSimplePort>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ModportSimplePort {
    Ordered(Box<ModportSimplePortOrdered>),
    Named(Box<ModportSimplePortNamed>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportSimplePortOrdered {
    pub nodes: (PortIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportSimplePortNamed {
    pub nodes: (Symbol, PortIdentifier, Paren<Option<Expression>>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportTfPortsDeclaration {
    pub nodes: (ImportExport, List<Symbol, ModportTfPort>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ModportTfPort {
    MethodPrototype(Box<MethodPrototype>),
    TfIdentifier(Box<TfIdentifier>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ImportExport {
    Import(Box<Keyword>),
    Export(Box<Keyword>),
}
