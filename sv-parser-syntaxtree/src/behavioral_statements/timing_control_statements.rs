
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProceduralTimingControlStatement {
    pub nodes: (ProceduralTimingControl, StatementOrNull),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DelayOrEventControl {
    Delay(Box<DelayControl>),
    Event(Box<EventControl>),
    Repeat(Box<DelayOrEventControlRepeat>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DelayOrEventControlRepeat {
    pub nodes: (Keyword, Paren<Expression>, EventControl),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DelayControl {
    Delay(Box<DelayControlDelay>),
    Mintypmax(Box<DelayControlMintypmax>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DelayControlDelay {
    pub nodes: (Symbol, DelayValue),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DelayControlMintypmax {
    pub nodes: (Symbol, Paren<MintypmaxExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum EventControl {
    EventIdentifier(Box<EventControlEventIdentifier>),
    EventExpression(Box<EventControlEventExpression>),
    Asterisk(Box<EventControlAsterisk>),
    ParenAsterisk(Box<EventControlParenAsterisk>),
    SequenceIdentifier(Box<EventControlSequenceIdentifier>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventControlEventIdentifier {
    pub nodes: (Symbol, HierarchicalEventIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventControlEventExpression {
    pub nodes: (Symbol, Paren<EventExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventControlAsterisk {
    pub nodes: (Symbol,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventControlParenAsterisk {
    pub nodes: (Symbol, Paren<Symbol>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventControlSequenceIdentifier {
    pub nodes: (Symbol, PsOrHierarchicalSequenceIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum EventExpression {
    Expression(Box<EventExpressionExpression>),
    Sequence(Box<EventExpressionSequence>),
    Or(Box<EventExpressionOr>),
    Comma(Box<EventExpressionComma>),
    Paren(Box<EventExpressionParen>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventExpressionExpression {
    pub nodes: (
        Option<EdgeIdentifier>,
        Expression,
        Option<(Keyword, Expression)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventExpressionSequence {
    pub nodes: (SequenceInstance, Option<(Keyword, Expression)>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventExpressionOr {
    pub nodes: (EventExpression, Keyword, EventExpression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventExpressionComma {
    pub nodes: (EventExpression, Symbol, EventExpression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventExpressionParen {
    pub nodes: (Paren<EventExpression>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ProceduralTimingControl {
    DelayControl(Box<DelayControl>),
    EventControl(Box<EventControl>),
    CycleDelay(Box<CycleDelay>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum JumpStatement {
    Return(Box<JumpStatementReturn>),
    Break(Box<JumpStatementBreak>),
    Continue(Box<JumpStatementContinue>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct JumpStatementReturn {
    pub nodes: (Keyword, Option<Expression>, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct JumpStatementBreak {
    pub nodes: (Keyword, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct JumpStatementContinue {
    pub nodes: (Keyword, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum WaitStatement {
    Wait(Box<WaitStatementWait>),
    Fork(Box<WaitStatementFork>),
    Order(Box<WaitStatementOrder>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct WaitStatementWait {
    pub nodes: (Keyword, Paren<Expression>, StatementOrNull),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct WaitStatementFork {
    pub nodes: (Keyword, Keyword, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct WaitStatementOrder {
    pub nodes: (
        Keyword,
        Paren<List<Symbol, HierarchicalIdentifier>>,
        ActionBlock,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum EventTrigger {
    Named(Box<EventTriggerNamed>),
    Nonblocking(Box<EventTriggerNonblocking>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventTriggerNamed {
    pub nodes: (Symbol, HierarchicalEventIdentifier, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventTriggerNonblocking {
    pub nodes: (
        Symbol,
        Option<DelayOrEventControl>,
        HierarchicalEventIdentifier,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DisableStatement {
    Task(Box<DisableStatementTask>),
    Block(Box<DisableStatementBlock>),
    Fork(Box<DisableStatementFork>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DisableStatementTask {
    pub nodes: (Keyword, HierarchicalTaskIdentifier, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DisableStatementBlock {
    pub nodes: (Keyword, HierarchicalBlockIdentifier, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DisableStatementFork {
    pub nodes: (Keyword, Keyword, Symbol),
}
