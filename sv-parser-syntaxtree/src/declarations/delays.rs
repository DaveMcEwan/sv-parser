
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum Delay3 {
    Single(Box<Delay3Single>),
    Mintypmax(Box<Delay3Mintypmax>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Delay3Single {
    pub nodes: (Symbol, DelayValue),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Delay3Mintypmax {
    pub nodes: (
        Symbol,
        Paren<(
            MintypmaxExpression,
            Option<(
                Symbol,
                MintypmaxExpression,
                Option<(Symbol, MintypmaxExpression)>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum Delay2 {
    Single(Box<Delay2Single>),
    Mintypmax(Box<Delay2Mintypmax>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Delay2Single {
    pub nodes: (Symbol, DelayValue),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Delay2Mintypmax {
    pub nodes: (
        Symbol,
        Paren<(MintypmaxExpression, Option<(Symbol, MintypmaxExpression)>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DelayValue {
    UnsignedNumber(Box<UnsignedNumber>),
    RealNumber(Box<RealNumber>),
    PsIdentifier(Box<PsIdentifier>),
    HierarchicalIdentifier(Box<HierarchicalIdentifier>),
    TimeLiteral(Box<TimeLiteral>),
    Step1(Box<Keyword>),
}
