
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum AssertionItem {
    Concurrent(Box<ConcurrentAssertionItem>),
    Immediate(Box<DeferredImmediateAssertionItem>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DeferredImmediateAssertionItem {
    pub nodes: (
        Option<(BlockIdentifier, Symbol)>,
        DeferredImmediateAssertionStatement,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ProceduralAssertionStatement {
    Concurrent(Box<ConcurrentAssertionStatement>),
    Immediate(Box<ImmediateAssertionStatement>),
    Checker(Box<CheckerInstantiation>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ImmediateAssertionStatement {
    Simple(Box<SimpleImmediateAssertionStatement>),
    Deferred(Box<DeferredImmediateAssertionStatement>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum SimpleImmediateAssertionStatement {
    Assert(Box<SimpleImmediateAssertStatement>),
    Assume(Box<SimpleImmediateAssumeStatement>),
    Cover(Box<SimpleImmediateCoverStatement>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SimpleImmediateAssertStatement {
    pub nodes: (Keyword, Paren<Expression>, ActionBlock),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SimpleImmediateAssumeStatement {
    pub nodes: (Keyword, Paren<Expression>, ActionBlock),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SimpleImmediateCoverStatement {
    pub nodes: (Keyword, Paren<Expression>, StatementOrNull),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DeferredImmediateAssertionStatement {
    Assert(Box<DeferredImmediateAssertStatement>),
    Assume(Box<DeferredImmediateAssumeStatement>),
    Cover(Box<DeferredImmediateCoverStatement>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DeferredImmediateAssertStatement {
    pub nodes: (Keyword, AssertTiming, Paren<Expression>, ActionBlock),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DeferredImmediateAssumeStatement {
    pub nodes: (Keyword, AssertTiming, Paren<Expression>, ActionBlock),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DeferredImmediateCoverStatement {
    pub nodes: (Keyword, AssertTiming, Paren<Expression>, StatementOrNull),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum AssertTiming {
    Zero(Box<Symbol>),
    Final(Box<Keyword>),
}
