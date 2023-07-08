
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum GateInstantiation {
    Cmos(Box<GateInstantiationCmos>),
    Enable(Box<GateInstantiationEnable>),
    Mos(Box<GateInstantiationMos>),
    NInput(Box<GateInstantiationNInput>),
    NOutput(Box<GateInstantiationNOutput>),
    PassEn(Box<GateInstantiationPassEn>),
    Pass(Box<GateInstantiationPass>),
    Pulldown(Box<GateInstantiationPulldown>),
    Pullup(Box<GateInstantiationPullup>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GateInstantiationCmos {
    pub nodes: (
        CmosSwitchtype,
        Option<Delay3>,
        List<Symbol, CmosSwitchInstance>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GateInstantiationEnable {
    pub nodes: (
        EnableGatetype,
        Option<DriveStrength>,
        Option<Delay3>,
        List<Symbol, EnableGateInstance>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GateInstantiationMos {
    pub nodes: (
        MosSwitchtype,
        Option<Delay3>,
        List<Symbol, MosSwitchInstance>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GateInstantiationNInput {
    pub nodes: (
        NInputGatetype,
        Option<DriveStrength>,
        Option<Delay2>,
        List<Symbol, NInputGateInstance>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GateInstantiationNOutput {
    pub nodes: (
        NOutputGatetype,
        Option<DriveStrength>,
        Option<Delay2>,
        List<Symbol, NOutputGateInstance>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GateInstantiationPassEn {
    pub nodes: (
        PassEnSwitchtype,
        Option<Delay2>,
        List<Symbol, PassEnableSwitchInstance>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GateInstantiationPass {
    pub nodes: (PassSwitchtype, List<Symbol, PassSwitchInstance>, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GateInstantiationPulldown {
    pub nodes: (
        Keyword,
        Option<PulldownStrength>,
        List<Symbol, PullGateInstance>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GateInstantiationPullup {
    pub nodes: (
        Keyword,
        Option<PullupStrength>,
        List<Symbol, PullGateInstance>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CmosSwitchInstance {
    pub nodes: (
        Option<NameOfInstance>,
        Paren<(
            OutputTerminal,
            Symbol,
            InputTerminal,
            Symbol,
            NcontrolTerminal,
            Symbol,
            PcontrolTerminal,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EnableGateInstance {
    pub nodes: (
        Option<NameOfInstance>,
        Paren<(
            OutputTerminal,
            Symbol,
            InputTerminal,
            Symbol,
            EnableTerminal,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct MosSwitchInstance {
    pub nodes: (
        Option<NameOfInstance>,
        Paren<(
            OutputTerminal,
            Symbol,
            InputTerminal,
            Symbol,
            EnableTerminal,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NInputGateInstance {
    pub nodes: (
        Option<NameOfInstance>,
        Paren<(OutputTerminal, Symbol, List<Symbol, InputTerminal>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NOutputGateInstance {
    pub nodes: (
        Option<NameOfInstance>,
        Paren<(List<Symbol, OutputTerminal>, Symbol, InputTerminal)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PassSwitchInstance {
    pub nodes: (
        Option<NameOfInstance>,
        Paren<(InoutTerminal, Symbol, InoutTerminal)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PassEnableSwitchInstance {
    pub nodes: (
        Option<NameOfInstance>,
        Paren<(InoutTerminal, Symbol, InoutTerminal, Symbol, EnableTerminal)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PullGateInstance {
    pub nodes: (Option<NameOfInstance>, Paren<OutputTerminal>),
}
