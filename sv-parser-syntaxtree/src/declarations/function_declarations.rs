
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum FunctionDataTypeOrImplicit {
    DataTypeOrVoid(Box<DataTypeOrVoid>),
    ImplicitDataType(Box<ImplicitDataType>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub nodes: (Keyword, Option<Lifetime>, FunctionBodyDeclaration),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum FunctionBodyDeclaration {
    WithoutPort(Box<FunctionBodyDeclarationWithoutPort>),
    WithPort(Box<FunctionBodyDeclarationWithPort>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FunctionBodyDeclarationWithoutPort {
    pub nodes: (
        FunctionDataTypeOrImplicit,
        Option<InterfaceIdentifierOrClassScope>,
        FunctionIdentifier,
        Symbol,
        Vec<TfItemDeclaration>,
        Vec<FunctionStatementOrNull>,
        Keyword,
        Option<(Symbol, FunctionIdentifier)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FunctionBodyDeclarationWithPort {
    pub nodes: (
        FunctionDataTypeOrImplicit,
        Option<InterfaceIdentifierOrClassScope>,
        FunctionIdentifier,
        Paren<Option<TfPortList>>,
        Symbol,
        Vec<BlockItemDeclaration>,
        Vec<FunctionStatementOrNull>,
        Keyword,
        Option<(Symbol, FunctionIdentifier)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum InterfaceIdentifierOrClassScope {
    InterfaceIdentifier(Box<(InterfaceIdentifier, Symbol)>),
    ClassScope(Box<ClassScope>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FunctionPrototype {
    pub nodes: (
        Keyword,
        DataTypeOrVoid,
        FunctionIdentifier,
        Option<Paren<Option<TfPortList>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DpiImportExport {
    ImportFunction(Box<DpiImportExportImportFunction>),
    ImportTask(Box<DpiImportExportImportTask>),
    ExportFunction(Box<DpiImportExportExportFunction>),
    ExportTask(Box<DpiImportExportExportTask>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DpiImportExportImportFunction {
    pub nodes: (
        Keyword,
        DpiSpecString,
        Option<DpiFunctionImportProperty>,
        Option<(CIdentifier, Symbol)>,
        DpiFunctionProto,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DpiImportExportImportTask {
    pub nodes: (
        Keyword,
        DpiSpecString,
        Option<DpiTaskImportProperty>,
        Option<(CIdentifier, Symbol)>,
        DpiTaskProto,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DpiImportExportExportFunction {
    pub nodes: (
        Keyword,
        DpiSpecString,
        Option<(CIdentifier, Symbol)>,
        Keyword,
        FunctionIdentifier,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DpiImportExportExportTask {
    pub nodes: (
        Keyword,
        DpiSpecString,
        Option<(CIdentifier, Symbol)>,
        Keyword,
        TaskIdentifier,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DpiSpecString {
    DpiC(Box<Keyword>),
    Dpi(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DpiFunctionImportProperty {
    Context(Box<Keyword>),
    Pure(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DpiTaskImportProperty {
    Context(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DpiFunctionProto {
    pub nodes: (FunctionPrototype,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DpiTaskProto {
    pub nodes: (TaskPrototype,),
}
