
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CmosSwitchtype {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EnableGatetype {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct MosSwitchtype {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NInputGatetype {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NOutputGatetype {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PassEnSwitchtype {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PassSwitchtype {
    pub nodes: (Keyword,),
}
