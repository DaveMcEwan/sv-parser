use crate::parser::*;
//use nom::branch::*;
//use nom::combinator::*;
use nom::error::*;
use nom::{Err, IResult};

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct DefparamAssignment<'a> {
    pub nodes: (HierarchicalIdentifier<'a>, ConstantMintypmaxExpression<'a>),
}

#[derive(Debug)]
pub struct NetDeclAssignment<'a> {
    pub nodes: (Identifier<'a>, Vec<UnpackedDimension<'a>>, Expression<'a>),
}

#[derive(Debug)]
pub struct ParamAssignment<'a> {
    pub nodes: (
        Identifier<'a>,
        Vec<UnpackedDimension<'a>>,
        ConstantParamExpression<'a>,
    ),
}

#[derive(Debug)]
pub enum SpecparamAssignment<'a> {
    Mintypmax(SpecparamAssignmentMintypmax<'a>),
    PulseControl(PulseControlSpecparam<'a>),
}

#[derive(Debug)]
pub struct SpecparamAssignmentMintypmax<'a> {
    pub nodes: (Identifier<'a>, ConstantMintypmaxExpression<'a>),
}

#[derive(Debug)]
pub struct TypeAssignment<'a> {
    pub nodes: (Identifier<'a>, Option<DataType<'a>>),
}

#[derive(Debug)]
pub struct PulseControlSpecparam<'a> {
    pub nodes: (
        Option<(
            SpecifyInputTerminalDescriptor<'a>,
            SpecifyOutputTerminalDescriptor<'a>,
        )>,
        RejectLimitValue<'a>,
        Option<ErrorLimitValue<'a>>,
    ),
}

#[derive(Debug)]
pub struct ErrorLimitValue<'a> {
    pub nodes: (LimitValue<'a>,),
}

#[derive(Debug)]
pub struct RejectLimitValue<'a> {
    pub nodes: (LimitValue<'a>,),
}

#[derive(Debug)]
pub struct LimitValue<'a> {
    pub nodes: (ConstantMintypmaxExpression<'a>,),
}

#[derive(Debug)]
pub enum VariableDeclAssignment<'a> {
    Variable(VariableDeclAssignmentVariable<'a>),
    DynamicArray(VariableDeclAssignmentDynamicArray<'a>),
    Class(VariableDeclAssignmentClass<'a>),
}

#[derive(Debug)]
pub struct VariableDeclAssignmentVariable<'a> {
    pub nodes: (
        Identifier<'a>,
        Vec<VariableDimension<'a>>,
        Option<Expression<'a>>,
    ),
}

#[derive(Debug)]
pub struct VariableDeclAssignmentDynamicArray<'a> {
    pub nodes: (
        Identifier<'a>,
        UnsizedDimension,
        Vec<VariableDimension<'a>>,
        Option<DynamicArrayNew<'a>>,
    ),
}

#[derive(Debug)]
pub struct VariableDeclAssignmentClass<'a> {
    pub nodes: (Identifier<'a>, Option<ClassNew<'a>>),
}

#[derive(Debug)]
pub enum ClassNew<'a> {
    Argument(ClassNewArgument<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug)]
pub struct ClassNewArgument<'a> {
    pub nodes: (Option<ClassScope<'a>>, Option<ListOfArguments<'a>>),
}

#[derive(Debug)]
pub struct DynamicArrayNew<'a> {
    pub nodes: (Expression<'a>, Option<Expression<'a>>),
}

// -----------------------------------------------------------------------------

pub fn defparam_assignment(s: Span) -> IResult<Span, DefparamAssignment> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn net_decl_assignment(s: Span) -> IResult<Span, NetDeclAssignment> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn param_assignment(s: Span) -> IResult<Span, ParamAssignment> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn specparam_assignment(s: Span) -> IResult<Span, SpecparamAssignment> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn type_assignment(s: Span) -> IResult<Span, TypeAssignment> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn pulse_control_specparam(s: Span) -> IResult<Span, PulseControlSpecparam> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn error_limit_value(s: Span) -> IResult<Span, ErrorLimitValue> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn reject_limit_value(s: Span) -> IResult<Span, RejectLimitValue> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn limit_value(s: Span) -> IResult<Span, LimitValue> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn variable_decl_assignment(s: Span) -> IResult<Span, VariableDeclAssignment> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn class_new(s: Span) -> IResult<Span, ClassNew> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}

pub fn dynamic_array_new(s: Span) -> IResult<Span, DynamicArrayNew> {
    Err(Err::Error(make_error(s, ErrorKind::Fix)))
}
