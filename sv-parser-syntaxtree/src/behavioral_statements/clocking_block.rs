
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ClockingDeclaration {
    Local(Box<ClockingDeclarationLocal>),
    Global(Box<ClockingDeclarationGlobal>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingDeclarationLocal {
    pub nodes: (
        Option<Default>,
        Keyword,
        Option<ClockingIdentifier>,
        ClockingEvent,
        Symbol,
        Vec<ClockingItem>,
        Keyword,
        Option<(Symbol, ClockingIdentifier)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Default {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingDeclarationGlobal {
    pub nodes: (
        Keyword,
        Keyword,
        Option<ClockingIdentifier>,
        ClockingEvent,
        Symbol,
        Keyword,
        Option<(Symbol, ClockingIdentifier)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ClockingEvent {
    Identifier(Box<ClockingEventIdentifier>),
    Expression(Box<ClockingEventExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingEventIdentifier {
    pub nodes: (Symbol, Identifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingEventExpression {
    pub nodes: (Symbol, Paren<EventExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ClockingItem {
    Default(Box<ClockingItemDefault>),
    Direction(Box<ClockingItemDirection>),
    Assertion(Box<ClockingItemAssertion>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingItemDefault {
    pub nodes: (Keyword, DefaultSkew, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingItemDirection {
    pub nodes: (ClockingDirection, ListOfClockingDeclAssign, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingItemAssertion {
    pub nodes: (Vec<AttributeInstance>, AssertionItemDeclaration),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DefaultSkew {
    Input(Box<DefaultSkewInput>),
    Output(Box<DefaultSkewOutput>),
    InputOutput(Box<DefaultSkewInputOutput>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DefaultSkewInput {
    pub nodes: (Keyword, ClockingSkew),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DefaultSkewOutput {
    pub nodes: (Keyword, ClockingSkew),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DefaultSkewInputOutput {
    pub nodes: (Keyword, ClockingSkew, Keyword, ClockingSkew),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ClockingDirection {
    Input(Box<ClockingDirectionInput>),
    Output(Box<ClockingDirectionOutput>),
    InputOutput(Box<ClockingDirectionInputOutput>),
    Inout(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingDirectionInput {
    pub nodes: (Keyword, Option<ClockingSkew>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingDirectionOutput {
    pub nodes: (Keyword, Option<ClockingSkew>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingDirectionInputOutput {
    pub nodes: (Keyword, Option<ClockingSkew>, Keyword, Option<ClockingSkew>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfClockingDeclAssign {
    pub nodes: (List<Symbol, ClockingDeclAssign>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingDeclAssign {
    pub nodes: (SignalIdentifier, Option<(Symbol, Expression)>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ClockingSkew {
    Edge(Box<ClockingSkewEdge>),
    DelayControl(Box<DelayControl>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingSkewEdge {
    pub nodes: (EdgeIdentifier, Option<DelayControl>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingDrive {
    pub nodes: (ClockvarExpression, Symbol, Option<CycleDelay>, Expression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum CycleDelay {
    Integral(Box<CycleDelayIntegral>),
    Identifier(Box<CycleDelayIdentifier>),
    Expression(Box<CycleDelayExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CycleDelayIntegral {
    pub nodes: (Symbol, IntegralNumber),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CycleDelayIdentifier {
    pub nodes: (Symbol, Identifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CycleDelayExpression {
    pub nodes: (Symbol, Paren<Expression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Clockvar {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockvarExpression {
    pub nodes: (Clockvar, Select),
}
