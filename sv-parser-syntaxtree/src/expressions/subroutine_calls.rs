
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantFunctionCall {
    pub nodes: (FunctionSubroutineCall,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TfCall {
    pub nodes: (
        PsOrHierarchicalTfIdentifier,
        Vec<AttributeInstance>,
        Option<Paren<ListOfArguments>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum SystemTfCall {
    ArgOptionl(Box<SystemTfCallArgOptional>),
    ArgDataType(Box<SystemTfCallArgDataType>),
    ArgExpression(Box<SystemTfCallArgExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SystemTfCallArgOptional {
    pub nodes: (SystemTfIdentifier, Option<Paren<ListOfArguments>>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SystemTfCallArgDataType {
    pub nodes: (
        SystemTfIdentifier,
        Paren<(DataType, Option<(Symbol, Expression)>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SystemTfCallArgExpression {
    pub nodes: (
        SystemTfIdentifier,
        Paren<(
            List<Symbol, Option<Expression>>,
            Option<(Symbol, Option<ClockingEvent>)>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum SubroutineCall {
    TfCall(Box<TfCall>),
    SystemTfCall(Box<SystemTfCall>),
    MethodCall(Box<MethodCall>),
    Randomize(Box<SubroutineCallRandomize>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SubroutineCallRandomize {
    pub nodes: (Option<(Keyword, Symbol)>, RandomizeCall),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FunctionSubroutineCall {
    pub nodes: (SubroutineCall,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ListOfArguments {
    Ordered(Box<ListOfArgumentsOrdered>),
    Named(Box<ListOfArgumentsNamed>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfArgumentsOrdered {
    pub nodes: (
        List<Symbol, Option<Expression>>,
        Vec<(Symbol, Symbol, Identifier, Paren<Option<Expression>>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ListOfArgumentsNamed {
    pub nodes: (
        Symbol,
        Identifier,
        Paren<Option<Expression>>,
        Vec<(Symbol, Symbol, Identifier, Paren<Option<Expression>>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct MethodCall {
    pub nodes: (MethodCallRoot, Symbol, MethodCallBody),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum MethodCallBody {
    User(Box<MethodCallBodyUser>),
    BuiltInMethodCall(Box<BuiltInMethodCall>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct MethodCallBodyUser {
    pub nodes: (
        MethodIdentifier,
        Vec<AttributeInstance>,
        Option<Paren<ListOfArguments>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum BuiltInMethodCall {
    ArrayManipulationCall(Box<ArrayManipulationCall>),
    RandomizeCall(Box<RandomizeCall>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ArrayManipulationCall {
    pub nodes: (
        ArrayMethodName,
        Vec<AttributeInstance>,
        Option<Paren<ListOfArguments>>,
        Option<(Keyword, Paren<Expression>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RandomizeCall {
    pub nodes: (
        Keyword,
        Vec<AttributeInstance>,
        Option<Paren<Option<VariableIdentifierListOrNull>>>,
        Option<(
            Keyword,
            Option<Paren<Option<IdentifierList>>>,
            ConstraintBlock,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum VariableIdentifierListOrNull {
    VariableIdentifierList(Box<VariableIdentifierList>),
    Null(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum MethodCallRoot {
    Primary(Box<Primary>),
    ImplicitClassHandle(Box<ImplicitClassHandle>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ArrayMethodName {
    MethodIdentifier(Box<MethodIdentifier>),
    Unique(Box<Keyword>),
    And(Box<Keyword>),
    Or(Box<Keyword>),
    Xor(Box<Keyword>),
}
