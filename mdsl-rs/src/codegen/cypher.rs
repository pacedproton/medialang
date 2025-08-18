//! Cypher code generator for Neo4j graph database

use crate::error::Result;
use crate::ir::nodes::*;

/// Cypher code generator
pub struct CypherGenerator {
    /// Prefix for node labels and relationship types
    prefix: String,
}

impl CypherGenerator {
    /// Create a new Cypher generator with default prefix
    pub fn new() -> Self {
        Self {
            prefix: "mdsl".to_string(),
        }
    }

    /// Create a new Cypher generator with custom prefix
    pub fn with_prefix(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    /// Get the media outlet label based on prefix
    fn media_outlet_label(&self) -> String {
        if self.prefix.is_empty() {
            "media_outlet".to_string()
        } else {
            format!("{}_media_outlet", self.prefix)
        }
    }

    /// Get the family label based on prefix
    fn family_label(&self) -> String {
        if self.prefix.is_empty() {
            "Family".to_string()
        } else {
            format!("{}_Family", self.prefix)
        }
    }

    /// Get the relationship type based on prefix
    fn relationship_type(&self, rel_type: &str) -> String {
        if self.prefix.is_empty() {
            rel_type.to_string()
        } else {
            format!("{}_{}", self.prefix, rel_type)
        }
    }

    /// Get the constraint/index prefix for naming
    fn constraint_prefix(&self) -> String {
        if self.prefix.is_empty() {
            "".to_string()
        } else {
            format!("{}_", self.prefix)
        }
    }

    /// Get the template label based on prefix
    fn template_label(&self) -> String {
        if self.prefix.is_empty() {
            "Template".to_string()
        } else {
            format!("{}_Template", self.prefix)
        }
    }

    /// Get the vocabulary label based on prefix
    fn vocabulary_label(&self) -> String {
        if self.prefix.is_empty() {
            "Vocabulary".to_string()
        } else {
            format!("{}_Vocabulary", self.prefix)
        }
    }

    /// Get the market data label based on prefix
    fn market_data_label(&self) -> String {
        if self.prefix.is_empty() {
            "MarketData".to_string()
        } else {
            format!("{}_MarketData", self.prefix)
        }
    }

    /// Get the metric label based on prefix
    fn metric_label(&self) -> String {
        if self.prefix.is_empty() {
            "Metric".to_string()
        } else {
            format!("{}_Metric", self.prefix)
        }
    }

    /// Get the vocabulary entry label based on prefix
    fn vocabulary_entry_label(&self) -> String {
        if self.prefix.is_empty() {
            "VocabularyEntry".to_string()
        } else {
            format!("{}_VocabularyEntry", self.prefix)
        }
    }

    /// Get the characteristic label based on prefix
    fn characteristic_label(&self) -> String {
        if self.prefix.is_empty() {
            "Characteristic".to_string()
        } else {
            format!("{}_Characteristic", self.prefix)
        }
    }

    /// Get the metadata label based on prefix
    fn metadata_label(&self) -> String {
        if self.prefix.is_empty() {
            "Metadata".to_string()
        } else {
            format!("{}_Metadata", self.prefix)
        }
    }

    /// Get the event label based on prefix
    fn event_label(&self) -> String {
        if self.prefix.is_empty() {
            "Event".to_string()
        } else {
            format!("{}_Event", self.prefix)
        }
    }

    /// Get the event entity label based on prefix
    fn event_entity_label(&self) -> String {
        if self.prefix.is_empty() {
            "EventEntity".to_string()
        } else {
            format!("{}_EventEntity", self.prefix)
        }
    }

    /// Get the event impact label based on prefix
    fn event_impact_label(&self) -> String {
        if self.prefix.is_empty() {
            "EventImpact".to_string()
        } else {
            format!("{}_EventImpact", self.prefix)
        }
    }

    /// Get the event metadata label based on prefix
    fn event_metadata_label(&self) -> String {
        if self.prefix.is_empty() {
            "EventMetadata".to_string()
        } else {
            format!("{}_EventMetadata", self.prefix)
        }
    }

    /// Get the outlet label based on prefix (for data relationships)
    fn outlet_label(&self) -> String {
        if self.prefix.is_empty() {
            "Outlet".to_string()
        } else {
            format!("{}_Outlet", self.prefix)
        }
    }

    /// Get the data aggregation label based on prefix
    fn data_aggregation_label(&self) -> String {
        if self.prefix.is_empty() {
            "DataAggregation".to_string()
        } else {
            format!("{}_DataAggregation", self.prefix)
        }
    }

    /// Generate both schema and data Cypher files from IR
    pub fn generate_split(&self, ir: &IRProgram) -> Result<(String, String)> {
        let schema = self.generate_schema_only(ir)?;
        let data = self.generate_data_only(ir)?;
        Ok((schema, data))
    }

    /// Generate only schema (constraints and indexes) Cypher
    pub fn generate_schema_only(&self, _ir: &IRProgram) -> Result<String> {
        let mut cypher = String::new();
        
        // Add header comment
        cypher.push_str("// Generated Cypher Schema from MediaLanguage DSL\n");
        cypher.push_str("// This file contains ONLY constraint and index definitions\n");
        cypher.push_str("// Must be run before the data file in a separate transaction\n\n");
        
        // Generate only constraints and indexes
        cypher.push_str(&self.generate_constraints()?);
        
        Ok(cypher)
    }

    /// Generate only data (nodes and relationships) Cypher
    pub fn generate_data_only(&self, ir: &IRProgram) -> Result<String> {
        let mut cypher = String::new();
        
        // Add header comment
        cypher.push_str("// Generated Cypher Data from MediaLanguage DSL\n");
        cypher.push_str("// This file contains CREATE/MERGE statements for nodes and relationships\n");
        cypher.push_str("// Must be run AFTER the schema file in a separate transaction\n\n");

        // Generate imports as comments
        if !ir.imports.is_empty() {
            cypher.push_str("// IMPORTS\n");
            for import in &ir.imports {
                cypher.push_str(&format!("// IMPORT \"{}\"\n", import.path));
            }
            cypher.push_str("\n");
        }

        // Generate variables as comments
        if !ir.variables.is_empty() {
            cypher.push_str("// VARIABLES\n");
            for var in &ir.variables {
                cypher.push_str(&format!(
                    "// LET {} = {}\n",
                    var.name,
                    self.expression_to_cypher_comment(&var.value)
                ));
            }
            cypher.push_str("\n");
        }

        // Generate vocabulary nodes
        for vocab in &ir.vocabularies {
            cypher.push_str(&self.generate_vocabulary_nodes(vocab)?);
            cypher.push_str("\n");
        }

        // Generate template nodes
        for template in &ir.templates {
            cypher.push_str(&self.generate_template_nodes(template)?);
            cypher.push_str("\n");
        }

        // Generate family and outlet nodes
        for family in &ir.families {
            cypher.push_str(&self.generate_family_graph(family)?);
            cypher.push_str("\n");
        }

        // Generate relationships
        cypher.push_str(&self.generate_relationships(ir)?);

        // Generate data nodes
        cypher.push_str(&self.generate_data_nodes(ir)?);

        // Generate event nodes
        cypher.push_str(&self.generate_event_nodes(ir)?);

        Ok(cypher)
    }

    /// Generate Cypher code from IR (legacy single file)
    pub fn generate(&self, ir: &IRProgram) -> Result<String> {
        let mut cypher = String::new();

        // Add header comment
        cypher.push_str("// Generated Cypher from MediaLanguage DSL\n");
        cypher.push_str("// This file contains CREATE statements for Neo4j graph database\n");
        cypher.push_str("// Represents media outlets, families, and relationships as a graph\n\n");

        // Generate imports as comments
        if !ir.imports.is_empty() {
            cypher.push_str("// IMPORTS\n");
            for import in &ir.imports {
                cypher.push_str(&format!("// IMPORT \"{}\"\n", import.path));
            }
            cypher.push_str("\n");
        }

        // Generate variables as comments
        if !ir.variables.is_empty() {
            cypher.push_str("// VARIABLES\n");
            for var in &ir.variables {
                cypher.push_str(&format!(
                    "// LET {} = {}\n",
                    var.name,
                    self.expression_to_cypher_comment(&var.value)
                ));
            }
            cypher.push_str("\n");
        }

        // Generate constraints and indexes
        cypher.push_str(&self.generate_constraints()?);

        // Generate vocabulary nodes
        for vocab in &ir.vocabularies {
            cypher.push_str(&self.generate_vocabulary_nodes(vocab)?);
            cypher.push_str("\n");
        }

        // Generate template nodes
        for template in &ir.templates {
            cypher.push_str(&self.generate_template_nodes(template)?);
            cypher.push_str("\n");
        }

        // Generate family and outlet nodes
        for family in &ir.families {
            cypher.push_str(&self.generate_family_graph(family)?);
            cypher.push_str("\n");
        }

        // Generate relationships
        cypher.push_str(&self.generate_relationships(ir)?);

        // Generate data nodes
        cypher.push_str(&self.generate_data_nodes(ir)?);

        // Generate event nodes
        cypher.push_str(&self.generate_event_nodes(ir)?);

        Ok(cypher)
    }

    /// Generate constraints and indexes
    fn generate_constraints(&self) -> Result<String> {
        let mut cypher = String::new();

        cypher.push_str("// CONSTRAINTS AND INDEXES\n");
        cypher.push_str("// Create constraints for unique identifiers\n\n");

        // Constraints - updated for media_outlet schema with configurable prefix
        cypher.push_str(&format!("CREATE CONSTRAINT {}media_outlet_id_unique IF NOT EXISTS FOR (o:{}) REQUIRE o.id_mo IS UNIQUE;\n", self.constraint_prefix(), self.media_outlet_label()));
        cypher.push_str(&format!("CREATE CONSTRAINT {}family_name_unique IF NOT EXISTS FOR (f:{}) REQUIRE f.name IS UNIQUE;\n", self.constraint_prefix(), self.family_label()));
        cypher.push_str(&format!("CREATE CONSTRAINT {}template_name_unique IF NOT EXISTS FOR (t:{}) REQUIRE t.name IS UNIQUE;\n", self.constraint_prefix(), self.template_label()));
        cypher.push_str(&format!("CREATE CONSTRAINT {}vocab_name_unique IF NOT EXISTS FOR (v:{}) REQUIRE v.name IS UNIQUE;\n\n", self.constraint_prefix(), self.vocabulary_label()));

        // Indexes - updated for media_outlet schema with configurable prefix
        cypher.push_str(&format!("CREATE INDEX {}media_outlet_title_index IF NOT EXISTS FOR (o:{}) ON (o.mo_title);\n", self.constraint_prefix(), self.media_outlet_label()));
        cypher.push_str(&format!("CREATE INDEX {}family_name_index IF NOT EXISTS FOR (f:{}) ON (f.name);\n", self.constraint_prefix(), self.family_label()));
        cypher.push_str(&format!("CREATE INDEX {}data_year_index IF NOT EXISTS FOR (d:{}) ON (d.year);\n", self.constraint_prefix(), self.market_data_label()));
        cypher.push_str(&format!("CREATE INDEX {}metric_name_index IF NOT EXISTS FOR (m:{}) ON (m.name);\n\n", self.constraint_prefix(), self.metric_label()));

        Ok(cypher)
    }

    /// Generate vocabulary nodes
    fn generate_vocabulary_nodes(&self, vocab: &IRVocabulary) -> Result<String> {
        let mut cypher = String::new();

        cypher.push_str(&format!("// Vocabulary: {}\n", vocab.name));

        // Create vocabulary node (idempotent)
        cypher.push_str(&format!(
            "MERGE (v:{} {{name: '{}'}})\n",
            self.vocabulary_label(),
            vocab.name.replace("'", "\\'")
        ));
        cypher.push_str(&format!(
            "ON CREATE SET v.body_name = '{}', v.created_at = datetime()\n",
            vocab.body_name.replace("'", "\\'")
        ));
        cypher.push_str(&format!(
            "ON MATCH SET v.body_name = '{}';\n",
            vocab.body_name.replace("'", "\\'")
        ));

        // Create vocabulary entry nodes and relationships
        for entry in &vocab.entries {
            let key_str = match &entry.key {
                IRVocabularyKey::Number(n) => n.to_string(),
                IRVocabularyKey::String(s) => s.clone(),
            };

            cypher.push_str(&format!(
                "MERGE (e:{} {{key: '{}', vocab_name: '{}'}})\n",
                self.vocabulary_entry_label(),
                key_str.replace("'", "\\'"),
                vocab.name.replace("'", "\\'")
            ));
            cypher.push_str(&format!(
                "ON CREATE SET e.value = '{}'\n",
                entry.value.replace("'", "\\'")
            ));
            cypher.push_str(&format!(
                "ON MATCH SET e.value = '{}';\n",
                entry.value.replace("'", "\\'")
            ));

            cypher.push_str(&format!(
                "MATCH (v:{} {{name: '{}'}}), (e:{} {{key: '{}', vocab_name: '{}'}}) MERGE (v)-[:{} ]->(e);\n",
                self.vocabulary_label(),
                vocab.name.replace("'", "\\'"),
                self.vocabulary_entry_label(),
                key_str.replace("'", "\\'"),
                vocab.name.replace("'", "\\'"),
                self.relationship_type("HAS_ENTRY")
            ));
        }

        Ok(cypher)
    }

    /// Generate template nodes
    fn generate_template_nodes(&self, template: &IRTemplate) -> Result<String> {
        let mut cypher = String::new();

        cypher.push_str(&format!("// Template: {}\n", template.name));

        // Create template node
        cypher.push_str(&format!(
            "CREATE (t:{} {{name: '{}', type: '{}', created_at: datetime()}});\n",
            self.template_label(),
            template.name.replace("'", "\\'"),
            template.template_type.replace("'", "\\'")
        ));

        // Create characteristic and metadata nodes
        for block in &template.blocks {
            match block {
                IRTemplateBlock::Characteristics(chars) => {
                    for char in chars {
                        cypher.push_str(&format!(
                            "CREATE (c:{} {{name: '{}', value: '{}', template_name: '{}'}});\n",
                            self.characteristic_label(),
                            char.name.replace("'", "\\'"),
                            self.expression_to_cypher_value(&char.value).replace("'", "\\'"),
                            template.name.replace("'", "\\'")
                        ));

                        cypher.push_str(&format!(
                            "MATCH (t:{} {{name: '{}'}}), (c:{} {{name: '{}', template_name: '{}'}}) CREATE (t)-[:{} ]->(c);\n",
                            self.template_label(),
                            template.name.replace("'", "\\'"),
                            self.characteristic_label(),
                            char.name.replace("'", "\\'"),
                            template.name.replace("'", "\\'"),
                            self.relationship_type("HAS_CHARACTERISTIC")
                        ));
                    }
                }
                IRTemplateBlock::Metadata(meta) => {
                    for m in meta {
                        cypher.push_str(&format!(
                            "CREATE (m:{} {{name: '{}', value: '{}', template_name: '{}'}});\n",
                            self.metadata_label(),
                            m.name.replace("'", "\\'"),
                            self.expression_to_cypher_value(&m.value).replace("'", "\\'"),
                            template.name.replace("'", "\\'")
                        ));

                        cypher.push_str(&format!(
                            "MATCH (t:{} {{name: '{}'}}), (m:{} {{name: '{}', template_name: '{}'}}) CREATE (t)-[:{} ]->(m);\n",
                            self.template_label(),
                            template.name.replace("'", "\\'"),
                            self.metadata_label(),
                            m.name.replace("'", "\\'"),
                            template.name.replace("'", "\\'"),
                            self.relationship_type("HAS_METADATA")
                        ));
                    }
                }
            }
        }

        Ok(cypher)
    }

    /// Generate family graph
    fn generate_family_graph(&self, family: &IRFamily) -> Result<String> {
        let mut cypher = String::new();

        cypher.push_str(&format!("// Family: {}\n", family.name));

        // Create family node (idempotent)
        cypher.push_str(&format!(
            "MERGE (f:{} {{name: '{}'}})\n",
            self.family_label(),
            family.name.replace("'", "\\'")
        ));
        cypher.push_str(&format!(
            "ON CREATE SET f.comment = {}, f.created_at = datetime()\n",
            match &family.comment {
                Some(comment) => format!("'{}'", comment.replace("'", "\\'")),
                None => "null".to_string(),
            }
        ));
        cypher.push_str(&format!(
            "ON MATCH SET f.comment = {};\n",
            match &family.comment {
                Some(comment) => format!("'{}'", comment.replace("'", "\\'")),
                None => "null".to_string(),
            }
        ));

        // Create outlet nodes
        for outlet in &family.outlets {
            cypher.push_str(&self.generate_outlet_node(outlet, &family.name)?);
        }

        Ok(cypher)
    }

    /// Generate outlet node
    fn generate_outlet_node(&self, outlet: &IROutlet, family_name: &str) -> Result<String> {
        let mut cypher = String::new();

        cypher.push_str(&format!("// Outlet: {}\n", outlet.name));

        // Create media_outlet node with proper schema and configurable prefix (idempotent)
        let outlet_id = outlet.id.unwrap_or(0);
        cypher.push_str(&format!(
            "MERGE (o:{} {{id_mo: {}}})\n",
            self.media_outlet_label(),
            outlet_id
        ));
        cypher.push_str(&format!(
            "ON CREATE SET o.mo_title = '{}', o.id_sector = 2, o.mandate = 1, o.location = 'Wien', o.primary_distr_area = 1, o.local = 0, o.language = 'deutsch', o.editorial_line_s = 'Öffentlich-rechtlich', o.comments = 'Generated from MDSL'\n",
            outlet.name.replace("'", "\\'")
        ));
        cypher.push_str(&format!(
            "ON MATCH SET o.mo_title = '{}';\n",
            outlet.name.replace("'", "\\'")
        ));

        // Set default dates for all outlets - these will be overridden by lifecycle data if present
        cypher.push_str(&format!(
            "MATCH (o:{} {{id_mo: {}}}) SET o.start_date = datetime('1955-01-01'), o.end_date = datetime('9999-01-01');\n",
            self.media_outlet_label(),
            outlet_id
        ));

        // Connect to family (keeping family structure for organization) - idempotent
        cypher.push_str(&format!(
            "MATCH (f:{} {{name: '{}'}}), (o:{} {{id_mo: {}}}) MERGE (f)-[:{}]->(o);\n",
            self.family_label(),
            family_name.replace("'", "\\'"),
            self.media_outlet_label(),
            outlet.id.unwrap_or(0),
            self.relationship_type("HAS_OUTLET")
        ));

        // Create template relationship if exists
        if let Some(template_ref) = &outlet.template_ref {
            cypher.push_str(&format!(
                "MATCH (t:{} {{name: '{}'}}), (o:{} {{id_mo: {}}}) MERGE (o)-[:{} ]->(t);\n",
                self.template_label(),
                template_ref.replace("'", "\\'"),
                self.media_outlet_label(),
                outlet.id.unwrap_or(0),
                self.relationship_type("EXTENDS_TEMPLATE")
            ));
        }

        // Create base relationship if exists
        if let Some(base_ref) = outlet.base_ref {
            cypher.push_str(&format!(
                "MATCH (base:{} {{id_mo: {}}}), (o:{} {{id_mo: {}}}) MERGE (o)-[:{} ]->(base);\n",
                self.media_outlet_label(),
                base_ref,
                self.media_outlet_label(),
                outlet.id.unwrap_or(0),
                self.relationship_type("BASED_ON")
            ));
        }

        // Process outlet blocks - simplified for {}_media_outlet compatibility
        for block in &outlet.blocks {
            match block {
                IROutletBlock::Identity(fields) => {
                    // Add identity fields as properties to the {}_media_outlet node
                    for field in fields {
                        match field.name.as_str() {
                            "title" => {
                                // Update mo_title if specified
                                cypher.push_str(&format!(
                                    "MATCH (o:{} {{id_mo: {}}}) SET o.mo_title = '{}';\n",
                                    self.media_outlet_label(),
                                    outlet.id.unwrap_or(0),
                                    self.expression_to_cypher_value(&field.value)
                                        .replace("'", "\\'")
                                ));
                            }
                            "url" => {
                                // Add as comment if it's a URL
                                cypher.push_str(&format!(
                                    "MATCH (o:{} {{id_mo: {}}}) SET o.comments = COALESCE(o.comments, '') + ' URL: {}';\n",
                                    self.media_outlet_label(),
                                    outlet.id.unwrap_or(0),
                                    self.expression_to_cypher_value(&field.value).replace("'", "\\'")
                                ));
                            }
                            _ => {
                                // Add other identity fields as comments
                                cypher.push_str(&format!(
                                    "MATCH (o:{} {{id_mo: {}}}) SET o.comments = COALESCE(o.comments, '') + ' {}: {}';\n",
                                    self.media_outlet_label(),
                                    outlet.id.unwrap_or(0),
                                    field.name.replace("'", "\\'"),
                                    self.expression_to_cypher_value(&field.value).replace("'", "\\'")
                                ));
                            }
                        }
                    }
                }
                IROutletBlock::Characteristics(fields) => {
                    // Map characteristics to {}_media_outlet schema
                    for field in fields {
                        match field.name.as_str() {
                            "sector" => {
                                if let Some(sector_val) =
                                    self.extract_number_from_expression(&field.value)
                                {
                                    cypher.push_str(&format!(
                                        "MATCH (o:{} {{id_mo: {}}}) SET o.id_sector = {};\n",
                                        self.media_outlet_label(),
                                        outlet.id.unwrap_or(0),
                                        sector_val
                                    ));
                                }
                            }
                            "mandate" => {
                                if let Some(mandate_val) =
                                    self.extract_number_from_expression(&field.value)
                                {
                                    cypher.push_str(&format!(
                                        "MATCH (o:{} {{id_mo: {}}}) SET o.mandate = {};\n",
                                        self.media_outlet_label(),
                                        outlet.id.unwrap_or(0),
                                        mandate_val
                                    ));
                                }
                            }
                            "language" => {
                                cypher.push_str(&format!(
                                    "MATCH (o:{} {{id_mo: {}}}) SET o.language = '{}';\n",
                                    self.media_outlet_label(),
                                    outlet.id.unwrap_or(0),
                                    self.expression_to_cypher_value(&field.value)
                                        .replace("'", "\\'")
                                ));
                            }
                            _ => {
                                // Add other characteristics as comments
                                cypher.push_str(&format!(
                                    "MATCH (o:{} {{id_mo: {}}}) SET o.comments = COALESCE(o.comments, '') + ' {}: {}';\n",
                                    self.media_outlet_label(),
                                    outlet.id.unwrap_or(0),
                                    field.name.replace("'", "\\'"),
                                    self.expression_to_cypher_value(&field.value).replace("'", "\\'")
                                ));
                            }
                        }
                    }
                }
                IROutletBlock::Lifecycle(entries) => {
                    // Handle lifecycle entries - extract dates
                    for entry in entries {
                        if let Some(start_date) = &entry.start_date {
                            cypher.push_str(&format!(
                                "MATCH (o:{} {{id_mo: {}}}) SET o.start_date = datetime('{}');\n",
                                self.media_outlet_label(),
                                outlet.id.unwrap_or(0),
                                start_date
                            ));
                        }
                        if let Some(end_date) = &entry.end_date {
                            let end_date_str = if end_date.to_lowercase() == "current" {
                                "9999-01-01".to_string()
                            } else {
                                end_date.clone()
                            };
                            cypher.push_str(&format!(
                                "MATCH (o:{} {{id_mo: {}}}) SET o.end_date = datetime('{}');\n",
                                self.media_outlet_label(),
                                outlet.id.unwrap_or(0),
                                end_date_str
                            ));
                        }
                    }
                }
                IROutletBlock::Metadata(fields) => {
                    // Handle metadata - add to comments
                    for field in fields {
                        if field.name == "comment" {
                            cypher.push_str(&format!(
                                "MATCH (o:{} {{id_mo: {}}}) SET o.comments = '{}';\n",
                                self.media_outlet_label(),
                                outlet.id.unwrap_or(0),
                                self.expression_to_cypher_value(&field.value)
                                    .replace("'", "\\'")
                            ));
                        }
                    }
                }
            }
        }

        Ok(cypher)
    }

    /// Generate relationships
    fn generate_relationships(&self, ir: &IRProgram) -> Result<String> {
        let mut cypher = String::new();

        cypher.push_str("// RELATIONSHIPS\n");

        for family in &ir.families {
            for relationship in &family.relationships {
                match relationship {
                    IRRelationship::Diachronic(diachronic) => {
                        cypher.push_str(&format!(
                            "// Diachronic relationship: {}\n",
                            diachronic.name
                        ));
                        // Use the relationship type directly as the relationship name
                        let rel_type = if diachronic.relationship_type.is_empty() {
                            "RELATED_TO".to_string()
                        } else {
                            diachronic.relationship_type.replace("'", "").replace("-", "_")
                        };
                        cypher.push_str(&format!(
                            "MATCH (pred:{} {{id_mo: {}}}), (succ:{} {{id_mo: {}}}) MERGE (pred)-[r:{}]->(succ) SET r.event_rel = datetime('{}');\n",
                            self.media_outlet_label(),
                            diachronic.predecessor,
                            self.media_outlet_label(),
                            diachronic.successor,
                            self.relationship_type(&rel_type),
                            diachronic.event_start_date.as_ref().unwrap_or(&"1900-01-01".to_string())
                        ));
                    }
                    IRRelationship::Synchronous(sync) => {
                        cypher.push_str(&format!("// Synchronous relationship: {}\n", sync.name));
                        // Use the relationship type directly as the relationship name
                        let rel_type = if sync.relationship_type.is_empty() {
                            "RELATED_TO".to_string()
                        } else {
                            sync.relationship_type.replace("'", "").replace("-", "_")
                        };
                        cypher.push_str(&format!(
                            "MATCH (o1:{} {{id_mo: {}}}), (o2:{} {{id_mo: {}}}) MERGE (o1)-[r:{}]->(o2) SET r.start_rel = datetime('{}'), r.end_rel = datetime('{}');\n",
                            self.media_outlet_label(),
                            sync.outlet_1.id,
                            self.media_outlet_label(),
                            sync.outlet_2.id,
                            self.relationship_type(&rel_type),
                            sync.period_start.as_ref().unwrap_or(&"1900-01-01".to_string()),
                            sync.period_end.as_ref().unwrap_or(&"9999-01-01".to_string())
                        ));
                    }
                }
            }
        }

        Ok(cypher)
    }

    /// Generate data nodes
    fn generate_data_nodes(&self, ir: &IRProgram) -> Result<String> {
        let mut cypher = String::new();

        cypher.push_str("// MARKET DATA\n");

        for family in &ir.families {
            for data_block in &family.data_blocks {
                // Create data aggregation nodes
                for agg in &data_block.aggregation {
                    cypher.push_str(&format!(
                        "CREATE (a:{} {{name: '{}', value: '{}', outlet_id: {}}});\n",
                        self.data_aggregation_label(),
                        agg.name.replace("'", "\\'"),
                        agg.value.replace("'", "\\'"),
                        data_block.outlet_id
                    ));

                    cypher.push_str(&format!(
                        "MATCH (o:{} {{id: {}}}), (a:{} {{name: '{}', outlet_id: {}}}) CREATE (o)-[:{} ]->(a);\n",
                        self.outlet_label(),
                        data_block.outlet_id,
                        self.data_aggregation_label(),
                        agg.name.replace("'", "\\'"),
                        data_block.outlet_id,
                        self.relationship_type("HAS_AGGREGATION")
                    ));
                }

                // Create market data nodes
                for year in &data_block.years {
                    cypher.push_str(&format!(
                        "CREATE (d:{} {{year: {}, outlet_id: {}, comment: {}, maps_to: {}}});\n",
                        self.market_data_label(),
                        year.year,
                        data_block.outlet_id,
                        self.optional_string_to_cypher(&year.comment),
                        self.optional_string_to_cypher(&data_block.maps_to)
                    ));

                    cypher.push_str(&format!(
                        "MATCH (o:{} {{id: {}}}), (d:{} {{year: {}, outlet_id: {}}}) CREATE (o)-[:{} ]->(d);\n",
                        self.outlet_label(),
                        data_block.outlet_id,
                        self.market_data_label(),
                        year.year,
                        data_block.outlet_id,
                        self.relationship_type("HAS_DATA")
                    ));

                    // Create metric nodes
                    for metric in &year.metrics {
                        cypher.push_str(&format!(
                            "CREATE (m:{} {{name: '{}', value: {}, unit: '{}', source: '{}', comment: {}, year: {}, outlet_id: {}}});\n",
                            self.metric_label(),
                            metric.name.replace("'", "\\'"),
                            metric.value,
                            metric.unit.replace("'", "\\'"),
                            metric.source.replace("'", "\\'"),
                            self.optional_string_to_cypher(&metric.comment),
                            year.year,
                            data_block.outlet_id
                        ));

                        cypher.push_str(&format!(
                            "MATCH (d:{} {{year: {}, outlet_id: {}}}), (m:{} {{name: '{}', year: {}, outlet_id: {}}}) CREATE (d)-[:{} ]->(m);\n",
                            self.market_data_label(),
                            year.year,
                            data_block.outlet_id,
                            self.metric_label(),
                            metric.name.replace("'", "\\'"),
                            year.year,
                            data_block.outlet_id,
                            self.relationship_type("HAS_METRIC")
                        ));
                    }
                }
            }
        }

        Ok(cypher)
    }

    /// Helper function to convert expression to Cypher comment
    fn expression_to_cypher_comment(&self, expr: &IRExpression) -> String {
        match expr {
            IRExpression::String(s) => format!("\"{}\"", s),
            IRExpression::Number(n) => n.to_string(),
            IRExpression::Boolean(b) => b.to_string(),
            IRExpression::Variable(v) => format!("${}", v),
            IRExpression::Object(_) => "object".to_string(),
            IRExpression::Array(_) => "array".to_string(),
        }
    }

    /// Helper function to convert expression to Cypher value
    fn expression_to_cypher_value(&self, expr: &IRExpression) -> String {
        match expr {
            IRExpression::String(s) => s.clone(),
            IRExpression::Number(n) => n.to_string(),
            IRExpression::Boolean(b) => b.to_string(),
            IRExpression::Variable(v) => format!("${}", v),
            IRExpression::Object(_) => "{}".to_string(),
            IRExpression::Array(_) => "[]".to_string(),
        }
    }


    /// Helper function to convert optional string to Cypher
    fn optional_string_to_cypher(&self, s: &Option<String>) -> String {
        match s {
            Some(text) => format!("'{}'", text.replace("'", "\\'")),
            None => "null".to_string(),
        }
    }

    /// Extract number from expression for schema mapping
    fn extract_number_from_expression(&self, expr: &IRExpression) -> Option<i64> {
        match expr {
            IRExpression::Number(n) => Some(*n as i64),
            IRExpression::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Generate event nodes
    fn generate_event_nodes(&self, ir: &IRProgram) -> Result<String> {
        let mut cypher = String::new();

        if !ir.events.is_empty() {
            cypher.push_str("// EVENTS\n");

            for event in &ir.events {
                // Create event node
                cypher.push_str(&format!(
                    "CREATE (e:{} {{name: '{}', type: '{}', date: {}, status: {}, created_at: datetime()}});\n",
                    self.event_label(),
                    event.name.replace("'", "\\'"),
                    event.event_type.replace("'", "\\'"),
                    match &event.date {
                        Some(date) => {
                            if date == "CURRENT" {
                                "datetime()".to_string()
                            } else {
                                format!("datetime('{}')", date.replace("'", "\\'"))
                            }
                        }
                        None => "null".to_string(),
                    },
                    self.optional_string_to_cypher(&event.status)
                ));

                // Create event entity nodes and relationships
                for entity in &event.entities {
                    cypher.push_str(&format!(
                        "CREATE (ee:{} {{name: '{}', entity_id: {}, role: '{}', stake_before: {}, stake_after: {}, event_name: '{}'}});\n",
                        self.event_entity_label(),
                        entity.name.replace("'", "\\'"),
                        entity.id,
                        entity.role.replace("'", "\\'"),
                        entity.stake_before.map(|s| s.to_string()).unwrap_or("null".to_string()),
                        entity.stake_after.map(|s| s.to_string()).unwrap_or("null".to_string()),
                        event.name.replace("'", "\\'")
                    ));

                    // Connect event entity to event
                    cypher.push_str(&format!(
                        "MATCH (e:{} {{name: '{}'}}), (ee:{} {{name: '{}', event_name: '{}'}}) CREATE (e)-[:{} ]->(ee);\n",
                        self.event_label(),
                        event.name.replace("'", "\\'"),
                        self.event_entity_label(),
                        entity.name.replace("'", "\\'"),
                        event.name.replace("'", "\\'"),
                        self.relationship_type("HAS_ENTITY")
                    ));

                    // Connect event entity to media outlet if possible
                    cypher.push_str(&format!(
                        "MATCH (mo:{} {{id_mo: {}}}), (ee:{} {{entity_id: {}, event_name: '{}'}}) CREATE (ee)-[:{} ]->(mo);\n",
                        self.media_outlet_label(),
                        entity.id,
                        self.event_entity_label(),
                        entity.id,
                        event.name.replace("'", "\\'"),
                        self.relationship_type("INVOLVES")
                    ));
                }

                // Create event impact nodes
                for impact in &event.impact {
                    cypher.push_str(&format!(
                        "CREATE (ei:{} {{name: '{}', value: '{}', event_name: '{}'}});\n",
                        self.event_impact_label(),
                        impact.name.replace("'", "\\'"),
                        self.expression_to_cypher_value(&impact.value).replace("'", "\\'"),
                        event.name.replace("'", "\\'")
                    ));

                    cypher.push_str(&format!(
                        "MATCH (e:{} {{name: '{}'}}), (ei:{} {{name: '{}', event_name: '{}'}}) CREATE (e)-[:{} ]->(ei);\n",
                        self.event_label(),
                        event.name.replace("'", "\\'"),
                        self.event_impact_label(),
                        impact.name.replace("'", "\\'"),
                        event.name.replace("'", "\\'"),
                        self.relationship_type("HAS_IMPACT")
                    ));
                }

                // Create event metadata nodes
                for metadata in &event.metadata {
                    cypher.push_str(&format!(
                        "CREATE (em:{} {{name: '{}', value: '{}', event_name: '{}'}});\n",
                        self.event_metadata_label(),
                        metadata.name.replace("'", "\\'"),
                        self.expression_to_cypher_value(&metadata.value).replace("'", "\\'"),
                        event.name.replace("'", "\\'")
                    ));

                    cypher.push_str(&format!(
                        "MATCH (e:{} {{name: '{}'}}), (em:{} {{name: '{}', event_name: '{}'}}) CREATE (e)-[:{} ]->(em);\n",
                        self.event_label(),
                        event.name.replace("'", "\\'"),
                        self.event_metadata_label(),
                        metadata.name.replace("'", "\\'"),
                        event.name.replace("'", "\\'"),
                        self.relationship_type("HAS_METADATA")
                    ));
                }
            }

            cypher.push_str("\n");
        }

        Ok(cypher)
    }
}
