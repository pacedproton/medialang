//! # MDSL Abstract Syntax Tree (AST)
//!
//! Purpose: Define the Abstract Syntax Tree for the Media Description Specification Language.
//! This AST represents the structural organization of MDSL documents.
//!
//! ## Design Philosophy
//! - Clear hierarchical structure matching MDSL semantics
//! - Extensible for future language features
//! - Rich position information for error reporting

use crate::lexer::Position;
use std::collections::HashMap;

/// Represents a complete MDSL program/file
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
    pub position: Position,
}

/// Top-level items in an MDSL program
#[derive(Debug, Clone)]
pub enum Item {
    Import(ImportStatement),
    VariableDeclaration(VariableDeclaration),
    TemplateDefinition(TemplateDefinition),
    FamilyDefinition(FamilyDefinition),
    UnitDefinition(UnitDefinition),
    VocabularyDefinition(VocabularyDefinition),
}

/// Import statement: IMPORT "filename.mdsl";
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub path: String,
    pub position: Position,
}

/// Variable declaration: LET name = value;
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub value: Value,
    pub position: Position,
}

/// Template definition for outlet inheritance
#[derive(Debug, Clone)]
pub struct TemplateDefinition {
    pub name: String,
    pub outlet_type: String,
    pub body: OutletBody,
    pub position: Position,
}

/// Family definition containing multiple outlets
#[derive(Debug, Clone)]
pub struct FamilyDefinition {
    pub name: String,
    pub annotations: Vec<Annotation>,
    pub items: Vec<FamilyItem>,
    pub position: Position,
}

/// Items that can appear inside a family
#[derive(Debug, Clone)]
pub enum FamilyItem {
    Outlet(OutletDefinition),
    OutletRef(OutletReference),
    DiachronicLink(DiachronicLink),
    SynchronousLink(SynchronousLink),
    DataBlock(DataBlock),
}

/// Outlet definition
#[derive(Debug, Clone)]
pub struct OutletDefinition {
    pub name: String,
    pub id: Option<Value>,
    pub inheritance: Option<Inheritance>,
    pub body: OutletBody,
    pub position: Position,
}

/// Outlet inheritance specification
#[derive(Debug, Clone)]
pub enum Inheritance {
    Extends { template: String },
    BasedOn { outlet_id: Value },
}

/// Reference to an existing outlet
#[derive(Debug, Clone)]
pub struct OutletReference {
    pub id: Value,
    pub names: Vec<String>,
    pub annotations: Vec<Annotation>,
    pub overrides: Vec<Override>,
    pub metadata: Option<MetadataBlock>,
    pub position: Position,
}

/// Override specification
#[derive(Debug, Clone)]
pub struct Override {
    pub from_date: Option<String>,
    pub until_date: Option<String>,
    pub conditions: Vec<OverrideCondition>,
    pub position: Position,
}

/// Override conditions
#[derive(Debug, Clone)]
pub enum OverrideCondition {
    ForPeriod {
        from: String,
        to: String,
        body: OutletBody,
    },
    InheritsFrom {
        id: Value,
        until: String,
    },
    Assignment {
        key: String,
        value: Value,
    },
}

/// Outlet body containing various sections
#[derive(Debug, Clone)]
pub struct OutletBody {
    pub identity: Option<IdentityBlock>,
    pub lifecycle: Option<LifecycleBlock>,
    pub characteristics: Option<CharacteristicsBlock>,
    pub metadata: Option<MetadataBlock>,
}

/// Identity section
#[derive(Debug, Clone)]
pub struct IdentityBlock {
    pub fields: HashMap<String, Value>,
    pub position: Position,
}

/// Lifecycle section
#[derive(Debug, Clone)]
pub struct LifecycleBlock {
    pub statuses: Vec<StatusDeclaration>,
    pub position: Position,
}

/// Status declaration in lifecycle
#[derive(Debug, Clone)]
pub struct StatusDeclaration {
    pub status: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub precision_start: Option<String>,
    pub precision_end: Option<String>,
    pub annotations: Vec<Annotation>,
    pub position: Position,
}

/// Characteristics section
#[derive(Debug, Clone)]
pub struct CharacteristicsBlock {
    pub fields: HashMap<String, Value>,
    pub position: Position,
}

/// Metadata section  
#[derive(Debug, Clone)]
pub struct MetadataBlock {
    pub fields: HashMap<String, Value>,
    pub position: Position,
}

/// Diachronic link (temporal relationship)
#[derive(Debug, Clone)]
pub struct DiachronicLink {
    pub name: String,
    pub predecessor: Value,
    pub successor: Value,
    pub event_date: Option<String>,
    pub relationship_type: Option<String>,
    pub annotations: Vec<Annotation>,
    pub position: Position,
}

/// Synchronous link (concurrent relationship)
#[derive(Debug, Clone)]
pub struct SynchronousLink {
    pub name: String,
    pub outlet_1: OutletLinkInfo,
    pub outlet_2: OutletLinkInfo,
    pub relationship_type: Option<String>,
    pub period: Option<String>,
    pub details: Option<String>,
    pub annotations: Vec<Annotation>,
    pub position: Position,
}

/// Outlet information in links
#[derive(Debug, Clone)]
pub struct OutletLinkInfo {
    pub id: Value,
    pub role: Option<String>,
}

/// Data block for market data
#[derive(Debug, Clone)]
pub struct DataBlock {
    pub outlet_id: Value,
    pub annotations: Vec<Annotation>,
    pub aggregation: Option<HashMap<String, Value>>,
    pub years: Vec<YearData>,
    pub position: Position,
}

/// Year-specific data
#[derive(Debug, Clone)]
pub struct YearData {
    pub year: Value,
    pub metrics: Option<HashMap<String, MetricValue>>,
    pub comment: Option<String>,
    pub position: Position,
}

/// Metric value with metadata
#[derive(Debug, Clone)]
pub struct MetricValue {
    pub value: Value,
    pub unit: Option<String>,
    pub source: Option<String>,
    pub comment: Option<String>,
}

/// Unit definition for schema
#[derive(Debug, Clone)]
pub struct UnitDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub position: Position,
}

/// Field definition in units
#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: FieldType,
    pub constraints: Vec<String>,
    pub position: Position,
}

/// Field types
#[derive(Debug, Clone)]
pub enum FieldType {
    Id,
    Text(Option<u32>),
    Number,
    Boolean,
    Category(Vec<String>),
}

/// Vocabulary definition
#[derive(Debug, Clone)]
pub struct VocabularyDefinition {
    pub name: String,
    pub entries: HashMap<String, HashMap<String, String>>,
    pub position: Position,
}

/// Annotation (comments, mappings, etc.)
#[derive(Debug, Clone)]
pub struct Annotation {
    pub kind: AnnotationKind,
    pub content: String,
    pub position: Position,
}

/// Types of annotations
#[derive(Debug, Clone)]
pub enum AnnotationKind {
    Comment,
    MapsTo,
    Family,
}

/// Values in the AST
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Variable(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Current, // Special CURRENT keyword
}
