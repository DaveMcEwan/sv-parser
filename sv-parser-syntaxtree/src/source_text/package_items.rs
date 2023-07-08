
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PackageItem {
    PackageOrGenerateItemDeclaration(Box<PackageOrGenerateItemDeclaration>),
    AnonymousProgram(Box<AnonymousProgram>),
    PackageExportDeclaration(Box<PackageExportDeclaration>),
    TimeunitsDeclaration(Box<TimeunitsDeclaration>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PackageOrGenerateItemDeclaration {
    NetDeclaration(Box<NetDeclaration>),
    DataDeclaration(Box<DataDeclaration>),
    TaskDeclaration(Box<TaskDeclaration>),
    FunctionDeclaration(Box<FunctionDeclaration>),
    CheckerDeclaration(Box<CheckerDeclaration>),
    DpiImportExport(Box<DpiImportExport>),
    ExternConstraintDeclaration(Box<ExternConstraintDeclaration>),
    ClassDeclaration(Box<ClassDeclaration>),
    InterfaceClassDeclaration(Box<InterfaceClassDeclaration>),
    ClassConstructorDeclaration(Box<ClassConstructorDeclaration>),
    LocalParameterDeclaration(Box<(LocalParameterDeclaration, Symbol)>),
    ParameterDeclaration(Box<(ParameterDeclaration, Symbol)>),
    CovergroupDeclaration(Box<CovergroupDeclaration>),
    AssertionItemDeclaration(Box<AssertionItemDeclaration>),
    Empty(Box<Symbol>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct AnonymousProgram {
    pub nodes: (Keyword, Symbol, Vec<AnonymousProgramItem>, Keyword),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum AnonymousProgramItem {
    TaskDeclaration(Box<TaskDeclaration>),
    FunctionDeclaration(Box<FunctionDeclaration>),
    ClassDeclaration(Box<ClassDeclaration>),
    InterfaceClassDeclaration(Box<InterfaceClassDeclaration>),
    CovergroupDeclaration(Box<CovergroupDeclaration>),
    ClassConstructorDeclaration(Box<ClassConstructorDeclaration>),
    Empty(Box<Symbol>),
}
