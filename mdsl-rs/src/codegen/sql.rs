//! SQL code generator

use crate::error::Result;
use crate::ir::nodes::*;

/// SQL code generator
pub struct SqlGenerator;

impl SqlGenerator {
    /// Create a new SQL generator
    pub fn new() -> Self {
        Self
    }

    /// Generate SQL code from IR
    pub fn generate(&self, ir: &IRProgram) -> Result<String> {
        let mut sql = String::new();

        // Add header comment
        sql.push_str("-- Generated SQL from MediaLanguage DSL\n");
        sql.push_str(
            "-- This file contains CREATE TABLE statements, INSERT statements, and constraints\n",
        );
        sql.push_str("-- Generated for comprehensive media outlet and relationship management\n\n");

        // Generate imports as comments
        if !ir.imports.is_empty() {
            sql.push_str("-- IMPORTS\n");
            for import in &ir.imports {
                sql.push_str(&format!("-- IMPORT \"{}\"\n", import.path));
            }
            sql.push_str("\n");
        }

        // Generate variables as comments
        if !ir.variables.is_empty() {
            sql.push_str("-- VARIABLES\n");
            for var in &ir.variables {
                sql.push_str(&format!(
                    "-- LET {} = {}\n",
                    var.name,
                    self.expression_to_sql_comment(&var.value)
                ));
            }
            sql.push_str("\n");
        }

        // Generate core schema tables
        sql.push_str(&self.generate_core_schema()?);

        // Generate CREATE TABLE statements for units
        for unit in &ir.units {
            sql.push_str(&self.generate_create_table(unit)?);
            sql.push_str("\n");
        }

        // Generate vocabulary tables
        for vocab in &ir.vocabularies {
            sql.push_str(&self.generate_vocabulary_table(vocab)?);
            sql.push_str("\n");
        }

        // Generate template tables
        for template in &ir.templates {
            sql.push_str(&self.generate_template_table(template)?);
            sql.push_str("\n");
        }

        // Generate family and outlet tables
        for family in &ir.families {
            sql.push_str(&self.generate_family_tables(family)?);
            sql.push_str("\n");
        }

        // Generate relationship tables
        sql.push_str(&self.generate_relationship_tables(ir)?);

        // Generate data insertion statements
        sql.push_str(&self.generate_data_inserts(ir)?);

        // Generate event insertion statements
        sql.push_str(&self.generate_event_inserts(ir)?);

        Ok(sql)
    }

    /// Generate core schema tables
    fn generate_core_schema(&self) -> Result<String> {
        let mut sql = String::new();

        sql.push_str("-- CORE SCHEMA TABLES\n");
        sql.push_str("-- These tables support the MediaLanguage DSL structure\n\n");

        // Media outlets table
        sql.push_str("CREATE TABLE media_outlets (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    name VARCHAR(255) NOT NULL,\n");
        sql.push_str("    family_id INTEGER,\n");
        sql.push_str("    template_id INTEGER,\n");
        sql.push_str("    base_outlet_id INTEGER,\n");
        sql.push_str("    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,\n");
        sql.push_str("    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,\n");
        sql.push_str("    FOREIGN KEY (family_id) REFERENCES families(id),\n");
        sql.push_str("    FOREIGN KEY (template_id) REFERENCES templates(id),\n");
        sql.push_str("    FOREIGN KEY (base_outlet_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Families table
        sql.push_str("CREATE TABLE families (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    name VARCHAR(255) NOT NULL,\n");
        sql.push_str("    comment TEXT,\n");
        sql.push_str("    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n");
        sql.push_str(");\n\n");

        // Templates table
        sql.push_str("CREATE TABLE templates (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    name VARCHAR(255) NOT NULL,\n");
        sql.push_str("    template_type VARCHAR(100) NOT NULL,\n");
        sql.push_str("    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n");
        sql.push_str(");\n\n");

        // Outlet identity table
        sql.push_str("CREATE TABLE outlet_identity (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    outlet_id INTEGER NOT NULL,\n");
        sql.push_str("    field_name VARCHAR(100) NOT NULL,\n");
        sql.push_str("    field_value TEXT,\n");
        sql.push_str("    field_type VARCHAR(50) DEFAULT 'string',\n");
        sql.push_str("    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Outlet lifecycle table
        sql.push_str("CREATE TABLE outlet_lifecycle (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    outlet_id INTEGER NOT NULL,\n");
        sql.push_str("    status VARCHAR(100) NOT NULL,\n");
        sql.push_str("    start_date DATE,\n");
        sql.push_str("    end_date DATE,\n");
        sql.push_str("    precision_start VARCHAR(50),\n");
        sql.push_str("    precision_end VARCHAR(50),\n");
        sql.push_str("    comment TEXT,\n");
        sql.push_str("    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Outlet characteristics table
        sql.push_str("CREATE TABLE outlet_characteristics (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    outlet_id INTEGER NOT NULL,\n");
        sql.push_str("    characteristic_name VARCHAR(100) NOT NULL,\n");
        sql.push_str("    characteristic_value TEXT,\n");
        sql.push_str("    characteristic_type VARCHAR(50) DEFAULT 'string',\n");
        sql.push_str("    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Outlet metadata table
        sql.push_str("CREATE TABLE outlet_metadata (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    outlet_id INTEGER NOT NULL,\n");
        sql.push_str("    metadata_name VARCHAR(100) NOT NULL,\n");
        sql.push_str("    metadata_value TEXT,\n");
        sql.push_str("    metadata_type VARCHAR(50) DEFAULT 'string',\n");
        sql.push_str("    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Relationships table
        sql.push_str("CREATE TABLE relationships (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    relationship_name VARCHAR(255) NOT NULL,\n");
        sql.push_str(
            "    relationship_type VARCHAR(50) NOT NULL, -- 'diachronic' or 'synchronous'\n",
        );
        sql.push_str("    family_id INTEGER,\n");
        sql.push_str("    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,\n");
        sql.push_str("    FOREIGN KEY (family_id) REFERENCES families(id)\n");
        sql.push_str(");\n\n");

        // Diachronic relationships table
        sql.push_str("CREATE TABLE diachronic_relationships (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    relationship_id INTEGER NOT NULL,\n");
        sql.push_str("    predecessor_id INTEGER NOT NULL,\n");
        sql.push_str("    successor_id INTEGER NOT NULL,\n");
        sql.push_str("    event_start_date DATE,\n");
        sql.push_str("    event_end_date DATE,\n");
        sql.push_str("    relationship_subtype VARCHAR(100),\n");
        sql.push_str("    comment TEXT,\n");
        sql.push_str("    maps_to VARCHAR(255),\n");
        sql.push_str("    FOREIGN KEY (relationship_id) REFERENCES relationships(id),\n");
        sql.push_str("    FOREIGN KEY (predecessor_id) REFERENCES media_outlets(id),\n");
        sql.push_str("    FOREIGN KEY (successor_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Synchronous relationships table
        sql.push_str("CREATE TABLE synchronous_relationships (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    relationship_id INTEGER NOT NULL,\n");
        sql.push_str("    outlet_1_id INTEGER NOT NULL,\n");
        sql.push_str("    outlet_1_role VARCHAR(100),\n");
        sql.push_str("    outlet_2_id INTEGER NOT NULL,\n");
        sql.push_str("    outlet_2_role VARCHAR(100),\n");
        sql.push_str("    relationship_subtype VARCHAR(100),\n");
        sql.push_str("    period_start DATE,\n");
        sql.push_str("    period_end DATE,\n");
        sql.push_str("    details TEXT,\n");
        sql.push_str("    maps_to VARCHAR(255),\n");
        sql.push_str("    FOREIGN KEY (relationship_id) REFERENCES relationships(id),\n");
        sql.push_str("    FOREIGN KEY (outlet_1_id) REFERENCES media_outlets(id),\n");
        sql.push_str("    FOREIGN KEY (outlet_2_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Market data table
        sql.push_str("CREATE TABLE market_data (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    outlet_id INTEGER NOT NULL,\n");
        sql.push_str("    data_year INTEGER NOT NULL,\n");
        sql.push_str("    metric_name VARCHAR(100) NOT NULL,\n");
        sql.push_str("    metric_value DECIMAL(15,2),\n");
        sql.push_str("    metric_unit VARCHAR(50),\n");
        sql.push_str("    data_source VARCHAR(100),\n");
        sql.push_str("    comment TEXT,\n");
        sql.push_str("    maps_to VARCHAR(255),\n");
        sql.push_str("    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,\n");
        sql.push_str("    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Data aggregation settings table
        sql.push_str("CREATE TABLE data_aggregation (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    outlet_id INTEGER NOT NULL,\n");
        sql.push_str("    aggregation_name VARCHAR(100) NOT NULL,\n");
        sql.push_str("    aggregation_value VARCHAR(100) NOT NULL,\n");
        sql.push_str("    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Events table
        sql.push_str("CREATE TABLE events (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    name VARCHAR(255) NOT NULL,\n");
        sql.push_str("    event_type VARCHAR(100) NOT NULL,\n");
        sql.push_str("    event_date DATE,\n");
        sql.push_str("    status VARCHAR(100),\n");
        sql.push_str("    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n");
        sql.push_str(");\n\n");

        // Event entities table
        sql.push_str("CREATE TABLE event_entities (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    event_id INTEGER NOT NULL,\n");
        sql.push_str("    entity_name VARCHAR(255) NOT NULL,\n");
        sql.push_str("    entity_id INTEGER NOT NULL,\n");
        sql.push_str("    entity_role VARCHAR(100),\n");
        sql.push_str("    stake_before DECIMAL(10,2),\n");
        sql.push_str("    stake_after DECIMAL(10,2),\n");
        sql.push_str("    FOREIGN KEY (event_id) REFERENCES events(id),\n");
        sql.push_str("    FOREIGN KEY (entity_id) REFERENCES media_outlets(id)\n");
        sql.push_str(");\n\n");

        // Event impact table
        sql.push_str("CREATE TABLE event_impact (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    event_id INTEGER NOT NULL,\n");
        sql.push_str("    impact_name VARCHAR(100) NOT NULL,\n");
        sql.push_str("    impact_value TEXT,\n");
        sql.push_str("    impact_type VARCHAR(50) DEFAULT 'string',\n");
        sql.push_str("    FOREIGN KEY (event_id) REFERENCES events(id)\n");
        sql.push_str(");\n\n");

        // Event metadata table
        sql.push_str("CREATE TABLE event_metadata (\n");
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    event_id INTEGER NOT NULL,\n");
        sql.push_str("    metadata_name VARCHAR(100) NOT NULL,\n");
        sql.push_str("    metadata_value TEXT,\n");
        sql.push_str("    metadata_type VARCHAR(50) DEFAULT 'string',\n");
        sql.push_str("    FOREIGN KEY (event_id) REFERENCES events(id)\n");
        sql.push_str(");\n\n");

        Ok(sql)
    }

    /// Generate CREATE TABLE statement for a unit
    fn generate_create_table(&self, unit: &IRUnit) -> Result<String> {
        let mut sql = format!("-- Table for unit: {}\n", unit.name);
        sql.push_str(&format!("CREATE TABLE {} (\n", unit.name.to_lowercase()));

        let mut field_definitions = Vec::new();

        for field in &unit.fields {
            let field_def = self.generate_field_definition(field)?;
            field_definitions.push(field_def);
        }

        sql.push_str(&field_definitions.join(",\n"));
        sql.push_str("\n);\n");

        Ok(sql)
    }

    /// Generate field definition
    fn generate_field_definition(&self, field: &IRField) -> Result<String> {
        let mut def = format!("    {}", field.name);

        // Add field type
        match &field.field_type {
            IRFieldType::Id => {
                def.push_str(" INTEGER");
            }
            IRFieldType::Text(length) => match length {
                Some(len) => def.push_str(&format!(" VARCHAR({})", len)),
                None => def.push_str(" TEXT"),
            },
            IRFieldType::Number => {
                def.push_str(" DECIMAL(15,2)");
            }
            IRFieldType::Boolean => {
                def.push_str(" BOOLEAN");
            }
            IRFieldType::Category(_) => {
                def.push_str(" VARCHAR(100)");
            }
        }

        // Add primary key constraint
        if field.is_primary_key {
            def.push_str(" PRIMARY KEY");
        }

        // Add NOT NULL constraint for primary keys
        if field.is_primary_key {
            def.push_str(" NOT NULL");
        }

        Ok(def)
    }

    /// Generate vocabulary table
    fn generate_vocabulary_table(&self, vocab: &IRVocabulary) -> Result<String> {
        let mut sql = format!("-- Vocabulary table: {}\n", vocab.name);
        sql.push_str(&format!("CREATE TABLE {} (\n", vocab.name.to_lowercase()));
        sql.push_str("    id INTEGER PRIMARY KEY,\n");
        sql.push_str("    code VARCHAR(50) NOT NULL,\n");
        sql.push_str("    description TEXT NOT NULL,\n");
        sql.push_str("    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n");
        sql.push_str(");\n\n");

        // Generate INSERT statements for vocabulary entries
        sql.push_str(&format!("-- Insert vocabulary data for {}\n", vocab.name));
        for (index, entry) in vocab.entries.iter().enumerate() {
            let key_str = match &entry.key {
                IRVocabularyKey::Number(n) => n.to_string(),
                IRVocabularyKey::String(s) => s.clone(),
            };
            sql.push_str(&format!(
                "INSERT INTO {} (id, code, description) VALUES ({}, '{}', '{}');\n",
                vocab.name.to_lowercase(),
                index + 1,
                key_str.replace("'", "''"),
                entry.value.replace("'", "''")
            ));
        }

        Ok(sql)
    }

    /// Generate template table
    fn generate_template_table(&self, template: &IRTemplate) -> Result<String> {
        let mut sql = format!("-- Template: {}\n", template.name);
        sql.push_str(&format!(
            "INSERT INTO templates (name, template_type) VALUES ('{}', '{}');\n",
            template.name.replace("'", "''"),
            template.template_type.replace("'", "''")
        ));

        // Template characteristics and metadata would be stored in separate tables
        // For now, we'll generate comments about the template structure
        for block in &template.blocks {
            match block {
                IRTemplateBlock::Characteristics(chars) => {
                    sql.push_str(&format!("-- Template {} characteristics:\n", template.name));
                    for char in chars {
                        sql.push_str(&format!(
                            "--   {}: {}\n",
                            char.name,
                            self.expression_to_sql_comment(&char.value)
                        ));
                    }
                }
                IRTemplateBlock::Metadata(meta) => {
                    sql.push_str(&format!("-- Template {} metadata:\n", template.name));
                    for m in meta {
                        sql.push_str(&format!(
                            "--   {}: {}\n",
                            m.name,
                            self.expression_to_sql_comment(&m.value)
                        ));
                    }
                }
            }
        }

        Ok(sql)
    }

    /// Generate family tables
    fn generate_family_tables(&self, family: &IRFamily) -> Result<String> {
        let mut sql = String::new();

        sql.push_str(&format!("-- Family: {}\n", family.name));
        sql.push_str(&format!(
            "INSERT INTO families (name, comment) VALUES ('{}', {});\n",
            family.name.replace("'", "''"),
            match &family.comment {
                Some(comment) => format!("'{}'", comment.replace("'", "''")),
                None => "NULL".to_string(),
            }
        ));

        // Generate outlets for this family
        for outlet in &family.outlets {
            sql.push_str(&self.generate_outlet_data(outlet, &family.name)?);
        }

        Ok(sql)
    }

    /// Generate outlet data
    fn generate_outlet_data(&self, outlet: &IROutlet, family_name: &str) -> Result<String> {
        let mut sql = String::new();

        sql.push_str(&format!("-- Outlet: {}\n", outlet.name));

        // Insert into media_outlets table
        sql.push_str(&format!(
            "INSERT INTO media_outlets (id, name, family_id) VALUES ({}, '{}', (SELECT id FROM families WHERE name = '{}'));\n",
            outlet.id.unwrap_or(0),
            outlet.name.replace("'", "''"),
            family_name.replace("'", "''")
        ));

        // Generate data for outlet blocks
        for block in &outlet.blocks {
            match block {
                IROutletBlock::Identity(fields) => {
                    for field in fields {
                        sql.push_str(&format!(
                            "INSERT INTO outlet_identity (outlet_id, field_name, field_value) VALUES ({}, '{}', '{}');\n",
                            outlet.id.unwrap_or(0),
                            field.name.replace("'", "''"),
                            self.expression_to_sql_value(&field.value).replace("'", "''")
                        ));
                    }
                }
                IROutletBlock::Lifecycle(statuses) => {
                    for status in statuses {
                        sql.push_str(&format!(
                            "INSERT INTO outlet_lifecycle (outlet_id, status, start_date, end_date, precision_start, precision_end, comment) VALUES ({}, '{}', {}, {}, {}, {}, {});\n",
                            outlet.id.unwrap_or(0),
                            status.status.replace("'", "''"),
                            self.optional_date_to_sql(&status.start_date),
                            self.optional_date_to_sql(&status.end_date),
                            self.optional_string_to_sql(&status.precision_start),
                            self.optional_string_to_sql(&status.precision_end),
                            self.optional_string_to_sql(&status.comment)
                        ));
                    }
                }
                IROutletBlock::Characteristics(chars) => {
                    for char in chars {
                        sql.push_str(&format!(
                            "INSERT INTO outlet_characteristics (outlet_id, characteristic_name, characteristic_value) VALUES ({}, '{}', '{}');\n",
                            outlet.id.unwrap_or(0),
                            char.name.replace("'", "''"),
                            self.expression_to_sql_value(&char.value).replace("'", "''")
                        ));
                    }
                }
                IROutletBlock::Metadata(meta) => {
                    for m in meta {
                        sql.push_str(&format!(
                            "INSERT INTO outlet_metadata (outlet_id, metadata_name, metadata_value) VALUES ({}, '{}', '{}');\n",
                            outlet.id.unwrap_or(0),
                            m.name.replace("'", "''"),
                            self.expression_to_sql_value(&m.value).replace("'", "''")
                        ));
                    }
                }
            }
        }

        Ok(sql)
    }

    /// Generate relationship tables
    fn generate_relationship_tables(&self, ir: &IRProgram) -> Result<String> {
        let mut sql = String::new();

        sql.push_str("-- RELATIONSHIPS\n");

        for family in &ir.families {
            for relationship in &family.relationships {
                match relationship {
                    IRRelationship::Diachronic(diachronic) => {
                        sql.push_str(&format!(
                            "INSERT INTO relationships (relationship_name, relationship_type, family_id) VALUES ('{}', 'diachronic', (SELECT id FROM families WHERE name = '{}'));\n",
                            diachronic.name.replace("'", "''"),
                            family.name.replace("'", "''")
                        ));

                        sql.push_str(&format!(
                            "INSERT INTO diachronic_relationships (relationship_id, predecessor_id, successor_id, event_start_date, event_end_date, relationship_subtype, comment, maps_to) VALUES ((SELECT id FROM relationships WHERE relationship_name = '{}'), {}, {}, {}, {}, {}, {}, {});\n",
                            diachronic.name.replace("'", "''"),
                            diachronic.predecessor,
                            diachronic.successor,
                            self.optional_date_to_sql(&diachronic.event_start_date),
                            self.optional_date_to_sql(&diachronic.event_end_date),
                            self.optional_string_to_sql(&Some(diachronic.relationship_type.clone())),
                            self.optional_string_to_sql(&diachronic.comment),
                            self.optional_string_to_sql(&diachronic.maps_to)
                        ));
                    }
                    IRRelationship::Synchronous(sync) => {
                        sql.push_str(&format!(
                            "INSERT INTO relationships (relationship_name, relationship_type, family_id) VALUES ('{}', 'synchronous', (SELECT id FROM families WHERE name = '{}'));\n",
                            sync.name.replace("'", "''"),
                            family.name.replace("'", "''")
                        ));

                        sql.push_str(&format!(
                            "INSERT INTO synchronous_relationships (relationship_id, outlet_1_id, outlet_1_role, outlet_2_id, outlet_2_role, relationship_subtype, period_start, period_end, details, maps_to) VALUES ((SELECT id FROM relationships WHERE relationship_name = '{}'), {}, {}, {}, {}, {}, {}, {}, {}, {});\n",
                            sync.name.replace("'", "''"),
                            sync.outlet_1.id,
                            self.optional_string_to_sql(&Some(sync.outlet_1.role.clone())),
                            sync.outlet_2.id,
                            self.optional_string_to_sql(&Some(sync.outlet_2.role.clone())),
                            self.optional_string_to_sql(&Some(sync.relationship_type.clone())),
                            self.optional_date_to_sql(&sync.period_start),
                            self.optional_date_to_sql(&sync.period_end),
                            self.optional_string_to_sql(&sync.details),
                            self.optional_string_to_sql(&sync.maps_to)
                        ));
                    }
                }
            }
        }

        Ok(sql)
    }

    /// Generate data insertion statements
    fn generate_data_inserts(&self, ir: &IRProgram) -> Result<String> {
        let mut sql = String::new();

        sql.push_str("-- MARKET DATA\n");

        for family in &ir.families {
            for data_block in &family.data_blocks {
                // Insert aggregation settings
                for agg in &data_block.aggregation {
                    sql.push_str(&format!(
                        "INSERT INTO data_aggregation (outlet_id, aggregation_name, aggregation_value) VALUES ({}, '{}', '{}');\n",
                        data_block.outlet_id,
                        agg.name.replace("'", "''"),
                        agg.value.replace("'", "''")
                    ));
                }

                // Insert market data
                for year in &data_block.years {
                    for metric in &year.metrics {
                        sql.push_str(&format!(
                            "INSERT INTO market_data (outlet_id, data_year, metric_name, metric_value, metric_unit, data_source, comment, maps_to) VALUES ({}, {}, '{}', {}, {}, {}, {}, {});\n",
                            data_block.outlet_id,
                            year.year,
                            metric.name.replace("'", "''"),
                            metric.value,
                            self.optional_string_to_sql(&Some(metric.unit.clone())),
                            self.optional_string_to_sql(&Some(metric.source.clone())),
                            self.optional_string_to_sql(&metric.comment),
                            self.optional_string_to_sql(&data_block.maps_to)
                        ));
                    }
                }
            }
        }

        Ok(sql)
    }

    /// Helper function to convert expression to SQL comment
    fn expression_to_sql_comment(&self, expr: &IRExpression) -> String {
        match expr {
            IRExpression::String(s) => format!("\"{}\"", s),
            IRExpression::Number(n) => n.to_string(),
            IRExpression::Boolean(b) => b.to_string(),
            IRExpression::Variable(v) => format!("${}", v),
            IRExpression::Object(_) => "object".to_string(),
            IRExpression::Array(_) => "array".to_string(),
        }
    }

    /// Helper function to convert expression to SQL value
    fn expression_to_sql_value(&self, expr: &IRExpression) -> String {
        match expr {
            IRExpression::String(s) => s.clone(),
            IRExpression::Number(n) => n.to_string(),
            IRExpression::Boolean(b) => b.to_string(),
            IRExpression::Variable(v) => format!("${}", v),
            IRExpression::Object(_) => "{}".to_string(),
            IRExpression::Array(_) => "[]".to_string(),
        }
    }

    /// Helper function to convert optional date to SQL
    fn optional_date_to_sql(&self, date: &Option<String>) -> String {
        match date {
            Some(d) => format!("'{}'", d.replace("'", "''")),
            None => "NULL".to_string(),
        }
    }

    /// Helper function to convert optional string to SQL
    fn optional_string_to_sql(&self, s: &Option<String>) -> String {
        match s {
            Some(text) => format!("'{}'", text.replace("'", "''")),
            None => "NULL".to_string(),
        }
    }

    /// Generate event insertion statements
    fn generate_event_inserts(&self, ir: &IRProgram) -> Result<String> {
        let mut sql = String::new();

        if !ir.events.is_empty() {
            sql.push_str("-- EVENTS\n");

            for (event_index, event) in ir.events.iter().enumerate() {
                let event_id = event_index + 1;

                // Insert event
                sql.push_str(&format!(
                    "INSERT INTO events (id, name, event_type, event_date, status) VALUES ({}, '{}', '{}', {}, {});\n",
                    event_id,
                    event.name.replace("'", "''"),
                    event.event_type.replace("'", "''"),
                    self.optional_date_to_sql(&event.date),
                    self.optional_string_to_sql(&event.status)
                ));

                // Insert event entities
                for entity in &event.entities {
                    sql.push_str(&format!(
                        "INSERT INTO event_entities (event_id, entity_name, entity_id, entity_role, stake_before, stake_after) VALUES ({}, '{}', {}, {}, {}, {});\n",
                        event_id,
                        entity.name.replace("'", "''"),
                        entity.id,
                        self.optional_string_to_sql(&Some(entity.role.clone())),
                        entity.stake_before.map(|s| s.to_string()).unwrap_or("NULL".to_string()),
                        entity.stake_after.map(|s| s.to_string()).unwrap_or("NULL".to_string())
                    ));
                }

                // Insert event impact
                for impact in &event.impact {
                    sql.push_str(&format!(
                        "INSERT INTO event_impact (event_id, impact_name, impact_value) VALUES ({}, '{}', '{}');\n",
                        event_id,
                        impact.name.replace("'", "''"),
                        self.expression_to_sql_value(&impact.value).replace("'", "''")
                    ));
                }

                // Insert event metadata
                for metadata in &event.metadata {
                    sql.push_str(&format!(
                        "INSERT INTO event_metadata (event_id, metadata_name, metadata_value) VALUES ({}, '{}', '{}');\n",
                        event_id,
                        metadata.name.replace("'", "''"),
                        self.expression_to_sql_value(&metadata.value).replace("'", "''")
                    ));
                }
            }

            sql.push_str("\n");
        }

        Ok(sql)
    }
}
