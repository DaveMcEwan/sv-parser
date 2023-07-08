
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LetDeclaration {
    pub nodes: (
        Keyword,
        LetIdentifier,
        Option<Paren<Option<LetPortList>>>,
        Symbol,
        Expression,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LetIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LetPortList {
    pub nodes: (List<Symbol, LetPortItem>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LetPortItem {
    pub nodes: (
        Vec<AttributeInstance>,
        LetFormalType,
        FormalPortIdentifier,
        Vec<VariableDimension>,
        Option<(Symbol, Expression)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum LetFormalType {
    DataTypeOrImplicit(Box<DataTypeOrImplicit>),
    Untyped(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LetExpression {
    pub nodes: (
        Option<PackageScope>,
        LetIdentifier,
        Option<Paren<Option<LetListOfArguments>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum LetListOfArguments {
    Ordered(Box<LetListOfArgumentsOrdered>),
    Named(Box<LetListOfArgumentsNamed>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LetListOfArgumentsOrdered {
    pub nodes: (
        List<Symbol, Option<LetActualArg>>,
        Vec<(Symbol, Symbol, Identifier, Paren<Option<LetActualArg>>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LetListOfArgumentsNamed {
    pub nodes: (List<Symbol, (Symbol, Identifier, Paren<Option<LetActualArg>>)>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LetActualArg {
    pub nodes: (Expression,),
}
