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

        Ok(cypher)
    }

    /// Generate constraints and indexes
    fn generate_constraints(&self) -> Result<String> {
        let mut cypher = String::new();

        cypher.push_str("// CONSTRAINTS AND INDEXES\n");
        cypher.push_str("// Create constraints for unique identifiers\n\n");

        // Constraints
        cypher.push_str("CREATE CONSTRAINT outlet_id_unique IF NOT EXISTS FOR (o:Outlet) REQUIRE o.id IS UNIQUE;\n");
        cypher.push_str("CREATE CONSTRAINT family_name_unique IF NOT EXISTS FOR (f:Family) REQUIRE f.name IS UNIQUE;\n");
        cypher.push_str("CREATE CONSTRAINT template_name_unique IF NOT EXISTS FOR (t:Template) REQUIRE t.name IS UNIQUE;\n");
        cypher.push_str("CREATE CONSTRAINT vocab_name_unique IF NOT EXISTS FOR (v:Vocabulary) REQUIRE v.name IS UNIQUE;\n\n");

        // Indexes
        cypher
            .push_str("CREATE INDEX outlet_name_index IF NOT EXISTS FOR (o:Outlet) ON (o.name);\n");
        cypher
            .push_str("CREATE INDEX family_name_index IF NOT EXISTS FOR (f:Family) ON (f.name);\n");
        cypher.push_str(
            "CREATE INDEX data_year_index IF NOT EXISTS FOR (d:MarketData) ON (d.year);\n",
        );
        cypher.push_str(
            "CREATE INDEX metric_name_index IF NOT EXISTS FOR (m:Metric) ON (m.name);\n\n",
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
            "CREATE (f:Family {{name: '{}', comment: {}, created_at: datetime()}});\n",
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

        // Create outlet node
        cypher.push_str(&format!(
            "CREATE (o:Outlet {{id: {}, name: '{}', family_name: '{}', created_at: datetime()}});\n",
            outlet.id.unwrap_or(0),
            outlet.name.replace("'", "\\'"),
            family_name.replace("'", "\\'")
        ));

        // Connect to family
        cypher.push_str(&format!(
            "MATCH (f:Family {{name: '{}'}}), (o:Outlet {{id: {}}}) CREATE (f)-[:HAS_OUTLET]->(o);\n",
            family_name.replace("'", "\\'"),
            outlet.id.unwrap_or(0)
        ));

        // Create template relationship if exists
        if let Some(template_ref) = &outlet.template_ref {
            cypher.push_str(&format!(
                "MATCH (t:Template {{name: '{}'}}), (o:Outlet {{id: {}}}) CREATE (o)-[:EXTENDS_TEMPLATE]->(t);\n",
                template_ref.replace("'", "\\'"),
                outlet.id.unwrap_or(0)
            ));
        }

        // Create base relationship if exists
        if let Some(base_ref) = outlet.base_ref {
            cypher.push_str(&format!(
                "MATCH (base:Outlet {{id: {}}}), (o:Outlet {{id: {}}}) CREATE (o)-[:BASED_ON]->(base);\n",
                base_ref,
                outlet.id.unwrap_or(0)
            ));
        }

        // Process outlet blocks
        for block in &outlet.blocks {
            match block {
                IROutletBlock::Identity(fields) => {
                    for field in fields {
                        cypher.push_str(&format!(
                            "CREATE (i:Identity {{name: '{}', value: '{}', outlet_id: {}}});\n",
                            field.name.replace("'", "\\'"),
                            self.expression_to_cypher_value(&field.value)
                                .replace("'", "\\'"),
                            outlet.id.unwrap_or(0)
                        ));

                        cypher.push_str(&format!(
                            "MATCH (o:Outlet {{id: {}}}), (i:Identity {{name: '{}', outlet_id: {}}}) CREATE (o)-[:HAS_IDENTITY]->(i);\n",
                            outlet.id.unwrap_or(0),
                            field.name.replace("'", "\\'"),
                            outlet.id.unwrap_or(0)
                        ));
                    }
                }
                IROutletBlock::Lifecycle(statuses) => {
                    for status in statuses {
                        cypher.push_str(&format!(
                            "CREATE (l:Lifecycle {{status: '{}', start_date: {}, end_date: {}, precision_start: {}, precision_end: {}, comment: {}, outlet_id: {}}});\n",
                            status.status.replace("'", "\\'"),
                            self.optional_date_to_cypher(&status.start_date),
                            self.optional_date_to_cypher(&status.end_date),
                            self.optional_string_to_cypher(&status.precision_start),
                            self.optional_string_to_cypher(&status.precision_end),
                            self.optional_string_to_cypher(&status.comment),
                            outlet.id.unwrap_or(0)
                        ));

                        cypher.push_str(&format!(
                            "MATCH (o:Outlet {{id: {}}}), (l:Lifecycle {{status: '{}', outlet_id: {}}}) CREATE (o)-[:HAS_LIFECYCLE]->(l);\n",
                            outlet.id.unwrap_or(0),
                            status.status.replace("'", "\\'"),
                            outlet.id.unwrap_or(0)
                        ));
                    }
                }
                IROutletBlock::Characteristics(chars) => {
                    for char in chars {
                        cypher.push_str(&format!(
                            "CREATE (c:Characteristic {{name: '{}', value: '{}', outlet_id: {}}});\n",
                            char.name.replace("'", "\\'"),
                            self.expression_to_cypher_value(&char.value).replace("'", "\\'"),
                            outlet.id.unwrap_or(0)
                        ));

                        cypher.push_str(&format!(
                            "MATCH (o:Outlet {{id: {}}}), (c:Characteristic {{name: '{}', outlet_id: {}}}) CREATE (o)-[:HAS_CHARACTERISTIC]->(c);\n",
                            outlet.id.unwrap_or(0),
                            char.name.replace("'", "\\'"),
                            outlet.id.unwrap_or(0)
                        ));
                    }
                }
                IROutletBlock::Metadata(meta) => {
                    for m in meta {
                        cypher.push_str(&format!(
                            "CREATE (m:Metadata {{name: '{}', value: '{}', outlet_id: {}}});\n",
                            m.name.replace("'", "\\'"),
                            self.expression_to_cypher_value(&m.value)
                                .replace("'", "\\'"),
                            outlet.id.unwrap_or(0)
                        ));

                        cypher.push_str(&format!(
                            "MATCH (o:Outlet {{id: {}}}), (m:Metadata {{name: '{}', outlet_id: {}}}) CREATE (o)-[:HAS_METADATA]->(m);\n",
                            outlet.id.unwrap_or(0),
                            m.name.replace("'", "\\'"),
                            outlet.id.unwrap_or(0)
                        ));
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
                        cypher.push_str(&format!(
                            "MATCH (pred:Outlet {{id: {}}}), (succ:Outlet {{id: {}}}) CREATE (pred)-[:DIACHRONIC_LINK {{name: '{}', relationship_type: '{}', event_start_date: {}, event_end_date: {}, comment: {}, maps_to: {}}}]->(succ);\n",
                            diachronic.predecessor,
                            diachronic.successor,
                            diachronic.name.replace("'", "\\'"),
                            diachronic.relationship_type.replace("'", "\\'"),
                            self.optional_date_to_cypher(&diachronic.event_start_date),
                            self.optional_date_to_cypher(&diachronic.event_end_date),
                            self.optional_string_to_cypher(&diachronic.comment),
                            self.optional_string_to_cypher(&diachronic.maps_to)
                        ));
                    }
                    IRRelationship::Synchronous(sync) => {
                        cypher.push_str(&format!("// Synchronous relationship: {}\n", sync.name));
                        cypher.push_str(&format!(
                            "MATCH (o1:Outlet {{id: {}}}), (o2:Outlet {{id: {}}}) CREATE (o1)-[:SYNCHRONOUS_LINK {{name: '{}', relationship_type: '{}', outlet_1_role: '{}', outlet_2_role: '{}', period_start: {}, period_end: {}, details: {}, maps_to: {}}}]->(o2);\n",
                            sync.outlet_1.id,
                            sync.outlet_2.id,
                            sync.name.replace("'", "\\'"),
                            sync.relationship_type.replace("'", "\\'"),
                            sync.outlet_1.role.replace("'", "\\'"),
                            sync.outlet_2.role.replace("'", "\\'"),
                            self.optional_date_to_cypher(&sync.period_start),
                            self.optional_date_to_cypher(&sync.period_end),
                            self.optional_string_to_cypher(&sync.details),
                            self.optional_string_to_cypher(&sync.maps_to)
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

    /// Helper function to convert optional date to Cypher
    fn optional_date_to_cypher(&self, date: &Option<String>) -> String {
        match date {
            Some(d) => format!("date('{}')", d.replace("'", "\\'")),
            None => "null".to_string(),
        }
    }

    /// Helper function to convert optional string to Cypher
    fn optional_string_to_cypher(&self, s: &Option<String>) -> String {
        match s {
            Some(text) => format!("'{}'", text.replace("'", "\\'")),
            None => "null".to_string(),
        }
    }
}
