//! Abstract Syntax Tree definitions for the MediaLanguage DSL
//!
//! This module defines all AST node types that represent the structure of parsed MediaLanguage code.

use crate::error::SourcePosition;

/// Root node of the AST
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// List of top-level statements
    pub statements: Vec<Statement>,
    /// Source position of the program
    pub position: SourcePosition,
}

/// Top-level statement types
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Import statement
    Import(ImportStatement),
    /// Variable declaration
    Variable(VariableDeclaration),
    /// Unit declaration
    Unit(UnitDeclaration),
    /// Vocabulary declaration
    Vocabulary(VocabularyDeclaration),
    /// Family declaration
    Family(FamilyDeclaration),
    /// Template declaration
    Template(TemplateDeclaration),
    /// Data declaration
    Data(DataDeclaration),
    /// Relationship declaration
    Relationship(RelationshipDeclaration),
    /// Event declaration
    Event(EventDeclaration),
    /// Catalog declaration
    Catalog(CatalogDeclaration),
    /// Comment
    Comment(CommentStatement),
}

/// Import statement
#[derive(Debug, Clone, PartialEq)]
pub struct ImportStatement {
    /// Path to import
    pub path: String,
    /// Source position
    pub position: SourcePosition,
}

/// Variable declaration
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    /// Variable name
    pub name: String,
    /// Variable value
    pub value: Expression,
    /// Source position
    pub position: SourcePosition,
}

/// Unit declaration (table/entity definition)
#[derive(Debug, Clone, PartialEq)]
pub struct UnitDeclaration {
    /// Unit name
    pub name: String,
    /// Field definitions
    pub fields: Vec<FieldDeclaration>,
    /// Source position
    pub position: SourcePosition,
}

/// Field declaration within a unit
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDeclaration {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: FieldType,
    /// Whether this is a primary key
    pub is_primary_key: bool,
    /// Source position
    pub position: SourcePosition,
}

/// Field type definitions
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    /// ID type
    Id,
    /// Text type with optional length
    Text(Option<u32>),
    /// Number type
    Number,
    /// Boolean type
    Boolean,
    /// Category type with allowed values
    Category(Vec<String>),
}

/// Vocabulary declaration
#[derive(Debug, Clone, PartialEq)]
pub struct VocabularyDeclaration {
    /// Vocabulary name
    pub name: String,
    /// Vocabulary bodies (can have multiple nested vocabularies)
    pub bodies: Vec<VocabularyBody>,
    /// Source position
    pub position: SourcePosition,
}

/// Vocabulary body
#[derive(Debug, Clone, PartialEq)]
pub struct VocabularyBody {
    /// Body name
    pub name: String,
    /// Vocabulary entries
    pub entries: Vec<VocabularyEntry>,
    /// Source position
    pub position: SourcePosition,
}

/// Vocabulary entry
#[derive(Debug, Clone, PartialEq)]
pub struct VocabularyEntry {
    /// Key (number or string)
    pub key: VocabularyKey,
    /// Value
    pub value: String,
    /// Source position
    pub position: SourcePosition,
}

/// Vocabulary key
#[derive(Debug, Clone, PartialEq)]
pub enum VocabularyKey {
    /// Numeric key
    Number(f64),
    /// String key
    String(String),
}

/// Family declaration
#[derive(Debug, Clone, PartialEq)]
pub struct FamilyDeclaration {
    /// Family name
    pub name: String,
    /// Family members
    pub members: Vec<FamilyMember>,
    /// Source position
    pub position: SourcePosition,
}

/// Family member types
#[derive(Debug, Clone, PartialEq)]
pub enum FamilyMember {
    /// Outlet declaration
    Outlet(OutletDeclaration),
    /// Outlet reference
    OutletReference(OutletReference),
    /// Data declaration
    Data(DataDeclaration),
    /// Relationship declaration
    Relationship(RelationshipDeclaration),
    /// Comment
    Comment(CommentStatement),
}

/// Outlet declaration
#[derive(Debug, Clone, PartialEq)]
pub struct OutletDeclaration {
    /// Outlet name
    pub name: String,
    /// Inheritance clause
    pub inheritance: Option<InheritanceClause>,
    /// Outlet blocks
    pub blocks: Vec<OutletBlock>,
    /// Source position
    pub position: SourcePosition,
}

/// Inheritance clause
#[derive(Debug, Clone, PartialEq)]
pub enum InheritanceClause {
    /// Extends template
    ExtendsTemplate(String),
    /// Based on ID
    BasedOn(f64),
}

/// Outlet block types
#[derive(Debug, Clone, PartialEq)]
pub enum OutletBlock {
    /// Identity block
    Identity(IdentityBlock),
    /// Lifecycle block
    Lifecycle(LifecycleBlock),
    /// Characteristics block
    Characteristics(CharacteristicsBlock),
    /// Metadata block
    Metadata(MetadataBlock),
    /// Comment
    Comment(CommentStatement),
}

/// Identity block
#[derive(Debug, Clone, PartialEq)]
pub struct IdentityBlock {
    /// Identity fields
    pub fields: Vec<IdentityField>,
    /// Source position
    pub position: SourcePosition,
}

/// Identity field
#[derive(Debug, Clone, PartialEq)]
pub enum IdentityField {
    /// Simple field assignment
    Assignment {
        /// Field name
        name: String,
        /// Field value
        value: Expression,
        /// Source position
        position: SourcePosition,
    },
    /// Array field assignment
    ArrayAssignment {
        /// Field name
        name: String,
        /// Array values
        values: Vec<ObjectLiteral>,
        /// Source position
        position: SourcePosition,
    },
    /// Comment
    Comment(CommentStatement),
}

/// Lifecycle block
#[derive(Debug, Clone, PartialEq)]
pub struct LifecycleBlock {
    /// Lifecycle entries
    pub entries: Vec<LifecycleEntry>,
    /// Source position
    pub position: SourcePosition,
}

/// Lifecycle entry
#[derive(Debug, Clone, PartialEq)]
pub struct LifecycleEntry {
    /// Status name
    pub status: String,
    /// Start date
    pub from: DateExpression,
    /// End date (optional)
    pub to: Option<DateExpression>,
    /// Lifecycle attributes
    pub attributes: Vec<LifecycleAttribute>,
    /// Source position
    pub position: SourcePosition,
}

/// Lifecycle attribute
#[derive(Debug, Clone, PartialEq)]
pub enum LifecycleAttribute {
    /// Simple attribute assignment
    Assignment {
        /// Attribute name
        name: String,
        /// Attribute value
        value: Expression,
        /// Source position
        position: SourcePosition,
    },
    /// Comment
    Comment(CommentStatement),
}

/// Date expression
#[derive(Debug, Clone, PartialEq)]
pub enum DateExpression {
    /// Date literal
    Literal(String),
    /// Current date
    Current,
}

/// Characteristics block
#[derive(Debug, Clone, PartialEq)]
pub struct CharacteristicsBlock {
    /// Characteristic fields
    pub fields: Vec<CharacteristicField>,
    /// Source position
    pub position: SourcePosition,
}

/// Characteristic field
#[derive(Debug, Clone, PartialEq)]
pub enum CharacteristicField {
    /// Simple field assignment
    Assignment {
        /// Field name
        name: String,
        /// Field value
        value: Expression,
        /// Source position
        position: SourcePosition,
    },
    /// Nested field assignment
    NestedAssignment {
        /// Field name
        name: String,
        /// Nested fields
        fields: Vec<NestedField>,
        /// Source position
        position: SourcePosition,
    },
    /// Comment
    Comment(CommentStatement),
}

/// Nested field
#[derive(Debug, Clone, PartialEq)]
pub enum NestedField {
    /// Simple assignment
    Assignment {
        /// Field name
        name: String,
        /// Field value
        value: Expression,
        /// Source position
        position: SourcePosition,
    },
    /// Comment
    Comment(CommentStatement),
}

/// Metadata block
#[derive(Debug, Clone, PartialEq)]
pub struct MetadataBlock {
    /// Metadata fields
    pub fields: Vec<MetadataField>,
    /// Source position
    pub position: SourcePosition,
}

/// Metadata field
#[derive(Debug, Clone, PartialEq)]
pub enum MetadataField {
    /// Simple field assignment
    Assignment {
        /// Field name
        name: String,
        /// Field value
        value: Expression,
        /// Source position
        position: SourcePosition,
    },
    /// Comment
    Comment(CommentStatement),
}

/// Template declaration
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateDeclaration {
    /// Template name
    pub name: String,
    /// Template blocks
    pub blocks: Vec<OutletBlock>,
    /// Source position
    pub position: SourcePosition,
}

/// Outlet reference
#[derive(Debug, Clone, PartialEq)]
pub struct OutletReference {
    /// Referenced outlet ID
    pub id: f64,
    /// Reference name
    pub name: String,
    /// Source position
    pub position: SourcePosition,
}

/// Data declaration
#[derive(Debug, Clone, PartialEq)]
pub struct DataDeclaration {
    /// Target ID
    pub target_id: f64,
    /// Data blocks
    pub blocks: Vec<DataBlock>,
    /// Source position
    pub position: SourcePosition,
}

/// Data block types
#[derive(Debug, Clone, PartialEq)]
pub enum DataBlock {
    /// Annotation
    Annotation(AnnotationStatement),
    /// Aggregation declaration
    Aggregation(AggregationDeclaration),
    /// Year declaration
    Year(YearDeclaration),
    /// Comment
    Comment(CommentStatement),
}

/// Aggregation declaration
#[derive(Debug, Clone, PartialEq)]
pub struct AggregationDeclaration {
    /// Aggregation fields
    pub fields: Vec<AggregationField>,
    /// Source position
    pub position: SourcePosition,
}

/// Aggregation field
#[derive(Debug, Clone, PartialEq)]
pub struct AggregationField {
    /// Field name
    pub name: String,
    /// Field value
    pub value: String,
    /// Source position
    pub position: SourcePosition,
}

/// Year declaration
#[derive(Debug, Clone, PartialEq)]
pub struct YearDeclaration {
    /// Year value
    pub year: f64,
    /// Year blocks
    pub blocks: Vec<YearBlock>,
    /// Source position
    pub position: SourcePosition,
}

/// Year block types
#[derive(Debug, Clone, PartialEq)]
pub enum YearBlock {
    /// Metrics block
    Metrics(MetricsBlock),
    /// Comment assignment
    CommentAssignment {
        /// Comment value
        value: String,
        /// Source position
        position: SourcePosition,
    },
    /// Comment
    Comment(CommentStatement),
}

/// Metrics block
#[derive(Debug, Clone, PartialEq)]
pub struct MetricsBlock {
    /// Metric fields
    pub fields: Vec<MetricField>,
    /// Source position
    pub position: SourcePosition,
}

/// Metric field
#[derive(Debug, Clone, PartialEq)]
pub struct MetricField {
    /// Field name
    pub name: String,
    /// Metric attributes
    pub attributes: Vec<MetricAttribute>,
    /// Source position
    pub position: SourcePosition,
}

/// Metric attribute
#[derive(Debug, Clone, PartialEq)]
pub struct MetricAttribute {
    /// Attribute name
    pub name: String,
    /// Attribute value
    pub value: Expression,
    /// Source position
    pub position: SourcePosition,
}

/// Relationship declaration
#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipDeclaration {
    /// Diachronic link
    Diachronic(DiachronicLink),
    /// Synchronous link
    Synchronous(SynchronousLink),
}

/// Event declaration
#[derive(Debug, Clone, PartialEq)]
pub struct EventDeclaration {
    /// Event name
    pub name: String,
    /// Event fields
    pub fields: Vec<EventField>,
    /// Source position
    pub position: SourcePosition,
}

/// Event field
#[derive(Debug, Clone, PartialEq)]
pub enum EventField {
    /// Event type
    Type {
        /// Type value
        value: String,
        /// Source position
        position: SourcePosition,
    },
    /// Event date
    Date {
        /// Date value
        value: DateExpression,
        /// Source position
        position: SourcePosition,
    },
    /// Event status
    Status {
        /// Status value
        value: String,
        /// Source position
        position: SourcePosition,
    },
    /// Event entities
    Entities {
        /// Entity list
        entities: Vec<EventEntity>,
        /// Source position
        position: SourcePosition,
    },
    /// Event impact
    Impact {
        /// Impact fields
        impact: Vec<ImpactField>,
        /// Source position
        position: SourcePosition,
    },
    /// Event metadata
    Metadata {
        /// Metadata fields
        metadata: Vec<MetadataField>,
        /// Source position
        position: SourcePosition,
    },
    /// Annotation
    Annotation {
        /// Annotation name
        name: String,
        /// Annotation value
        value: Option<String>,
        /// Source position
        position: SourcePosition,
    },
    /// Comment
    Comment {
        /// Comment text
        text: String,
        /// Source position
        position: SourcePosition,
    },
}

/// Event entity
#[derive(Debug, Clone, PartialEq)]
pub struct EventEntity {
    /// Entity name
    pub name: String,
    /// Entity roles
    pub roles: Vec<EntityRole>,
    /// Source position
    pub position: SourcePosition,
}

/// Entity role
#[derive(Debug, Clone, PartialEq)]
pub enum EntityRole {
    /// Entity ID
    Id {
        /// ID value
        value: f64,
        /// Source position
        position: SourcePosition,
    },
    /// Entity role
    Role {
        /// Role value
        value: String,
        /// Source position
        position: SourcePosition,
    },
    /// Stake before event
    StakeBefore {
        /// Stake percentage
        value: f64,
        /// Source position
        position: SourcePosition,
    },
    /// Stake after event
    StakeAfter {
        /// Stake percentage
        value: f64,
        /// Source position
        position: SourcePosition,
    },
}

/// Impact field
#[derive(Debug, Clone, PartialEq)]
pub struct ImpactField {
    /// Field name
    pub name: String,
    /// Field value
    pub value: Expression,
    /// Source position
    pub position: SourcePosition,
}


/// Diachronic link
#[derive(Debug, Clone, PartialEq)]
pub struct DiachronicLink {
    /// Link name
    pub name: String,
    /// Link fields
    pub fields: Vec<DiachronicField>,
    /// Source position
    pub position: SourcePosition,
}

/// Diachronic field
#[derive(Debug, Clone, PartialEq)]
pub enum DiachronicField {
    /// Predecessor assignment
    Predecessor {
        /// Predecessor ID
        value: f64,
        /// Source position
        position: SourcePosition,
    },
    /// Successor assignment
    Successor {
        /// Successor ID
        value: f64,
        /// Source position
        position: SourcePosition,
    },
    /// Event date assignment
    EventDate {
        /// Date range
        value: DateRange,
        /// Source position
        position: SourcePosition,
    },
    /// Relationship type assignment
    RelationshipType {
        /// Type value
        value: String,
        /// Source position
        position: SourcePosition,
    },
    /// Triggered by event
    TriggeredByEvent {
        /// Event identifier
        value: String,
        /// Source position
        position: SourcePosition,
    },
    /// Annotation
    Annotation(AnnotationStatement),
    /// Comment
    Comment(CommentStatement),
}

/// Synchronous link
#[derive(Debug, Clone, PartialEq)]
pub struct SynchronousLink {
    /// Link name
    pub name: String,
    /// Link fields
    pub fields: Vec<SynchronousField>,
    /// Source position
    pub position: SourcePosition,
}

/// Synchronous field
#[derive(Debug, Clone, PartialEq)]
pub enum SynchronousField {
    /// Outlet 1 specification
    Outlet1 {
        /// Outlet specification
        spec: OutletSpec,
        /// Source position
        position: SourcePosition,
    },
    /// Outlet 2 specification
    Outlet2 {
        /// Outlet specification
        spec: OutletSpec,
        /// Source position
        position: SourcePosition,
    },
    /// Relationship type assignment
    RelationshipType {
        /// Type value
        value: String,
        /// Source position
        position: SourcePosition,
    },
    /// Period assignment
    Period {
        /// Date range
        value: DateRange,
        /// Source position
        position: SourcePosition,
    },
    /// Details assignment
    Details {
        /// Details value
        value: String,
        /// Source position
        position: SourcePosition,
    },
    /// Created by event
    CreatedByEvent {
        /// Event identifier
        value: String,
        /// Source position
        position: SourcePosition,
    },
    /// Annotation
    Annotation(AnnotationStatement),
    /// Comment
    Comment(CommentStatement),
}

/// Outlet specification
#[derive(Debug, Clone, PartialEq)]
pub struct OutletSpec {
    /// Outlet ID
    pub id: f64,
    /// Outlet role
    pub role: Option<String>,
    /// Source position
    pub position: SourcePosition,
}

/// Date range
#[derive(Debug, Clone, PartialEq)]
pub struct DateRange {
    /// Start date
    pub from: DateExpression,
    /// End date (optional)
    pub to: Option<DateExpression>,
    /// Source position
    pub position: SourcePosition,
}

/// Object literal
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectLiteral {
    /// Object fields
    pub fields: Vec<ObjectField>,
    /// Source position
    pub position: SourcePosition,
}

/// Object field
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectField {
    /// Simple assignment
    Assignment {
        /// Field name
        name: String,
        /// Field value
        value: Expression,
        /// Source position
        position: SourcePosition,
    },
    /// Period assignment
    Period {
        /// Date range
        value: DateRange,
        /// Source position
        position: SourcePosition,
    },
}

/// Expression types
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Variable reference
    Variable(String),
    /// String literal
    String(String),
    /// Number literal
    Number(f64),
    /// Boolean literal
    Boolean(bool),
    /// Object literal
    Object(ObjectLiteral),
}

/// Comment statement
#[derive(Debug, Clone, PartialEq)]
pub struct CommentStatement {
    /// Comment text
    pub text: String,
    /// Whether this is a multi-line comment
    pub is_multiline: bool,
    /// Source position
    pub position: SourcePosition,
}

/// Annotation statement
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationStatement {
    /// Annotation name
    pub name: String,
    /// Annotation value (optional)
    pub value: Option<String>,
    /// Source position
    pub position: SourcePosition,
}

/// Catalog declaration
#[derive(Debug, Clone, PartialEq)]
pub struct CatalogDeclaration {
    /// Catalog name
    pub name: String,
    /// Source entries
    pub sources: Vec<SourceDeclaration>,
    /// Source position
    pub position: SourcePosition,
}

/// Source declaration within a catalog
#[derive(Debug, Clone, PartialEq)]
pub struct SourceDeclaration {
    /// Source name
    pub name: String,
    /// Source fields
    pub fields: Vec<SourceField>,
    /// Source position
    pub position: SourcePosition,
}

/// Source field types
#[derive(Debug, Clone, PartialEq)]
pub enum SourceField {
    /// Simple field assignment
    Assignment {
        /// Field name
        name: String,
        /// Field value
        value: Expression,
        /// Source position
        position: SourcePosition,
    },
    /// Nested field assignment
    NestedAssignment {
        /// Field name
        name: String,
        /// Nested fields
        fields: Vec<NestedSourceField>,
        /// Source position
        position: SourcePosition,
    },
    /// Annotation
    Annotation(AnnotationStatement),
    /// Comment
    Comment(CommentStatement),
}

/// Nested source field
#[derive(Debug, Clone, PartialEq)]
pub enum NestedSourceField {
    /// Simple assignment
    Assignment {
        /// Field name
        name: String,
        /// Field value
        value: Expression,
        /// Source position
        position: SourcePosition,
    },
    /// Comment
    Comment(CommentStatement),
}

// Convenience implementations
impl Program {
    /// Create a new program
    pub fn new(statements: Vec<Statement>, position: SourcePosition) -> Self {
        Self {
            statements,
            position,
        }
    }
}

impl Statement {
    /// Get the source position of this statement
    pub fn position(&self) -> SourcePosition {
        match self {
            Statement::Import(s) => s.position,
            Statement::Variable(s) => s.position,
            Statement::Unit(s) => s.position,
            Statement::Vocabulary(s) => s.position,
            Statement::Family(s) => s.position,
            Statement::Template(s) => s.position,
            Statement::Data(s) => s.position,
            Statement::Relationship(s) => match s {
                RelationshipDeclaration::Diachronic(d) => d.position,
                RelationshipDeclaration::Synchronous(s) => s.position,
            },
            Statement::Event(s) => s.position,
            Statement::Catalog(s) => s.position,
            Statement::Comment(s) => s.position,
        }
    }
}
