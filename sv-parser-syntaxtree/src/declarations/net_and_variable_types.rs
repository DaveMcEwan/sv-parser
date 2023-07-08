
use serde::{Serialize, Deserialize};
use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum CastingType {
    SimpleType(Box<SimpleType>),
    ConstantPrimary(Box<ConstantPrimary>),
    Signing(Box<Signing>),
    String(Box<Keyword>),
    Const(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DataType {
    Vector(Box<DataTypeVector>),
    Atom(Box<DataTypeAtom>),
    NonIntegerType(Box<NonIntegerType>),
    StructUnion(Box<DataTypeStructUnion>),
    Enum(Box<DataTypeEnum>),
    String(Box<Keyword>),
    Chandle(Box<Keyword>),
    Virtual(Box<DataTypeVirtual>),
    Type(Box<DataTypeType>),
    ClassType(Box<ClassType>),
    Event(Box<Keyword>),
    PsCovergroupIdentifier(Box<PsCovergroupIdentifier>),
    TypeReference(Box<TypeReference>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DataTypeVector {
    pub nodes: (IntegerVectorType, Option<Signing>, Vec<PackedDimension>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DataTypeAtom {
    pub nodes: (IntegerAtomType, Option<Signing>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DataTypeStructUnion {
    pub nodes: (
        StructUnion,
        Option<(Packed, Option<Signing>)>,
        Brace<(StructUnionMember, Vec<StructUnionMember>)>,
        Vec<PackedDimension>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Packed {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DataTypeEnum {
    pub nodes: (
        Keyword,
        Option<EnumBaseType>,
        Brace<List<Symbol, EnumNameDeclaration>>,
        Vec<PackedDimension>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DataTypeVirtual {
    pub nodes: (
        Keyword,
        Option<Interface>,
        InterfaceIdentifier,
        Option<ParameterValueAssignment>,
        Option<(Symbol, ModportIdentifier)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct Interface {
    pub nodes: (Keyword,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct DataTypeType {
    pub nodes: (
        Option<PackageScopeOrClassScope>,
        TypeIdentifier,
        Vec<PackedDimension>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DataTypeOrImplicit {
    DataType(Box<DataType>),
    ImplicitDataType(Box<ImplicitDataType>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ImplicitDataType {
    pub nodes: (Option<Signing>, Vec<PackedDimension>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum EnumBaseType {
    Atom(Box<EnumBaseTypeAtom>),
    Vector(Box<EnumBaseTypeVector>),
    Type(Box<EnumBaseTypeType>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EnumBaseTypeAtom {
    pub nodes: (IntegerAtomType, Option<Signing>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EnumBaseTypeVector {
    pub nodes: (IntegerVectorType, Option<Signing>, Option<PackedDimension>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EnumBaseTypeType {
    pub nodes: (TypeIdentifier, Option<PackedDimension>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct EnumNameDeclaration {
    pub nodes: (
        EnumIdentifier,
        Option<Bracket<(IntegralNumber, Option<(Symbol, IntegralNumber)>)>>,
        Option<(Symbol, ConstantExpression)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClassScope {
    pub nodes: (ClassType, Symbol),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct ClassType {
    pub nodes: (
        PsClassIdentifier,
        Option<ParameterValueAssignment>,
        Vec<(Symbol, ClassIdentifier, Option<ParameterValueAssignment>)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum IntegerType {
    IntegerVectorType(Box<IntegerVectorType>),
    IntegerAtomType(Box<IntegerAtomType>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum IntegerAtomType {
    Byte(Box<Keyword>),
    Shortint(Box<Keyword>),
    Int(Box<Keyword>),
    Longint(Box<Keyword>),
    Integer(Box<Keyword>),
    Time(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum IntegerVectorType {
    Bit(Box<Keyword>),
    Logic(Box<Keyword>),
    Reg(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum NonIntegerType {
    Shortreal(Box<Keyword>),
    Real(Box<Keyword>),
    Realtime(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum NetType {
    Supply0(Box<Keyword>),
    Supply1(Box<Keyword>),
    Tri(Box<Keyword>),
    Triand(Box<Keyword>),
    Trior(Box<Keyword>),
    Trireg(Box<Keyword>),
    Tri0(Box<Keyword>),
    Tri1(Box<Keyword>),
    Uwire(Box<Keyword>),
    Wire(Box<Keyword>),
    Wand(Box<Keyword>),
    Wor(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum NetPortType {
    DataType(Box<NetPortTypeDataType>),
    NetTypeIdentifier(Box<NetTypeIdentifier>),
    Interconnect(Box<NetPortTypeInterconnect>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NetPortTypeDataType {
    pub nodes: (Option<NetType>, DataTypeOrImplicit),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct NetPortTypeInterconnect {
    pub nodes: (Keyword, ImplicitDataType),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VariablePortType {
    pub nodes: (VarDataType,),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum VarDataType {
    DataType(Box<DataType>),
    Var(Box<VarDataTypeVar>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct VarDataTypeVar {
    pub nodes: (Keyword, DataTypeOrImplicit),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum Signing {
    Signed(Box<Keyword>),
    Unsigned(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum SimpleType {
    IntegerType(Box<IntegerType>),
    NonIntegerType(Box<NonIntegerType>),
    PsTypeIdentifier(Box<PsTypeIdentifier>),
    PsParameterIdentifier(Box<PsParameterIdentifier>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct StructUnionMember {
    pub nodes: (
        Vec<AttributeInstance>,
        Option<RandomQualifier>,
        DataTypeOrVoid,
        ListOfVariableDeclAssignments,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum DataTypeOrVoid {
    DataType(Box<DataType>),
    Void(Box<Keyword>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum StructUnion {
    Struct(Box<Keyword>),
    Union(Box<Keyword>),
    UnionTagged(Box<(Keyword, Keyword)>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub enum TypeReference {
    Expression(Box<TypeReferenceExpression>),
    DataType(Box<TypeReferenceDataType>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TypeReferenceExpression {
    pub nodes: (Keyword, Paren<Expression>),
}

#[derive(Clone, Debug, PartialEq, Node, Serialize, Deserialize)]
pub struct TypeReferenceDataType {
    pub nodes: (Keyword, Paren<DataType>),
}
