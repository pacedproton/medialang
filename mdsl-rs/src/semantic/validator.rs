//! Comprehensive semantic validator for MediaLanguage DSL
//!
//! This module provides multi-level validation including:
//! - Semantic consistency checking
//! - Reference resolution validation
//! - Business rule validation
//! - Domain-specific MediaLanguage validation

use crate::error::SourcePosition;
use crate::parser::ast::*;
use std::collections::{HashMap, HashSet};

/// Validation severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationSeverity {
    /// Critical errors that prevent code generation
    Error,
    /// Warnings about potential issues
    Warning,
    /// Informational messages about style or best practices
    Info,
}

/// Validation issue with detailed context
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Severity level
    pub severity: ValidationSeverity,
    /// Error code for categorization
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Source position where issue occurs
    pub position: SourcePosition,
    /// Optional suggestion for fixing the issue
    pub suggestion: Option<String>,
    /// Additional context information
    pub context: HashMap<String, String>,
}

/// Validation result containing all issues found
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// List of all validation issues
    pub issues: Vec<ValidationIssue>,
    /// Whether validation passed (no errors)
    pub passed: bool,
    /// Summary statistics
    pub summary: ValidationSummary,
}

/// Summary statistics for validation
#[derive(Debug, Clone)]
pub struct ValidationSummary {
    /// Number of error-level issues
    pub errors: usize,
    /// Number of warning-level issues
    pub warnings: usize,
    /// Number of info-level issues
    pub info: usize,
    /// Total number of constructs validated
    pub total_constructs: usize,
}

/// Symbol table for tracking declarations and references
#[derive(Debug, Clone)]
struct SymbolTable {
    /// Imported files
    imports: HashSet<String>,
    /// Variable declarations
    variables: HashMap<String, SourcePosition>,
    /// Template declarations
    templates: HashMap<String, SourcePosition>,
    /// Unit declarations
    units: HashMap<String, SourcePosition>,
    /// Vocabulary declarations
    vocabularies: HashMap<String, SourcePosition>,
    /// Family declarations
    families: HashMap<String, SourcePosition>,
    /// Outlet declarations (ID -> position)
    outlets: HashMap<u32, SourcePosition>,
    /// Outlet names (name -> ID)
    outlet_names: HashMap<String, u32>,
}

/// Comprehensive semantic validator
pub struct Validator {
    /// Symbol table for tracking declarations
    symbols: SymbolTable,
    /// Validation issues found
    issues: Vec<ValidationIssue>,
    /// Current validation context
    context: Vec<String>,
}

impl Validator {
    /// Create a new validator
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable {
                imports: HashSet::new(),
                variables: HashMap::new(),
                templates: HashMap::new(),
                units: HashMap::new(),
                vocabularies: HashMap::new(),
                families: HashMap::new(),
                outlets: HashMap::new(),
                outlet_names: HashMap::new(),
            },
            issues: Vec::new(),
            context: Vec::new(),
        }
    }

    /// Validate a complete program
    pub fn validate(&mut self, program: &Program) -> ValidationResult {
        self.push_context("Program");

        // Phase 1: Collect all declarations
        self.collect_declarations(program);

        // Phase 2: Validate individual constructs
        self.validate_statements(&program.statements);

        // Phase 3: Validate cross-references
        self.validate_references(program);

        // Phase 4: Validate business rules
        self.validate_business_rules(program);

        self.pop_context();

        // Generate summary
        let summary = self.generate_summary();
        let passed = summary.errors == 0;

        ValidationResult {
            issues: self.issues.clone(),
            passed,
            summary,
        }
    }

    /// Collect all declarations in the program
    fn collect_declarations(&mut self, program: &Program) {
        for statement in &program.statements {
            match statement {
                Statement::Import(import) => {
                    self.symbols.imports.insert(import.path.clone());
                }
                Statement::Variable(var) => {
                    if let Some(existing) = self
                        .symbols
                        .variables
                        .insert(var.name.clone(), var.position.clone())
                    {
                        self.add_error(
                            "VAR_REDECLARED",
                            format!("Variable '{}' is already declared", var.name),
                            var.position.clone(),
                            Some(format!(
                                "Previous declaration at {}:{}",
                                existing.line, existing.column
                            )),
                        );
                    }
                }
                Statement::Template(template) => {
                    if let Some(existing) = self
                        .symbols
                        .templates
                        .insert(template.name.clone(), template.position.clone())
                    {
                        self.add_error(
                            "TEMPLATE_REDECLARED",
                            format!("Template '{}' is already declared", template.name),
                            template.position.clone(),
                            Some(format!(
                                "Previous declaration at {}:{}",
                                existing.line, existing.column
                            )),
                        );
                    }
                }
                Statement::Unit(unit) => {
                    if let Some(existing) = self
                        .symbols
                        .units
                        .insert(unit.name.clone(), unit.position.clone())
                    {
                        self.add_error(
                            "UNIT_REDECLARED",
                            format!("Unit '{}' is already declared", unit.name),
                            unit.position.clone(),
                            Some(format!(
                                "Previous declaration at {}:{}",
                                existing.line, existing.column
                            )),
                        );
                    }
                }
                Statement::Vocabulary(vocab) => {
                    if let Some(existing) = self
                        .symbols
                        .vocabularies
                        .insert(vocab.name.clone(), vocab.position.clone())
                    {
                        self.add_error(
                            "VOCAB_REDECLARED",
                            format!("Vocabulary '{}' is already declared", vocab.name),
                            vocab.position.clone(),
                            Some(format!(
                                "Previous declaration at {}:{}",
                                existing.line, existing.column
                            )),
                        );
                    }
                }
                Statement::Family(family) => {
                    if let Some(existing) = self
                        .symbols
                        .families
                        .insert(family.name.clone(), family.position.clone())
                    {
                        self.add_error(
                            "FAMILY_REDECLARED",
                            format!("Family '{}' is already declared", family.name),
                            family.position.clone(),
                            Some(format!(
                                "Previous declaration at {}:{}",
                                existing.line, existing.column
                            )),
                        );
                    }

                    // Collect outlet declarations
                    for member in &family.members {
                        if let FamilyMember::Outlet(outlet) = member {
                            // Extract outlet ID from identity block
                            let outlet_id = self.extract_outlet_id(outlet);
                            if let Some(id) = outlet_id {
                                if let Some(existing) =
                                    self.symbols.outlets.insert(id, outlet.position.clone())
                                {
                                    self.add_error(
                                        "OUTLET_ID_DUPLICATE",
                                        format!("Outlet ID {} is already used", id),
                                        outlet.position.clone(),
                                        Some(format!(
                                            "Previous outlet at {}:{}",
                                            existing.line, existing.column
                                        )),
                                    );
                                }
                                self.symbols.outlet_names.insert(outlet.name.clone(), id);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Validate individual statements
    fn validate_statements(&mut self, statements: &[Statement]) {
        for statement in statements {
            match statement {
                Statement::Import(import) => self.validate_import(import),
                Statement::Variable(var) => self.validate_variable(var),
                Statement::Template(template) => self.validate_template(template),
                Statement::Unit(unit) => self.validate_unit(unit),
                Statement::Vocabulary(vocab) => self.validate_vocabulary(vocab),
                Statement::Family(family) => self.validate_family(family),
                Statement::Data(data) => self.validate_data(data),
                Statement::Relationship(rel) => self.validate_relationship(rel),
                _ => {}
            }
        }
    }

    /// Validate import statement
    fn validate_import(&mut self, import: &ImportStatement) {
        self.push_context(&format!("Import({})", import.path));

        // Check for valid file extension
        if !import.path.ends_with(".mdsl") {
            self.add_warning(
                "IMPORT_NO_EXTENSION",
                format!("Import path '{}' should end with '.mdsl'", import.path),
                import.position.clone(),
                Some("Add '.mdsl' extension to import path".to_string()),
            );
        }

        // Check for relative path issues
        if import.path.contains("..") {
            self.add_info(
                "IMPORT_RELATIVE_PATH",
                format!("Import uses relative path: '{}'", import.path),
                import.position.clone(),
                Some("Consider using absolute paths for better maintainability".to_string()),
            );
        }

        self.pop_context();
    }

    /// Validate variable declaration
    fn validate_variable(&mut self, var: &VariableDeclaration) {
        self.push_context(&format!("Variable({})", var.name));

        // Check naming convention
        if !var.name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            self.add_warning(
                "VAR_NAMING",
                format!(
                    "Variable name '{}' contains non-alphanumeric characters",
                    var.name
                ),
                var.position.clone(),
                Some("Use only letters, numbers, and underscores in variable names".to_string()),
            );
        }

        // Validate expression
        self.validate_expression(&var.value);

        self.pop_context();
    }

    /// Validate template declaration
    fn validate_template(&mut self, template: &TemplateDeclaration) {
        self.push_context(&format!("Template({})", template.name));

        // Check for empty template
        if template.blocks.is_empty() {
            self.add_warning(
                "TEMPLATE_EMPTY",
                format!("Template '{}' has no blocks", template.name),
                template.position.clone(),
                Some("Add characteristics or metadata blocks to make template useful".to_string()),
            );
        }

        // Validate blocks
        for block in &template.blocks {
            self.validate_outlet_block(block);
        }

        self.pop_context();
    }

    /// Validate unit declaration
    fn validate_unit(&mut self, unit: &UnitDeclaration) {
        self.push_context(&format!("Unit({})", unit.name));

        // Check for empty unit
        if unit.fields.is_empty() {
            self.add_error(
                "UNIT_EMPTY",
                format!("Unit '{}' has no fields", unit.name),
                unit.position.clone(),
                Some("Add field declarations to unit".to_string()),
            );
        }

        // Check for primary key
        let has_primary_key = unit.fields.iter().any(|f| f.is_primary_key);
        if !has_primary_key {
            self.add_warning(
                "UNIT_NO_PRIMARY_KEY",
                format!("Unit '{}' has no primary key", unit.name),
                unit.position.clone(),
                Some("Consider adding a PRIMARY KEY field".to_string()),
            );
        }

        // Validate fields
        let mut field_names = HashSet::new();
        for field in &unit.fields {
            if !field_names.insert(field.name.clone()) {
                self.add_error(
                    "UNIT_FIELD_DUPLICATE",
                    format!(
                        "Field '{}' is declared multiple times in unit '{}'",
                        field.name, unit.name
                    ),
                    field.position.clone(),
                    Some("Remove duplicate field declaration".to_string()),
                );
            }
            self.validate_field(field);
        }

        self.pop_context();
    }

    /// Validate field declaration
    fn validate_field(&mut self, field: &FieldDeclaration) {
        self.push_context(&format!("Field({})", field.name));

        // Check field type validity
        match &field.field_type {
            FieldType::Text(length) => {
                if let Some(len) = length {
                    if *len == 0 {
                        self.add_error(
                            "FIELD_TEXT_ZERO_LENGTH",
                            format!("TEXT field '{}' has zero length", field.name),
                            field.position.clone(),
                            Some("Specify a positive length for TEXT fields".to_string()),
                        );
                    }
                    if *len > 65535 {
                        self.add_warning(
                            "FIELD_TEXT_LARGE",
                            format!(
                                "TEXT field '{}' has very large length ({})",
                                field.name, len
                            ),
                            field.position.clone(),
                            Some(
                                "Consider using a smaller length or different field type"
                                    .to_string(),
                            ),
                        );
                    }
                }
            }
            FieldType::Category(values) => {
                if values.is_empty() {
                    self.add_error(
                        "FIELD_CATEGORY_EMPTY",
                        format!("CATEGORY field '{}' has no values", field.name),
                        field.position.clone(),
                        Some("Add at least one value to CATEGORY field".to_string()),
                    );
                }

                // Check for duplicate values
                let mut seen = HashSet::new();
                for value in values {
                    if !seen.insert(value) {
                        self.add_error(
                            "FIELD_CATEGORY_DUPLICATE",
                            format!(
                                "CATEGORY field '{}' has duplicate value '{}'",
                                field.name, value
                            ),
                            field.position.clone(),
                            Some("Remove duplicate values from CATEGORY field".to_string()),
                        );
                    }
                }
            }
            _ => {}
        }

        self.pop_context();
    }

    /// Validate vocabulary declaration
    fn validate_vocabulary(&mut self, vocab: &VocabularyDeclaration) {
        self.push_context(&format!("Vocabulary({})", vocab.name));

        if vocab.bodies.is_empty() {
            self.add_error(
                "VOCAB_EMPTY",
                format!("Vocabulary '{}' has no bodies", vocab.name),
                vocab.position.clone(),
                Some("Add at least one vocabulary body".to_string()),
            );
        }

        for body in &vocab.bodies {
            self.validate_vocabulary_body(body);
        }

        self.pop_context();
    }

    /// Validate vocabulary body
    fn validate_vocabulary_body(&mut self, body: &VocabularyBody) {
        self.push_context(&format!("VocabBody({})", body.name));

        if body.entries.is_empty() {
            self.add_warning(
                "VOCAB_BODY_EMPTY",
                format!("Vocabulary body '{}' has no entries", body.name),
                body.position.clone(),
                Some("Add vocabulary entries".to_string()),
            );
        }

        // Check for duplicate keys
        let mut seen_keys = HashSet::new();
        for entry in &body.entries {
            let key_str = match &entry.key {
                VocabularyKey::Number(n) => n.to_string(),
                VocabularyKey::String(s) => s.clone(),
            };

            if !seen_keys.insert(key_str.clone()) {
                self.add_error(
                    "VOCAB_DUPLICATE_KEY",
                    format!(
                        "Vocabulary body '{}' has duplicate key '{}'",
                        body.name, key_str
                    ),
                    entry.position.clone(),
                    Some("Remove duplicate key or use different key".to_string()),
                );
            }
        }

        self.pop_context();
    }

    /// Validate family declaration
    fn validate_family(&mut self, family: &FamilyDeclaration) {
        self.push_context(&format!("Family({})", family.name));

        if family.members.is_empty() {
            self.add_warning(
                "FAMILY_EMPTY",
                format!("Family '{}' has no members", family.name),
                family.position.clone(),
                Some("Add outlets, relationships, or data declarations".to_string()),
            );
        }

        // Count different member types
        let mut outlet_count = 0;
        let mut relationship_count = 0;
        let mut _data_count = 0;

        for member in &family.members {
            match member {
                FamilyMember::Outlet(outlet) => {
                    outlet_count += 1;
                    self.validate_outlet(outlet);
                }
                FamilyMember::Relationship(rel) => {
                    relationship_count += 1;
                    self.validate_relationship(rel);
                }
                FamilyMember::Data(data) => {
                    _data_count += 1;
                    self.validate_data(data);
                }
                _ => {}
            }
        }

        // Validate family structure
        if outlet_count == 0 {
            self.add_warning(
                "FAMILY_NO_OUTLETS",
                format!("Family '{}' has no outlets", family.name),
                family.position.clone(),
                Some("Add outlet declarations to family".to_string()),
            );
        }

        if outlet_count == 1 && relationship_count > 0 {
            self.add_warning(
                "FAMILY_SINGLE_OUTLET_RELATIONSHIPS",
                format!(
                    "Family '{}' has only one outlet but {} relationships",
                    family.name, relationship_count
                ),
                family.position.clone(),
                Some("Relationships typically require multiple outlets".to_string()),
            );
        }

        self.pop_context();
    }

    /// Validate outlet declaration
    fn validate_outlet(&mut self, outlet: &OutletDeclaration) {
        self.push_context(&format!("Outlet({})", outlet.name));

        // Check for required blocks
        let mut has_identity = false;
        let mut has_characteristics = false;

        for block in &outlet.blocks {
            match block {
                OutletBlock::Identity(_) => has_identity = true,
                OutletBlock::Characteristics(_) => has_characteristics = true,
                _ => {}
            }
            self.validate_outlet_block(block);
        }

        if !has_identity {
            self.add_error(
                "OUTLET_NO_IDENTITY",
                format!("Outlet '{}' has no identity block", outlet.name),
                outlet.position.clone(),
                Some("Add an identity block with required fields".to_string()),
            );
        }

        if !has_characteristics {
            self.add_warning(
                "OUTLET_NO_CHARACTERISTICS",
                format!("Outlet '{}' has no characteristics block", outlet.name),
                outlet.position.clone(),
                Some("Consider adding characteristics to describe the outlet".to_string()),
            );
        }

        // Validate inheritance
        if let Some(inheritance) = &outlet.inheritance {
            self.validate_inheritance(inheritance);
        }

        self.pop_context();
    }

    /// Validate inheritance clause
    fn validate_inheritance(&mut self, inheritance: &InheritanceClause) {
        match inheritance {
            InheritanceClause::ExtendsTemplate(template_name) => {
                if !self.symbols.templates.contains_key(template_name) {
                    self.add_error(
                        "TEMPLATE_NOT_FOUND",
                        format!("Template '{}' not found", template_name),
                        SourcePosition::start(), // TODO: Get actual position
                        Some("Declare the template before using it".to_string()),
                    );
                }
            }
            InheritanceClause::BasedOn(outlet_id) => {
                let id_u32 = *outlet_id as u32;
                if !self.symbols.outlets.contains_key(&id_u32) {
                    self.add_error(
                        "OUTLET_NOT_FOUND",
                        format!("Outlet with ID {} not found", outlet_id),
                        SourcePosition::start(), // TODO: Get actual position
                        Some("Declare the base outlet before referencing it".to_string()),
                    );
                }
            }
        }
    }

    /// Validate outlet block
    fn validate_outlet_block(&mut self, block: &OutletBlock) {
        match block {
            OutletBlock::Identity(identity) => {
                self.validate_identity_block(identity);
            }
            OutletBlock::Lifecycle(lifecycle) => {
                self.validate_lifecycle_block(lifecycle);
            }
            OutletBlock::Characteristics(chars) => {
                self.validate_characteristics_block(chars);
            }
            OutletBlock::Metadata(metadata) => {
                self.validate_metadata_block(metadata);
            }
            OutletBlock::Comment(_) => {
                // Comments don't need validation
            }
        }
    }

    /// Validate identity block
    fn validate_identity_block(&mut self, identity: &IdentityBlock) {
        self.push_context("Identity");

        // Check for required fields
        let mut has_id = false;
        let mut has_title = false;

        for field in &identity.fields {
            if let IdentityField::Assignment { name, .. } = field {
                match name.as_str() {
                    "id" => has_id = true,
                    "title" => has_title = true,
                    _ => {}
                }
            }
        }

        if !has_id {
            self.add_error(
                "IDENTITY_NO_ID",
                "Identity block missing required 'id' field".to_string(),
                identity.position.clone(),
                Some("Add 'id = <number>' to identity block".to_string()),
            );
        }

        if !has_title {
            self.add_warning(
                "IDENTITY_NO_TITLE",
                "Identity block missing 'title' field".to_string(),
                identity.position.clone(),
                Some("Add 'title = \"<name>\"' to identity block".to_string()),
            );
        }

        self.pop_context();
    }

    /// Validate lifecycle block
    fn validate_lifecycle_block(&mut self, lifecycle: &LifecycleBlock) {
        self.push_context("Lifecycle");

        if lifecycle.entries.is_empty() {
            self.add_warning(
                "LIFECYCLE_EMPTY",
                "Lifecycle block has no entries".to_string(),
                lifecycle.position.clone(),
                Some("Add lifecycle status entries".to_string()),
            );
        }

        // Check for overlapping periods
        for (i, entry) in lifecycle.entries.iter().enumerate() {
            for (j, other) in lifecycle.entries.iter().enumerate() {
                if i != j && entry.status == other.status {
                    self.add_warning(
                        "LIFECYCLE_DUPLICATE_STATUS",
                        format!("Duplicate lifecycle status '{}'", entry.status),
                        entry.position.clone(),
                        Some("Each status should appear only once".to_string()),
                    );
                }
            }
        }

        self.pop_context();
    }

    /// Validate characteristics block
    fn validate_characteristics_block(&mut self, chars: &CharacteristicsBlock) {
        self.push_context("Characteristics");

        if chars.fields.is_empty() {
            self.add_warning(
                "CHARACTERISTICS_EMPTY",
                "Characteristics block has no fields".to_string(),
                chars.position.clone(),
                Some("Add characteristic assignments".to_string()),
            );
        }

        // Check for duplicate characteristics
        let mut seen = HashSet::new();
        for field in &chars.fields {
            if let CharacteristicField::Assignment { name, .. } = field {
                if !seen.insert(name.clone()) {
                    self.add_warning(
                        "CHARACTERISTICS_DUPLICATE",
                        format!("Duplicate characteristic '{}'", name),
                        chars.position.clone(),
                        Some("Remove duplicate characteristic".to_string()),
                    );
                }
            }
        }

        self.pop_context();
    }

    /// Validate metadata block
    fn validate_metadata_block(&mut self, metadata: &MetadataBlock) {
        self.push_context("Metadata");

        if metadata.fields.is_empty() {
            self.add_info(
                "METADATA_EMPTY",
                "Metadata block has no fields".to_string(),
                metadata.position.clone(),
                Some("Add metadata assignments".to_string()),
            );
        }

        self.pop_context();
    }

    /// Validate data declaration
    fn validate_data(&mut self, data: &DataDeclaration) {
        self.push_context(&format!("Data({})", data.target_id));

        // Check if target outlet exists
        if !self.symbols.outlets.contains_key(&(data.target_id as u32)) {
            self.add_error(
                "DATA_OUTLET_NOT_FOUND",
                format!(
                    "Data declaration references non-existent outlet ID {}",
                    data.target_id
                ),
                data.position.clone(),
                Some("Declare the outlet before adding data".to_string()),
            );
        }

        if data.blocks.is_empty() {
            self.add_warning(
                "DATA_EMPTY",
                format!(
                    "Data declaration for outlet {} has no blocks",
                    data.target_id
                ),
                data.position.clone(),
                Some("Add data blocks (aggregation, years, etc.)".to_string()),
            );
        }

        self.pop_context();
    }

    /// Validate relationship declaration
    fn validate_relationship(&mut self, rel: &RelationshipDeclaration) {
        match rel {
            RelationshipDeclaration::Diachronic(diachronic) => {
                self.validate_diachronic_relationship(diachronic);
            }
            RelationshipDeclaration::Synchronous(sync) => {
                self.validate_synchronous_relationship(sync);
            }
        }
    }

    /// Validate diachronic relationship
    fn validate_diachronic_relationship(&mut self, diachronic: &DiachronicLink) {
        self.push_context(&format!("DiachronicRel({})", diachronic.name));

        // Extract outlet IDs from fields
        let mut predecessor_id = None;
        let mut successor_id = None;

        for field in &diachronic.fields {
            match field {
                DiachronicField::Predecessor { value, .. } => {
                    predecessor_id = Some(*value as u32);
                }
                DiachronicField::Successor { value, .. } => {
                    successor_id = Some(*value as u32);
                }
                _ => {}
            }
        }

        // Validate outlet references
        if let Some(pred_id) = predecessor_id {
            if !self.symbols.outlets.contains_key(&pred_id) {
                self.add_error(
                    "RELATIONSHIP_PREDECESSOR_NOT_FOUND",
                    format!("Predecessor outlet {} not found", pred_id),
                    diachronic.position.clone(),
                    Some("Declare the predecessor outlet before referencing it".to_string()),
                );
            }
        }

        if let Some(succ_id) = successor_id {
            if !self.symbols.outlets.contains_key(&succ_id) {
                self.add_error(
                    "RELATIONSHIP_SUCCESSOR_NOT_FOUND",
                    format!("Successor outlet {} not found", succ_id),
                    diachronic.position.clone(),
                    Some("Declare the successor outlet before referencing it".to_string()),
                );
            }
        }

        // Check for self-relationship
        if predecessor_id == successor_id && predecessor_id.is_some() {
            self.add_warning(
                "RELATIONSHIP_SELF_REFERENCE",
                "Diachronic relationship references the same outlet as both predecessor and successor".to_string(),
                diachronic.position.clone(),
                Some("Verify this self-relationship is intentional".to_string()),
            );
        }

        self.pop_context();
    }

    /// Validate synchronous relationship
    fn validate_synchronous_relationship(&mut self, sync: &SynchronousLink) {
        self.push_context(&format!("SynchronousRel({})", sync.name));

        // Extract outlet IDs from fields
        let mut outlet_1_id = None;
        let mut outlet_2_id = None;

        for field in &sync.fields {
            match field {
                SynchronousField::Outlet1 { spec, .. } => {
                    outlet_1_id = Some(spec.id as u32);
                }
                SynchronousField::Outlet2 { spec, .. } => {
                    outlet_2_id = Some(spec.id as u32);
                }
                _ => {}
            }
        }

        // Validate outlet references
        if let Some(id1) = outlet_1_id {
            if !self.symbols.outlets.contains_key(&id1) {
                self.add_error(
                    "RELATIONSHIP_OUTLET1_NOT_FOUND",
                    format!("Outlet 1 with ID {} not found", id1),
                    sync.position.clone(),
                    Some("Declare the outlet before referencing it".to_string()),
                );
            }
        }

        if let Some(id2) = outlet_2_id {
            if !self.symbols.outlets.contains_key(&id2) {
                self.add_error(
                    "RELATIONSHIP_OUTLET2_NOT_FOUND",
                    format!("Outlet 2 with ID {} not found", id2),
                    sync.position.clone(),
                    Some("Declare the outlet before referencing it".to_string()),
                );
            }
        }

        // Check for self-relationship
        if outlet_1_id == outlet_2_id && outlet_1_id.is_some() {
            self.add_warning(
                "RELATIONSHIP_SELF_REFERENCE",
                "Synchronous relationship references the same outlet twice".to_string(),
                sync.position.clone(),
                Some("Verify this self-relationship is intentional".to_string()),
            );
        }

        self.pop_context();
    }

    /// Validate cross-references in the program
    fn validate_references(&mut self, program: &Program) {
        self.push_context("References");

        // Check for unused declarations
        self.check_unused_declarations(program);

        // Check for circular dependencies
        self.check_circular_dependencies(program);

        self.pop_context();
    }

    /// Check for unused declarations
    fn check_unused_declarations(&mut self, _program: &Program) {
        // TODO: Implement usage tracking and report unused templates, variables, etc.
        // This would require a more sophisticated analysis pass
    }

    /// Check for circular dependencies
    fn check_circular_dependencies(&mut self, _program: &Program) {
        // TODO: Implement circular dependency detection for template inheritance
        // and outlet relationships
    }

    /// Validate business rules
    fn validate_business_rules(&mut self, _program: &Program) {
        self.push_context("BusinessRules");

        // TODO: Implement MediaLanguage-specific business rules:
        // - Outlet ID ranges (e.g., 200000-299999 for newspapers)
        // - Required characteristics for certain outlet types
        // - Temporal consistency in lifecycle and relationships
        // - Data integrity rules

        self.pop_context();
    }

    /// Validate expression
    fn validate_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Variable(name) => {
                if !self.symbols.variables.contains_key(name) {
                    self.add_error(
                        "VARIABLE_NOT_FOUND",
                        format!("Variable '{}' not found", name),
                        SourcePosition::start(), // TODO: Get actual position
                        Some("Declare the variable before using it".to_string()),
                    );
                }
            }
            Expression::Object(obj) => {
                for field in &obj.fields {
                    if let ObjectField::Assignment { value, .. } = field {
                        self.validate_expression(value);
                    }
                }
            }
            _ => {}
        }
    }

    /// Extract outlet ID from outlet declaration
    fn extract_outlet_id(&self, outlet: &OutletDeclaration) -> Option<u32> {
        for block in &outlet.blocks {
            if let OutletBlock::Identity(identity) = block {
                for field in &identity.fields {
                    if let IdentityField::Assignment { name, value, .. } = field {
                        if name == "id" {
                            if let Expression::Number(n) = value {
                                return Some(*n as u32);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Add an error to the validation results
    fn add_error(
        &mut self,
        code: &str,
        message: String,
        position: SourcePosition,
        suggestion: Option<String>,
    ) {
        self.issues.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            code: code.to_string(),
            message,
            position,
            suggestion,
            context: self.create_context_map(),
        });
    }

    /// Add a warning to the validation results
    fn add_warning(
        &mut self,
        code: &str,
        message: String,
        position: SourcePosition,
        suggestion: Option<String>,
    ) {
        self.issues.push(ValidationIssue {
            severity: ValidationSeverity::Warning,
            code: code.to_string(),
            message,
            position,
            suggestion,
            context: self.create_context_map(),
        });
    }

    /// Add an info message to the validation results
    fn add_info(
        &mut self,
        code: &str,
        message: String,
        position: SourcePosition,
        suggestion: Option<String>,
    ) {
        self.issues.push(ValidationIssue {
            severity: ValidationSeverity::Info,
            code: code.to_string(),
            message,
            position,
            suggestion,
            context: self.create_context_map(),
        });
    }

    /// Push a context onto the context stack
    fn push_context(&mut self, context: &str) {
        self.context.push(context.to_string());
    }

    /// Pop a context from the context stack
    fn pop_context(&mut self) {
        self.context.pop();
    }

    /// Create context map for current validation context
    fn create_context_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("context_path".to_string(), self.context.join(" > "));
        map
    }

    /// Generate validation summary
    fn generate_summary(&self) -> ValidationSummary {
        let mut errors = 0;
        let mut warnings = 0;
        let mut info = 0;

        for issue in &self.issues {
            match issue.severity {
                ValidationSeverity::Error => errors += 1,
                ValidationSeverity::Warning => warnings += 1,
                ValidationSeverity::Info => info += 1,
            }
        }

        ValidationSummary {
            errors,
            warnings,
            info,
            total_constructs: self.symbols.templates.len()
                + self.symbols.units.len()
                + self.symbols.vocabularies.len()
                + self.symbols.families.len(),
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to validate a program
pub fn validate_program(program: &Program) -> ValidationResult {
    let mut validator = Validator::new();
    validator.validate(program)
}

/// Validation report formatter
pub struct ValidationReporter;

impl ValidationReporter {
    /// Format validation result as a human-readable report
    pub fn format_report(result: &ValidationResult, file_name: Option<&str>) -> String {
        let mut report = String::new();

        // Header
        if let Some(name) = file_name {
            report.push_str(&format!("Validation Report for: {}\n", name));
        } else {
            report.push_str("Validation Report\n");
        }
        report.push_str(&"=".repeat(50));
        report.push('\n');

        // Summary
        report.push_str(&format!(
            "Status: {}\n",
            if result.passed { "PASSED" } else { "FAILED" }
        ));
        report.push_str(&format!(
            "Total Constructs: {}\n",
            result.summary.total_constructs
        ));
        report.push_str(&format!("Errors: {}\n", result.summary.errors));
        report.push_str(&format!("Warnings: {}\n", result.summary.warnings));
        report.push_str(&format!("Info: {}\n", result.summary.info));
        report.push('\n');

        // Issues
        if !result.issues.is_empty() {
            report.push_str("Issues Found:\n");
            report.push_str(&"-".repeat(30));
            report.push('\n');

            for (i, issue) in result.issues.iter().enumerate() {
                report.push_str(&format!("{}. ", i + 1));
                report.push_str(&Self::format_issue(issue));
                report.push('\n');
            }
        } else {
            report.push_str("No issues found!\n");
        }

        report
    }

    /// Format a single validation issue
    pub fn format_issue(issue: &ValidationIssue) -> String {
        let severity_str = match issue.severity {
            ValidationSeverity::Error => "ERROR",
            ValidationSeverity::Warning => "WARNING",
            ValidationSeverity::Info => "INFO",
        };

        let mut formatted = format!(
            "[{}] {} ({}:{}): {}",
            severity_str, issue.code, issue.position.line, issue.position.column, issue.message
        );

        if let Some(suggestion) = &issue.suggestion {
            formatted.push_str(&format!("\n   Suggestion: {}", suggestion));
        }

        if let Some(context) = issue.context.get("context_path") {
            formatted.push_str(&format!("\n   Context: {}", context));
        }

        formatted
    }

    /// Format validation result as JSON
    pub fn format_json(result: &ValidationResult) -> String {
        // Simple JSON formatting without external dependencies
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"passed\": {},\n", result.passed));
        json.push_str("  \"summary\": {\n");
        json.push_str(&format!("    \"errors\": {},\n", result.summary.errors));
        json.push_str(&format!("    \"warnings\": {},\n", result.summary.warnings));
        json.push_str(&format!("    \"info\": {},\n", result.summary.info));
        json.push_str(&format!(
            "    \"total_constructs\": {}\n",
            result.summary.total_constructs
        ));
        json.push_str("  },\n");
        json.push_str("  \"issues\": [\n");

        for (i, issue) in result.issues.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"severity\": \"{:?}\",\n", issue.severity));
            json.push_str(&format!("      \"code\": \"{}\",\n", issue.code));
            json.push_str(&format!(
                "      \"message\": \"{}\",\n",
                issue.message.replace('"', "\\\"")
            ));
            json.push_str("      \"position\": {\n");
            json.push_str(&format!("        \"line\": {},\n", issue.position.line));
            json.push_str(&format!("        \"column\": {}\n", issue.position.column));
            json.push_str("      }");

            if let Some(suggestion) = &issue.suggestion {
                json.push_str(",\n");
                json.push_str(&format!(
                    "      \"suggestion\": \"{}\"",
                    suggestion.replace('"', "\\\"")
                ));
            }

            json.push_str("\n    }");
            if i < result.issues.len() - 1 {
                json.push(',');
            }
            json.push('\n');
        }

        json.push_str("  ]\n");
        json.push_str("}\n");
        json
    }

    /// Format validation result as CSV
    pub fn format_csv(result: &ValidationResult) -> String {
        let mut csv = String::new();
        csv.push_str("Severity,Code,Line,Column,Message,Suggestion,Context\n");

        for issue in &result.issues {
            let severity = match issue.severity {
                ValidationSeverity::Error => "Error",
                ValidationSeverity::Warning => "Warning",
                ValidationSeverity::Info => "Info",
            };

            let suggestion = issue.suggestion.as_deref().unwrap_or("");
            let context = issue
                .context
                .get("context_path")
                .map(|s| s.as_str())
                .unwrap_or("");

            csv.push_str(&format!(
                "{},{},{},{},\"{}\",\"{}\",\"{}\"\n",
                severity,
                issue.code,
                issue.position.line,
                issue.position.column,
                issue.message.replace('"', "\"\""),
                suggestion.replace('"', "\"\""),
                context.replace('"', "\"\"")
            ));
        }

        csv
    }

    /// Print colored validation report to terminal
    pub fn print_colored_report(result: &ValidationResult, file_name: Option<&str>) {
        // Header
        if let Some(name) = file_name {
            println!("Validation Report for: {}", name);
        } else {
            println!("Validation Report");
        }
        println!("{}", "=".repeat(50));

        // Summary with colors
        if result.passed {
            println!("Status: \x1b[32mPASSED\x1b[0m");
        } else {
            println!("Status: \x1b[31mFAILED\x1b[0m");
        }

        println!("Total Constructs: {}", result.summary.total_constructs);

        if result.summary.errors > 0 {
            println!("Errors: \x1b[31m{}\x1b[0m", result.summary.errors);
        } else {
            println!("Errors: {}", result.summary.errors);
        }

        if result.summary.warnings > 0 {
            println!("Warnings: \x1b[33m{}\x1b[0m", result.summary.warnings);
        } else {
            println!("Warnings: {}", result.summary.warnings);
        }

        if result.summary.info > 0 {
            println!("Info: \x1b[36m{}\x1b[0m", result.summary.info);
        } else {
            println!("Info: {}", result.summary.info);
        }

        println!();

        // Issues with colors
        if !result.issues.is_empty() {
            println!("Issues Found:");
            println!("{}", "-".repeat(30));

            for (i, issue) in result.issues.iter().enumerate() {
                print!("{}. ", i + 1);
                Self::print_colored_issue(issue);
            }
        } else {
            println!("\x1b[32mNo issues found!\x1b[0m");
        }
    }

    /// Print a single colored validation issue
    pub fn print_colored_issue(issue: &ValidationIssue) {
        let (color, severity_str) = match issue.severity {
            ValidationSeverity::Error => ("\x1b[31m", "ERROR"),
            ValidationSeverity::Warning => ("\x1b[33m", "WARNING"),
            ValidationSeverity::Info => ("\x1b[36m", "INFO"),
        };

        println!(
            "{}[{}]\x1b[0m {} ({}:{}): {}",
            color,
            severity_str,
            issue.code,
            issue.position.line,
            issue.position.column,
            issue.message
        );

        if let Some(suggestion) = &issue.suggestion {
            println!("   \x1b[32mSuggestion:\x1b[0m {}", suggestion);
        }

        if let Some(context) = issue.context.get("context_path") {
            println!("   \x1b[90mContext:\x1b[0m {}", context);
        }
    }
}
