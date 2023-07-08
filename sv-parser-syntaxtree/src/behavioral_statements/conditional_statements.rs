
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConditionalStatement {
    pub nodes: (
        Option<UniquePriority>,
        Keyword,
        Paren<CondPredicate>,
        StatementOrNull,
        Vec<(Keyword, Keyword, Paren<CondPredicate>, StatementOrNull)>,
        Option<(Keyword, StatementOrNull)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum UniquePriority {
    Unique(Box<Keyword>),
    Unique0(Box<Keyword>),
    Priority(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CondPredicate {
    pub nodes: (List<Symbol, ExpressionOrCondPattern>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ExpressionOrCondPattern {
    Expression(Box<Expression>),
    CondPattern(Box<CondPattern>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CondPattern {
    pub nodes: (Expression, Keyword, Pattern),
}
