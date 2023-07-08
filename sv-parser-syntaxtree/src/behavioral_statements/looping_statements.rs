
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum LoopStatement {
    Forever(Box<LoopStatementForever>),
    Repeat(Box<LoopStatementRepeat>),
    While(Box<LoopStatementWhile>),
    For(Box<LoopStatementFor>),
    DoWhile(Box<LoopStatementDoWhile>),
    Foreach(Box<LoopStatementForeach>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LoopStatementForever {
    pub nodes: (Keyword, StatementOrNull),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LoopStatementRepeat {
    pub nodes: (Keyword, Paren<Expression>, StatementOrNull),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LoopStatementWhile {
    pub nodes: (Keyword, Paren<Expression>, StatementOrNull),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LoopStatementFor {
    pub nodes: (
        Keyword,
        Paren<(
            Option<ForInitialization>,
            Symbol,
            Option<Expression>,
            Symbol,
            Option<ForStep>,
        )>,
        StatementOrNull,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LoopStatementDoWhile {
    pub nodes: (Keyword, StatementOrNull, Keyword, Paren<Expression>, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LoopStatementForeach {
    pub nodes: (
        Keyword,
        Paren<(PsOrHierarchicalArrayIdentifier, Bracket<LoopVariables>)>,
        Statement,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ForInitialization {
    ListOfVariableAssignments(Box<ListOfVariableAssignments>),
    Declaration(Box<ForInitializationDeclaration>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ForInitializationDeclaration {
    pub nodes: (List<Symbol, ForVariableDeclaration>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ForVariableDeclaration {
    pub nodes: (
        Option<Var>,
        DataType,
        List<Symbol, (VariableIdentifier, Symbol, Expression)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Var {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ForStep {
    pub nodes: (List<Symbol, ForStepAssignment>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ForStepAssignment {
    OperatorAssignment(Box<OperatorAssignment>),
    IncOrDecExpression(Box<IncOrDecExpression>),
    FunctionSubroutineCall(Box<FunctionSubroutineCall>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LoopVariables {
    pub nodes: (List<Symbol, Option<IndexVariableIdentifier>>,),
}
