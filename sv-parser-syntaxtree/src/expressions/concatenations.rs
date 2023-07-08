
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Concatenation {
    pub nodes: (Brace<List<Symbol, Expression>>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantConcatenation {
    pub nodes: (Brace<List<Symbol, ConstantExpression>>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantMultipleConcatenation {
    pub nodes: (Brace<(ConstantExpression, ConstantConcatenation)>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModulePathConcatenation {
    pub nodes: (Brace<List<Symbol, ModulePathExpression>>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModulePathMultipleConcatenation {
    pub nodes: (Brace<(ConstantExpression, ModulePathConcatenation)>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct MultipleConcatenation {
    pub nodes: (Brace<(Expression, Concatenation)>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct StreamingConcatenation {
    pub nodes: (Brace<(StreamOperator, Option<SliceSize>, StreamConcatenation)>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct StreamOperator {
    pub nodes: (Symbol,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum SliceSize {
    SimpleType(Box<SimpleType>),
    ConstantExpression(Box<ConstantExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct StreamConcatenation {
    pub nodes: (Brace<List<Symbol, StreamExpression>>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct StreamExpression {
    pub nodes: (Expression, Option<(Keyword, Bracket<ArrayRangeExpression>)>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ArrayRangeExpression {
    Expression(Box<Expression>),
    Colon(Box<ArrayRangeExpressionColon>),
    PlusColon(Box<ArrayRangeExpressionPlusColon>),
    MinusColon(Box<ArrayRangeExpressionMinusColon>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ArrayRangeExpressionColon {
    pub nodes: (Expression, Symbol, Expression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ArrayRangeExpressionPlusColon {
    pub nodes: (Expression, Symbol, Expression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ArrayRangeExpressionMinusColon {
    pub nodes: (Expression, Symbol, Expression),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EmptyUnpackedArrayConcatenation {
    pub nodes: (Symbol, Symbol),
}
