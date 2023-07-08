
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RandsequenceStatement {
    pub nodes: (
        Keyword,
        Paren<Option<ProductionIdentifier>>,
        Production,
        Vec<Production>,
        Keyword,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Production {
    pub nodes: (
        Option<DataTypeOrVoid>,
        ProductionIdentifier,
        Option<Paren<TfPortList>>,
        Symbol,
        List<Symbol, RsRule>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RsRule {
    pub nodes: (
        RsProductionList,
        Option<(Symbol, WeightSpecification, Option<RsCodeBlock>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum RsProductionList {
    Prod(Box<RsProductionListProd>),
    Join(Box<RsProductionListJoin>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RsProductionListProd {
    pub nodes: (RsProd, Vec<RsProd>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RsProductionListJoin {
    pub nodes: (
        Keyword,
        Keyword,
        Option<Paren<Expression>>,
        ProductionItem,
        ProductionItem,
        Vec<ProductionItem>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum WeightSpecification {
    IntegralNumber(Box<IntegralNumber>),
    PsIdentifier(Box<PsIdentifier>),
    Expression(Box<WeightSpecificationExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct WeightSpecificationExpression {
    pub nodes: (Paren<Expression>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RsCodeBlock {
    pub nodes: (Brace<(Vec<DataDeclaration>, Vec<StatementOrNull>)>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum RsProd {
    ProductionItem(Box<ProductionItem>),
    RsCodeBlock(Box<RsCodeBlock>),
    RsIfElse(Box<RsIfElse>),
    RsRepeat(Box<RsRepeat>),
    RsCase(Box<RsCase>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProductionItem {
    pub nodes: (ProductionIdentifier, Option<Paren<ListOfArguments>>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RsIfElse {
    pub nodes: (
        Keyword,
        Paren<Expression>,
        ProductionItem,
        Option<(Keyword, ProductionItem)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RsRepeat {
    pub nodes: (Keyword, Paren<Expression>, ProductionItem),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RsCase {
    pub nodes: (
        Keyword,
        Paren<CaseExpression>,
        RsCaseItem,
        Vec<RsCaseItem>,
        Keyword,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum RsCaseItem {
    NonDefault(Box<RsCaseItemNondefault>),
    Default(Box<RsCaseItemDefault>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RsCaseItemNondefault {
    pub nodes: (
        List<Symbol, CaseItemExpression>,
        Symbol,
        ProductionItem,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct RsCaseItemDefault {
    pub nodes: (Keyword, Option<Symbol>, ProductionItem, Symbol),
}
