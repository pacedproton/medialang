//! Cypher code generator for Neo4j graph database

use crate::error::Result;
use crate::ir::nodes::*;

/// Cypher code generator
pub struct CypherGenerator;

impl CypherGenerator {
    /// Create a new Cypher generator
    pub fn new() -> Self {
        Self
    }

    /// Generate Cypher code from IR
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

        // Constraints - updated for mdsl_media_outlet schema
        cypher.push_str("CREATE CONSTRAINT mdsl_media_outlet_id_unique IF NOT EXISTS FOR (o:mdsl_media_outlet) REQUIRE o.id_mo IS UNIQUE;\n");
        cypher.push_str("CREATE CONSTRAINT mdsl_family_name_unique IF NOT EXISTS FOR (f:mdsl_Family) REQUIRE f.name IS UNIQUE;\n");
        cypher.push_str("CREATE CONSTRAINT mdsl_template_name_unique IF NOT EXISTS FOR (t:mdsl_Template) REQUIRE t.name IS UNIQUE;\n");
        cypher.push_str("CREATE CONSTRAINT mdsl_vocab_name_unique IF NOT EXISTS FOR (v:mdsl_Vocabulary) REQUIRE v.name IS UNIQUE;\n\n");

        // Indexes - updated for mdsl_media_outlet schema
        cypher.push_str("CREATE INDEX mdsl_media_outlet_title_index IF NOT EXISTS FOR (o:mdsl_media_outlet) ON (o.mo_title);\n");
        cypher
            .push_str("CREATE INDEX mdsl_family_name_index IF NOT EXISTS FOR (f:mdsl_Family) ON (f.name);\n");
        cypher.push_str(
            "CREATE INDEX mdsl_data_year_index IF NOT EXISTS FOR (d:mdsl_MarketData) ON (d.year);\n",
        );
        cypher.push_str(
            "CREATE INDEX mdsl_metric_name_index IF NOT EXISTS FOR (m:mdsl_Metric) ON (m.name);\n\n",
        );

        Ok(cypher)
    }

    /// Generate vocabulary nodes
    fn generate_vocabulary_nodes(&self, vocab: &IRVocabulary) -> Result<String> {
        let mut cypher = String::new();

        cypher.push_str(&format!("// Vocabulary: {}\n", vocab.name));

        // Create vocabulary node
        cypher.push_str(&format!(
            "CREATE (v:Vocabulary {{name: '{}', body_name: '{}', created_at: datetime()}});\n",
            vocab.name.replace("'", "\\'"),
            vocab.body_name.replace("'", "\\'")
        ));

        // Create vocabulary entry nodes and relationships
        for entry in &vocab.entries {
            let key_str = match &entry.key {
                IRVocabularyKey::Number(n) => n.to_string(),
                IRVocabularyKey::String(s) => s.clone(),
            };

            cypher.push_str(&format!(
                "CREATE (e:VocabularyEntry {{key: '{}', value: '{}', vocab_name: '{}'}});\n",
                key_str.replace("'", "\\'"),
                entry.value.replace("'", "\\'"),
                vocab.name.replace("'", "\\'")
            ));

            cypher.push_str(&format!(
                "MATCH (v:Vocabulary {{name: '{}'}}), (e:VocabularyEntry {{key: '{}', vocab_name: '{}'}}) CREATE (v)-[:HAS_ENTRY]->(e);\n",
                vocab.name.replace("'", "\\'"),
                key_str.replace("'", "\\'"),
                vocab.name.replace("'", "\\'")
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
            "CREATE (t:Template {{name: '{}', type: '{}', created_at: datetime()}});\n",
            template.name.replace("'", "\\'"),
            template.template_type.replace("'", "\\'")
        ));

        // Create characteristic and metadata nodes
        for block in &template.blocks {
            match block {
                IRTemplateBlock::Characteristics(chars) => {
                    for char in chars {
                        cypher.push_str(&format!(
                            "CREATE (c:Characteristic {{name: '{}', value: '{}', template_name: '{}'}});\n",
                            char.name.replace("'", "\\'"),
                            self.expression_to_cypher_value(&char.value).replace("'", "\\'"),
                            template.name.replace("'", "\\'")
                        ));

                        cypher.push_str(&format!(
                            "MATCH (t:Template {{name: '{}'}}), (c:Characteristic {{name: '{}', template_name: '{}'}}) CREATE (t)-[:HAS_CHARACTERISTIC]->(c);\n",
                            template.name.replace("'", "\\'"),
                            char.name.replace("'", "\\'"),
                            template.name.replace("'", "\\'")
                        ));
                    }
                }
                IRTemplateBlock::Metadata(meta) => {
                    for m in meta {
                        cypher.push_str(&format!(
                            "CREATE (m:Metadata {{name: '{}', value: '{}', template_name: '{}'}});\n",
                            m.name.replace("'", "\\'"),
                            self.expression_to_cypher_value(&m.value).replace("'", "\\'"),
                            template.name.replace("'", "\\'")
                        ));

                        cypher.push_str(&format!(
                            "MATCH (t:Template {{name: '{}'}}), (m:Metadata {{name: '{}', template_name: '{}'}}) CREATE (t)-[:HAS_METADATA]->(m);\n",
                            template.name.replace("'", "\\'"),
                            m.name.replace("'", "\\'"),
                            template.name.replace("'", "\\'")
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

        // Create family node
        cypher.push_str(&format!(
            "CREATE (f:mdsl_Family {{name: '{}', comment: {}, created_at: datetime()}});\n",
            family.name.replace("'", "\\'"),
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

        // Create mdsl_media_outlet node with proper schema
        let outlet_id = outlet.id.unwrap_or(0);
        cypher.push_str(&format!(
            "CREATE (o:mdsl_media_outlet {{id_mo: {}, mo_title: '{}', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'}});\n",
            outlet_id,
            outlet.name.replace("'", "\\'")
        ));

        // Connect to family (keeping family structure for organization)
        cypher.push_str(&format!(
            "MATCH (f:mdsl_Family {{name: '{}'}}), (o:mdsl_media_outlet {{id_mo: {}}}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);\n",
            family_name.replace("'", "\\'"),
            outlet.id.unwrap_or(0)
        ));

        // Create template relationship if exists
        if let Some(template_ref) = &outlet.template_ref {
            cypher.push_str(&format!(
                "MATCH (t:Template {{name: '{}'}}), (o:mdsl_media_outlet {{id_mo: {}}}) CREATE (o)-[:EXTENDS_TEMPLATE]->(t);\n",
                template_ref.replace("'", "\\'"),
                outlet.id.unwrap_or(0)
            ));
        }

        // Create base relationship if exists
        if let Some(base_ref) = outlet.base_ref {
            cypher.push_str(&format!(
                "MATCH (base:mdsl_media_outlet {{id_mo: {}}}), (o:mdsl_media_outlet {{id_mo: {}}}) CREATE (o)-[:BASED_ON]->(base);\n",
                base_ref,
                outlet.id.unwrap_or(0)
            ));
        }

        // Process outlet blocks - simplified for mdsl_media_outlet compatibility
        for block in &outlet.blocks {
            match block {
                IROutletBlock::Identity(fields) => {
                    // Add identity fields as properties to the mdsl_media_outlet node
                    for field in fields {
                        match field.name.as_str() {
                            "title" => {
                                // Update mo_title if specified
                                cypher.push_str(&format!(
                                    "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.mo_title = '{}';\n",
                                    outlet.id.unwrap_or(0),
                                    self.expression_to_cypher_value(&field.value)
                                        .replace("'", "\\'")
                                ));
                            }
                            "url" => {
                                // Add as comment if it's a URL
                                cypher.push_str(&format!(
                                    "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.comments = COALESCE(o.comments, '') + ' URL: {}';\n",
                                    outlet.id.unwrap_or(0),
                                    self.expression_to_cypher_value(&field.value).replace("'", "\\'")
                                ));
                            }
                            _ => {
                                // Add other identity fields as comments
                                cypher.push_str(&format!(
                                    "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.comments = COALESCE(o.comments, '') + ' {}: {}';\n",
                                    outlet.id.unwrap_or(0),
                                    field.name.replace("'", "\\'"),
                                    self.expression_to_cypher_value(&field.value).replace("'", "\\'")
                                ));
                            }
                        }
                    }
                }
                IROutletBlock::Characteristics(fields) => {
                    // Map characteristics to mdsl_media_outlet schema
                    for field in fields {
                        match field.name.as_str() {
                            "sector" => {
                                if let Some(sector_val) =
                                    self.extract_number_from_expression(&field.value)
                                {
                                    cypher.push_str(&format!(
                                        "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.id_sector = {};\n",
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
                                        "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.mandate = {};\n",
                                        outlet.id.unwrap_or(0),
                                        mandate_val
                                    ));
                                }
                            }
                            "language" => {
                                cypher.push_str(&format!(
                                    "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.language = '{}';\n",
                                    outlet.id.unwrap_or(0),
                                    self.expression_to_cypher_value(&field.value)
                                        .replace("'", "\\'")
                                ));
                            }
                            _ => {
                                // Add other characteristics as comments
                                cypher.push_str(&format!(
                                    "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.comments = COALESCE(o.comments, '') + ' {}: {}';\n",
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
                                "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.start_date = datetime('{}');\n",
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
                                "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.end_date = datetime('{}');\n",
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
                                "MATCH (o:mdsl_media_outlet {{id_mo: {}}}) SET o.comments = '{}';\n",
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
                            "MATCH (pred:mdsl_media_outlet {{id_mo: {}}}), (succ:mdsl_media_outlet {{id_mo: {}}}) MERGE (pred)-[r:mdsl_{}]->(succ) SET r.event_rel = datetime('{}');\n",
                            diachronic.predecessor,
                            diachronic.successor,
                            rel_type,
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
                            "MATCH (o1:mdsl_media_outlet {{id_mo: {}}}), (o2:mdsl_media_outlet {{id_mo: {}}}) MERGE (o1)-[r:mdsl_{}]->(o2) SET r.start_rel = datetime('{}'), r.end_rel = datetime('{}');\n",
                            sync.outlet_1.id,
                            sync.outlet_2.id,
                            rel_type,
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
                        "CREATE (a:DataAggregation {{name: '{}', value: '{}', outlet_id: {}}});\n",
                        agg.name.replace("'", "\\'"),
                        agg.value.replace("'", "\\'"),
                        data_block.outlet_id
                    ));

                    cypher.push_str(&format!(
                        "MATCH (o:Outlet {{id: {}}}), (a:DataAggregation {{name: '{}', outlet_id: {}}}) CREATE (o)-[:HAS_AGGREGATION]->(a);\n",
                        data_block.outlet_id,
                        agg.name.replace("'", "\\'"),
                        data_block.outlet_id
                    ));
                }

                // Create market data nodes
                for year in &data_block.years {
                    cypher.push_str(&format!(
                        "CREATE (d:MarketData {{year: {}, outlet_id: {}, comment: {}, maps_to: {}}});\n",
                        year.year,
                        data_block.outlet_id,
                        self.optional_string_to_cypher(&year.comment),
                        self.optional_string_to_cypher(&data_block.maps_to)
                    ));

                    cypher.push_str(&format!(
                        "MATCH (o:Outlet {{id: {}}}), (d:MarketData {{year: {}, outlet_id: {}}}) CREATE (o)-[:HAS_DATA]->(d);\n",
                        data_block.outlet_id,
                        year.year,
                        data_block.outlet_id
                    ));

                    // Create metric nodes
                    for metric in &year.metrics {
                        cypher.push_str(&format!(
                            "CREATE (m:Metric {{name: '{}', value: {}, unit: '{}', source: '{}', comment: {}, year: {}, outlet_id: {}}});\n",
                            metric.name.replace("'", "\\'"),
                            metric.value,
                            metric.unit.replace("'", "\\'"),
                            metric.source.replace("'", "\\'"),
                            self.optional_string_to_cypher(&metric.comment),
                            year.year,
                            data_block.outlet_id
                        ));

                        cypher.push_str(&format!(
                            "MATCH (d:MarketData {{year: {}, outlet_id: {}}}), (m:Metric {{name: '{}', year: {}, outlet_id: {}}}) CREATE (d)-[:HAS_METRIC]->(m);\n",
                            year.year,
                            data_block.outlet_id,
                            metric.name.replace("'", "\\'"),
                            year.year,
                            data_block.outlet_id
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
                    "CREATE (e:Event {{name: '{}', type: '{}', date: {}, status: {}, created_at: datetime()}});\n",
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
                        "CREATE (ee:EventEntity {{name: '{}', entity_id: {}, role: '{}', stake_before: {}, stake_after: {}, event_name: '{}'}});\n",
                        entity.name.replace("'", "\\'"),
                        entity.id,
                        entity.role.replace("'", "\\'"),
                        entity.stake_before.map(|s| s.to_string()).unwrap_or("null".to_string()),
                        entity.stake_after.map(|s| s.to_string()).unwrap_or("null".to_string()),
                        event.name.replace("'", "\\'")
                    ));

                    // Connect event entity to event
                    cypher.push_str(&format!(
                        "MATCH (e:Event {{name: '{}'}}), (ee:EventEntity {{name: '{}', event_name: '{}'}}) CREATE (e)-[:HAS_ENTITY]->(ee);\n",
                        event.name.replace("'", "\\'"),
                        entity.name.replace("'", "\\'"),
                        event.name.replace("'", "\\'")
                    ));

                    // Connect event entity to media outlet if possible
                    cypher.push_str(&format!(
                        "MATCH (mo:mdsl_media_outlet {{id_mo: {}}}), (ee:EventEntity {{entity_id: {}, event_name: '{}'}}) CREATE (ee)-[:INVOLVES]->(mo);\n",
                        entity.id,
                        entity.id,
                        event.name.replace("'", "\\'")
                    ));
                }

                // Create event impact nodes
                for impact in &event.impact {
                    cypher.push_str(&format!(
                        "CREATE (ei:EventImpact {{name: '{}', value: '{}', event_name: '{}'}});\n",
                        impact.name.replace("'", "\\'"),
                        self.expression_to_cypher_value(&impact.value).replace("'", "\\'"),
                        event.name.replace("'", "\\'")
                    ));

                    cypher.push_str(&format!(
                        "MATCH (e:Event {{name: '{}'}}), (ei:EventImpact {{name: '{}', event_name: '{}'}}) CREATE (e)-[:HAS_IMPACT]->(ei);\n",
                        event.name.replace("'", "\\'"),
                        impact.name.replace("'", "\\'"),
                        event.name.replace("'", "\\'")
                    ));
                }

                // Create event metadata nodes
                for metadata in &event.metadata {
                    cypher.push_str(&format!(
                        "CREATE (em:EventMetadata {{name: '{}', value: '{}', event_name: '{}'}});\n",
                        metadata.name.replace("'", "\\'"),
                        self.expression_to_cypher_value(&metadata.value).replace("'", "\\'"),
                        event.name.replace("'", "\\'")
                    ));

                    cypher.push_str(&format!(
                        "MATCH (e:Event {{name: '{}'}}), (em:EventMetadata {{name: '{}', event_name: '{}'}}) CREATE (e)-[:HAS_METADATA]->(em);\n",
                        event.name.replace("'", "\\'"),
                        metadata.name.replace("'", "\\'"),
                        event.name.replace("'", "\\'")
                    ));
                }
            }

            cypher.push_str("\n");
        }

        Ok(cypher)
    }
}
