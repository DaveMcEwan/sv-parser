
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CheckerInstantiation {
    pub nodes: (
        PsCheckerIdentifier,
        NameOfInstance,
        Paren<Option<ListOfCheckerPortConnections>>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ListOfCheckerPortConnections {
    Ordered(Box<ListOfCheckerPortConnectionsOrdered>),
    Named(Box<ListOfCheckerPortConnectionsNamed>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfCheckerPortConnectionsOrdered {
    pub nodes: (List<Symbol, OrderedCheckerPortConnection>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfCheckerPortConnectionsNamed {
    pub nodes: (List<Symbol, NamedCheckerPortConnection>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct OrderedCheckerPortConnection {
    pub nodes: (Vec<AttributeInstance>, Option<PropertyActualArg>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum NamedCheckerPortConnection {
    Identifier(Box<NamedCheckerPortConnectionIdentifier>),
    Asterisk(Box<NamedCheckerPortConnectionAsterisk>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NamedCheckerPortConnectionIdentifier {
    pub nodes: (
        Vec<AttributeInstance>,
        Symbol,
        FormalPortIdentifier,
        Option<Paren<Option<PropertyActualArg>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NamedCheckerPortConnectionAsterisk {
    pub nodes: (Vec<AttributeInstance>, Symbol),
}
