
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ArrayIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct BlockIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct BinIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CIdentifier {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CellIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CheckerIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClassIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClassVariableIdentifier {
    pub nodes: (VariableIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClockingIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConfigIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ConstraintIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CovergroupIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CovergroupVariableIdentifier {
    pub nodes: (VariableIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CoverPointIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct CrossIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DynamicArrayVariableIdentifier {
    pub nodes: (VariableIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EnumIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EscapedIdentifier {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FormalIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FormalPortIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct FunctionIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GenerateBlockIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct GenvarIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalArrayIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalBlockIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalEventIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalIdentifier {
    pub nodes: (
        Option<Root>,
        Vec<(Identifier, ConstantBitSelect, Symbol)>,
        Identifier,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Root {
    pub nodes: (Keyword, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalNetIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalParameterIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalPropertyIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalSequenceIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalTaskIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalTfIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct HierarchicalVariableIdentifier {
    pub nodes: (HierarchicalIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum Identifier {
    SimpleIdentifier(Box<SimpleIdentifier>),
    EscapedIdentifier(Box<EscapedIdentifier>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct IndexVariableIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InterfaceIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InterfaceInstanceIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InoutPortIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InputPortIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct InstanceIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct LibraryIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct MemberIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct MethodIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModportIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ModuleIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NetIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NetTypeIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct OutputPortIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PackageIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PackageScope {
    Package(Box<PackageScopePackage>),
    Unit(Box<Unit>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PackageScopePackage {
    pub nodes: (PackageIdentifier, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Unit {
    pub nodes: (Keyword, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ParameterIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PortIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProductionIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ProgramIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PropertyIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsClassIdentifier {
    pub nodes: (Option<PackageScope>, ClassIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsCovergroupIdentifier {
    pub nodes: (Option<PackageScope>, CovergroupIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsCheckerIdentifier {
    pub nodes: (Option<PackageScope>, CheckerIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsIdentifier {
    pub nodes: (Option<PackageScope>, Identifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsOrHierarchicalArrayIdentifier {
    pub nodes: (
        Option<ImplicitClassHandleOrClassScopeOrPackageScope>,
        HierarchicalArrayIdentifier,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PsOrHierarchicalNetIdentifier {
    PackageScope(Box<PsOrHierarchicalNetIdentifierPackageScope>),
    HierarchicalNetIdentifier(Box<HierarchicalNetIdentifier>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsOrHierarchicalNetIdentifierPackageScope {
    pub nodes: (Option<PackageScope>, NetIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsOrHierarchicalNetIdentifierHierarchical {
    pub nodes: (HierarchicalNetIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PsOrHierarchicalPropertyIdentifier {
    PackageScope(Box<PsOrHierarchicalPropertyIdentifierPackageScope>),
    HierarchicalPropertyIdentifier(Box<HierarchicalPropertyIdentifier>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsOrHierarchicalPropertyIdentifierPackageScope {
    pub nodes: (Option<PackageScope>, PropertyIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsOrHierarchicalPropertyIdentifierHierarchical {
    pub nodes: (HierarchicalPropertyIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PsOrHierarchicalSequenceIdentifier {
    PackageScope(Box<PsOrHierarchicalSequenceIdentifierPackageScope>),
    HierarchicalSequenceIdentifier(Box<HierarchicalSequenceIdentifier>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsOrHierarchicalSequenceIdentifierPackageScope {
    pub nodes: (Option<PackageScope>, SequenceIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsOrHierarchicalSequenceIdentifierHierarchical {
    pub nodes: (HierarchicalSequenceIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PsOrHierarchicalTfIdentifier {
    PackageScope(Box<PsOrHierarchicalTfIdentifierPackageScope>),
    HierarchicalTfIdentifier(Box<HierarchicalTfIdentifier>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsOrHierarchicalTfIdentifierPackageScope {
    pub nodes: (
        Option<ImplicitClassHandleOrClassScopeOrPackageScope>,
        TfIdentifier,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsOrHierarchicalTfIdentifierHierarchical {
    pub nodes: (HierarchicalTfIdentifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PsParameterIdentifier {
    Scope(Box<PsParameterIdentifierScope>),
    Generate(Box<PsParameterIdentifierGenerate>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsParameterIdentifierScope {
    pub nodes: (Option<PackageScopeOrClassScope>, ParameterIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsParameterIdentifierGenerate {
    pub nodes: (
        Vec<(
            GenerateBlockIdentifier,
            Option<Bracket<ConstantExpression>>,
            Symbol,
        )>,
        ParameterIdentifier,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct PsTypeIdentifier {
    pub nodes: (Option<LocalOrPackageScopeOrClassScope>, TypeIdentifier),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum LocalOrPackageScopeOrClassScope {
    Local(Box<Local>),
    PackageScope(Box<PackageScope>),
    ClassScope(Box<ClassScope>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Local {
    pub nodes: (Keyword, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SequenceIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SignalIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SimpleIdentifier {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SpecparamIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct SystemTfIdentifier {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TaskIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TfIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TerminalIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TopmoduleIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TypeIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct UdpIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VariableIdentifier {
    pub nodes: (Identifier,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ImplicitClassHandleOrClassScopeOrPackageScope {
    ImplicitClassHandle(Box<(ImplicitClassHandle, Symbol)>),
    ClassScope(Box<ClassScope>),
    PackageScope(Box<PackageScope>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ImplicitClassHandleOrPackageScope {
    ImplicitClassHandle(Box<(ImplicitClassHandle, Symbol)>),
    PackageScope(Box<PackageScope>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum ImplicitClassHandleOrClassScope {
    ImplicitClassHandle(Box<(ImplicitClassHandle, Symbol)>),
    ClassScope(Box<ClassScope>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum PackageScopeOrClassScope {
    PackageScope(Box<PackageScope>),
    ClassScope(Box<ClassScope>),
}
