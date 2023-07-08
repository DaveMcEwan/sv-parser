
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ConstantPrimary {
    PrimaryLiteral(Box<PrimaryLiteral>),
    PsParameter(Box<ConstantPrimaryPsParameter>),
    Specparam(Box<ConstantPrimarySpecparam>),
    GenvarIdentifier(Box<GenvarIdentifier>),
    FormalPort(Box<ConstantPrimaryFormalPort>),
    Enum(Box<ConstantPrimaryEnum>),
    Concatenation(Box<ConstantPrimaryConcatenation>),
    MultipleConcatenation(Box<ConstantPrimaryMultipleConcatenation>),
    ConstantFunctionCall(Box<ConstantFunctionCall>),
    ConstantLetExpression(Box<ConstantLetExpression>),
    MintypmaxExpression(Box<ConstantPrimaryMintypmaxExpression>),
    ConstantCast(Box<ConstantCast>),
    ConstantAssignmentPatternExpression(Box<ConstantAssignmentPatternExpression>),
    TypeReference(Box<TypeReference>),
    Null(Box<Keyword>),
    Dollar(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantPrimaryPsParameter {
    pub nodes: (PsParameterIdentifier, ConstantSelect),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantPrimarySpecparam {
    pub nodes: (
        SpecparamIdentifier,
        Option<Bracket<ConstantRangeExpression>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantPrimaryFormalPort {
    pub nodes: (FormalPortIdentifier, ConstantSelect),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantPrimaryEnum {
    pub nodes: (PackageScopeOrClassScope, EnumIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantPrimaryConcatenation {
    pub nodes: (
        ConstantConcatenation,
        Option<Bracket<ConstantRangeExpression>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantPrimaryMultipleConcatenation {
    pub nodes: (
        ConstantMultipleConcatenation,
        Option<Bracket<ConstantRangeExpression>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantPrimaryMintypmaxExpression {
    pub nodes: (Paren<ConstantMintypmaxExpression>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ModulePathPrimary {
    Number(Box<Number>),
    Identifier(Box<Identifier>),
    ModulePathConcatenation(Box<ModulePathConcatenation>),
    ModulePathMultipleConcatenation(Box<ModulePathMultipleConcatenation>),
    FunctionSubroutineCall(Box<FunctionSubroutineCall>),
    Mintypmax(Box<ModulePathPrimaryMintypmax>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModulePathPrimaryMintypmax {
    pub nodes: (Paren<ModulePathMintypmaxExpression>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum Primary {
    PrimaryLiteral(Box<PrimaryLiteral>),
    Hierarchical(Box<PrimaryHierarchical>),
    EmptyUnpackedArrayConcatenation(Box<EmptyUnpackedArrayConcatenation>),
    Concatenation(Box<PrimaryConcatenation>),
    MultipleConcatenation(Box<PrimaryMultipleConcatenation>),
    FunctionSubroutineCall(Box<FunctionSubroutineCall>),
    LetExpression(Box<LetExpression>),
    MintypmaxExpression(Box<PrimaryMintypmaxExpression>),
    Cast(Box<Cast>),
    AssignmentPatternExpression(Box<AssignmentPatternExpression>),
    StreamingConcatenation(Box<StreamingConcatenation>),
    SequenceMethodCall(Box<SequenceMethodCall>),
    This(Box<Keyword>),
    Dollar(Box<Keyword>),
    Null(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PrimaryHierarchical {
    pub nodes: (
        Option<ClassQualifierOrPackageScope>,
        HierarchicalIdentifier,
        Select,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PrimaryConcatenation {
    pub nodes: (Concatenation, Option<Bracket<RangeExpression>>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PrimaryMultipleConcatenation {
    pub nodes: (MultipleConcatenation, Option<Bracket<RangeExpression>>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PrimaryMintypmaxExpression {
    pub nodes: (Paren<MintypmaxExpression>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ClassQualifierOrPackageScope {
    ClassQualifier(Box<ClassQualifier>),
    PackageScope(Box<PackageScope>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClassQualifier {
    pub nodes: (Option<Local>, Option<ImplicitClassHandleOrClassScope>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum RangeExpression {
    Expression(Box<Expression>),
    PartSelectRange(Box<PartSelectRange>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PrimaryLiteral {
    Number(Box<Number>),
    TimeLiteral(Box<TimeLiteral>),
    UnbasedUnsizedLiteral(Box<UnbasedUnsizedLiteral>),
    StringLiteral(Box<StringLiteral>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum TimeLiteral {
    Unsigned(Box<TimeLiteralUnsigned>),
    FixedPoint(Box<TimeLiteralFixedPoint>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TimeLiteralUnsigned {
    pub nodes: (UnsignedNumber, TimeUnit),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TimeLiteralFixedPoint {
    pub nodes: (FixedPointNumber, TimeUnit),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum TimeUnit {
    S(Box<Keyword>),
    MS(Box<Keyword>),
    US(Box<Keyword>),
    NS(Box<Keyword>),
    PS(Box<Keyword>),
    FS(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ImplicitClassHandle {
    This(Box<Keyword>),
    Super(Box<Keyword>),
    ThisSuper(Box<(Keyword, Symbol, Keyword)>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct BitSelect {
    pub nodes: (Vec<Bracket<Expression>>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Select {
    pub nodes: (
        Option<(
            Vec<(Symbol, MemberIdentifier, BitSelect)>,
            Symbol,
            MemberIdentifier,
        )>,
        BitSelect,
        Option<Bracket<PartSelectRange>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NonrangeSelect {
    pub nodes: (
        Option<(
            Vec<(Symbol, MemberIdentifier, BitSelect)>,
            Symbol,
            MemberIdentifier,
        )>,
        BitSelect,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantBitSelect {
    pub nodes: (Vec<Bracket<ConstantExpression>>,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantSelect {
    pub nodes: (
        Option<(
            Vec<(Symbol, MemberIdentifier, ConstantBitSelect)>,
            Symbol,
            MemberIdentifier,
        )>,
        ConstantBitSelect,
        Option<Bracket<ConstantPartSelectRange>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantCast {
    pub nodes: (CastingType, Symbol, Paren<ConstantExpression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstantLetExpression {
    pub nodes: (LetExpression,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Cast {
    pub nodes: (CastingType, Symbol, Paren<Expression>),
}
