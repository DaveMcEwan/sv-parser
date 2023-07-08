
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TimecheckCondition {
    pub nodes: (MintypmaxExpression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ControlledReferenceEvent {
    pub nodes: (ControlledTimingCheckEvent,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DataEvent {
    pub nodes: (TimingCheckEvent,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DelayedData {
    TerminalIdentifier(Box<TerminalIdentifier>),
    WithMintypmax(Box<DelayedDataWithMintypmax>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DelayedDataWithMintypmax {
    pub nodes: (TerminalIdentifier, Bracket<ConstantMintypmaxExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DelayedReference {
    TerminalIdentifier(Box<TerminalIdentifier>),
    WithMintypmax(Box<DelayedReferenceWithMintypmax>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DelayedReferenceWithMintypmax {
    pub nodes: (TerminalIdentifier, Bracket<ConstantMintypmaxExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EndEdgeOffset {
    pub nodes: (MintypmaxExpression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EventBasedFlag {
    pub nodes: (ConstantExpression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Notifier {
    pub nodes: (VariableIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ReferenceEvent {
    pub nodes: (TimingCheckEvent,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RemainActiveFlag {
    pub nodes: (ConstantMintypmaxExpression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TimestampCondition {
    pub nodes: (MintypmaxExpression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct StartEdgeOffset {
    pub nodes: (MintypmaxExpression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Threshold {
    pub nodes: (ConstantExpression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TimingCheckLimit {
    pub nodes: (Expression,),
}
