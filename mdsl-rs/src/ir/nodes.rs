//! IR node definitions

/// Intermediate representation program
#[derive(Debug, Clone)]
pub struct IRProgram {
    /// List of imports
    pub imports: Vec<IRImport>,
    /// List of variables
    pub variables: Vec<IRVariable>,
    /// List of templates
    pub templates: Vec<IRTemplate>,
    /// List of units (schema definitions)
    pub units: Vec<IRUnit>,
    /// List of vocabularies
    pub vocabularies: Vec<IRVocabulary>,
    /// List of families
    pub families: Vec<IRFamily>,
    /// List of events
    pub events: Vec<IREvent>,
}

/// IR import
#[derive(Debug, Clone)]
pub struct IRImport {
    /// Import path
    pub path: String,
}

/// IR variable
#[derive(Debug, Clone)]
pub struct IRVariable {
    /// Variable name
    pub name: String,
    /// Variable value
    pub value: IRExpression,
}

/// IR template
#[derive(Debug, Clone)]
pub struct IRTemplate {
    /// Template name
    pub name: String,
    /// Template type
    pub template_type: String,
    /// Template blocks
    pub blocks: Vec<IRTemplateBlock>,
}

/// IR template block
#[derive(Debug, Clone)]
pub enum IRTemplateBlock {
    /// Characteristics block
    Characteristics(Vec<IRCharacteristic>),
    /// Metadata block
    Metadata(Vec<IRMetadata>),
}

/// IR characteristic
#[derive(Debug, Clone)]
pub struct IRCharacteristic {
    /// Characteristic name
    pub name: String,
    /// Characteristic value
    pub value: IRExpression,
}

/// IR metadata
#[derive(Debug, Clone)]
pub struct IRMetadata {
    /// Metadata name
    pub name: String,
    /// Metadata value
    pub value: IRExpression,
}

/// IR unit (schema definition)
#[derive(Debug, Clone)]
pub struct IRUnit {
    /// Unit name
    pub name: String,
    /// Unit fields
    pub fields: Vec<IRField>,
}

/// IR field
#[derive(Debug, Clone)]
pub struct IRField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: IRFieldType,
    /// Whether this is a primary key
    pub is_primary_key: bool,
}

/// IR field type
#[derive(Debug, Clone)]
pub enum IRFieldType {
    /// ID type
    Id,
    /// Text type with optional length
    Text(Option<u32>),
    /// Number type
    Number,
    /// Boolean type
    Boolean,
    /// Category type with values
    Category(Vec<String>),
}

/// IR vocabulary
#[derive(Debug, Clone)]
pub struct IRVocabulary {
    /// Vocabulary name
    pub name: String,
    /// Vocabulary body name
    pub body_name: String,
    /// Vocabulary entries
    pub entries: Vec<IRVocabularyEntry>,
}

/// IR vocabulary entry
#[derive(Debug, Clone)]
pub struct IRVocabularyEntry {
    /// Entry key
    pub key: IRVocabularyKey,
    /// Entry value
    pub value: String,
}

/// IR vocabulary key
#[derive(Debug, Clone)]
pub enum IRVocabularyKey {
    /// Numeric key
    Number(f64),
    /// String key
    String(String),
}

/// IR family
#[derive(Debug, Clone)]
pub struct IRFamily {
    /// Family name
    pub name: String,
    /// Family comment
    pub comment: Option<String>,
    /// Family outlets
    pub outlets: Vec<IROutlet>,
    /// Family relationships
    pub relationships: Vec<IRRelationship>,
    /// Family data blocks
    pub data_blocks: Vec<IRDataBlock>,
}

/// IR outlet
#[derive(Debug, Clone)]
pub struct IROutlet {
    /// Outlet name
    pub name: String,
    /// Outlet ID
    pub id: Option<u32>,
    /// Outlet template reference
    pub template_ref: Option<String>,
    /// Outlet base reference
    pub base_ref: Option<u32>,
    /// Outlet blocks
    pub blocks: Vec<IROutletBlock>,
}

/// IR outlet block
#[derive(Debug, Clone)]
pub enum IROutletBlock {
    /// Identity block
    Identity(Vec<IRIdentityField>),
    /// Lifecycle block
    Lifecycle(Vec<IRLifecycleStatus>),
    /// Characteristics block
    Characteristics(Vec<IRCharacteristic>),
    /// Metadata block
    Metadata(Vec<IRMetadata>),
}

/// IR identity field
#[derive(Debug, Clone)]
pub struct IRIdentityField {
    /// Field name
    pub name: String,
    /// Field value
    pub value: IRExpression,
}

/// IR lifecycle status
#[derive(Debug, Clone)]
pub struct IRLifecycleStatus {
    /// Status name
    pub status: String,
    /// Start date
    pub start_date: Option<String>,
    /// End date
    pub end_date: Option<String>,
    /// Precision start
    pub precision_start: Option<String>,
    /// Precision end
    pub precision_end: Option<String>,
    /// Comment
    pub comment: Option<String>,
}

/// IR data block
#[derive(Debug, Clone)]
pub struct IRDataBlock {
    /// Outlet ID this data is for
    pub outlet_id: u32,
    /// Data aggregation settings
    pub aggregation: Vec<IRDataAggregation>,
    /// Data years
    pub years: Vec<IRDataYear>,
    /// Maps to reference
    pub maps_to: Option<String>,
}

/// IR data aggregation
#[derive(Debug, Clone)]
pub struct IRDataAggregation {
    /// Aggregation name
    pub name: String,
    /// Aggregation value
    pub value: String,
}

/// IR data year
#[derive(Debug, Clone)]
pub struct IRDataYear {
    /// Year
    pub year: u32,
    /// Metrics
    pub metrics: Vec<IRDataMetric>,
    /// Comment
    pub comment: Option<String>,
}

/// IR data metric
#[derive(Debug, Clone)]
pub struct IRDataMetric {
    /// Metric name
    pub name: String,
    /// Metric value
    pub value: f64,
    /// Metric unit
    pub unit: String,
    /// Metric source
    pub source: String,
    /// Metric comment
    pub comment: Option<String>,
}

/// IR relationship
#[derive(Debug, Clone)]
pub enum IRRelationship {
    /// Diachronic link
    Diachronic(IRDiachronicLink),
    /// Synchronous link
    Synchronous(IRSynchronousLink),
}

/// IR diachronic link
#[derive(Debug, Clone)]
pub struct IRDiachronicLink {
    /// Link name
    pub name: String,
    /// Predecessor outlet ID
    pub predecessor: u32,
    /// Successor outlet ID
    pub successor: u32,
    /// Event start date
    pub event_start_date: Option<String>,
    /// Event end date
    pub event_end_date: Option<String>,
    /// Relationship type
    pub relationship_type: String,
    /// Comment
    pub comment: Option<String>,
    /// Maps to reference
    pub maps_to: Option<String>,
}

/// IR synchronous link
#[derive(Debug, Clone)]
pub struct IRSynchronousLink {
    /// Link name
    pub name: String,
    /// First outlet
    pub outlet_1: IRSyncOutlet,
    /// Second outlet
    pub outlet_2: IRSyncOutlet,
    /// Relationship type
    pub relationship_type: String,
    /// Period start
    pub period_start: Option<String>,
    /// Period end
    pub period_end: Option<String>,
    /// Details
    pub details: Option<String>,
    /// Maps to reference
    pub maps_to: Option<String>,
}

/// IR synchronous outlet reference
#[derive(Debug, Clone)]
pub struct IRSyncOutlet {
    /// Outlet ID
    pub id: u32,
    /// Outlet role
    pub role: String,
}

/// IR outlet field (deprecated, use IRIdentityField)
#[derive(Debug, Clone)]
pub struct IROutletField {
    /// Field name
    pub name: String,
    /// Field value
    pub value: IRExpression,
}

/// IR expression
#[derive(Debug, Clone)]
pub enum IRExpression {
    /// String literal
    String(String),
    /// Number literal
    Number(f64),
    /// Boolean literal
    Boolean(bool),
    /// Variable reference
    Variable(String),
    /// Object literal
    Object(Vec<IRObjectField>),
    /// Array literal
    Array(Vec<IRExpression>),
}

/// IR object field
#[derive(Debug, Clone)]
pub struct IRObjectField {
    /// Field name
    pub name: String,
    /// Field value
    pub value: IRExpression,
}

/// IR event declaration
#[derive(Debug, Clone)]
pub struct IREvent {
    /// Event name
    pub name: String,
    /// Event type
    pub event_type: String,
    /// Event date
    pub date: Option<String>,
    /// Event entities
    pub entities: Vec<IREventEntity>,
    /// Event impact
    pub impact: Vec<IREventImpact>,
    /// Event metadata
    pub metadata: Vec<IREventMetadata>,
    /// Event status
    pub status: Option<String>,
}

/// IR event entity
#[derive(Debug, Clone)]
pub struct IREventEntity {
    /// Entity name
    pub name: String,
    /// Entity ID
    pub id: u32,
    /// Entity role
    pub role: String,
    /// Stake before event
    pub stake_before: Option<f64>,
    /// Stake after event
    pub stake_after: Option<f64>,
}

/// IR event impact
#[derive(Debug, Clone)]
pub struct IREventImpact {
    /// Impact name
    pub name: String,
    /// Impact value
    pub value: IRExpression,
}

/// IR event metadata
#[derive(Debug, Clone)]
pub struct IREventMetadata {
    /// Metadata name
    pub name: String,
    /// Metadata value
    pub value: IRExpression,
}
