//! ANMI-compatible SQL code generator

use crate::error::Result;
use crate::ir::nodes::*;
use std::collections::HashMap;

/// ANMI-compatible SQL code generator
pub struct AnmiSqlGenerator;

impl AnmiSqlGenerator {
    /// Create a new ANMI SQL generator
    pub fn new() -> Self {
        Self
    }

    /// Generate ANMI-compatible SQL code from IR
    pub fn generate(&self, ir: &IRProgram) -> Result<String> {
        let mut sql = String::new();

        // Add header comment
        sql.push_str("-- Generated ANMI-compatible SQL from MediaLanguage DSL\n");
        sql.push_str("-- This file recreates the original ANMI database schema\n");
        sql.push_str("-- Compatible with graphv3 schema structure\n\n");

        // Create schema if needed
        sql.push_str("-- Create schema if not exists\n");
        sql.push_str("CREATE SCHEMA IF NOT EXISTS graphv3;\n\n");

        // Generate ANMI core tables
        sql.push_str(&self.generate_anmi_schema()?);

        // Generate relationship tables (numbered tables)
        sql.push_str(&self.generate_relationship_tables()?);

        // Generate insert statements for outlets
        sql.push_str(&self.generate_outlet_inserts(ir)?);

        // Generate insert statements for market data
        sql.push_str(&self.generate_market_data_inserts(ir)?);

        // Generate insert statements for relationships
        sql.push_str(&self.generate_relationship_inserts(ir)?);

        Ok(sql)
    }

    /// Generate ANMI core schema tables
    fn generate_anmi_schema(&self) -> Result<String> {
        let mut sql = String::new();

        sql.push_str("-- ANMI Core Tables\n\n");

        // mo_constant table (media outlets)
        sql.push_str("CREATE TABLE IF NOT EXISTS graphv3.mo_constant (\n");
        sql.push_str("    id_mo INTEGER PRIMARY KEY,\n");
        sql.push_str("    mo_title VARCHAR(120),\n");
        sql.push_str("    id_sector INTEGER,\n");
        sql.push_str("    mandate INTEGER,\n");
        sql.push_str("    location VARCHAR(25),\n");
        sql.push_str("    primary_distr_area INTEGER,\n");
        sql.push_str("    local INTEGER,\n");
        sql.push_str("    language VARCHAR(5),\n");
        sql.push_str("    start_date DATE,\n");
        sql.push_str("    end_date DATE,\n");
        sql.push_str("    editorial_line_s TEXT,\n");
        sql.push_str("    comments TEXT\n");
        sql.push_str(");\n\n");

        // mo_year table (market data)
        sql.push_str("CREATE TABLE IF NOT EXISTS graphv3.mo_year (\n");
        sql.push_str("    id_mo INTEGER,\n");
        sql.push_str("    year INTEGER,\n");
        sql.push_str("    mo_year INTEGER,\n");
        sql.push_str("    calc INTEGER,\n");
        sql.push_str("    circulation INTEGER,\n");
        sql.push_str("    circulation_source INTEGER,\n");
        sql.push_str("    unique_users INTEGER,\n");
        sql.push_str("    unique_users_source INTEGER,\n");
        sql.push_str("    reach_nat DECIMAL(5,2),\n");
        sql.push_str("    reach_nat_source INTEGER,\n");
        sql.push_str("    reach_reg DECIMAL(5,2),\n");
        sql.push_str("    reach_reg_source INTEGER,\n");
        sql.push_str("    market_share DECIMAL(5,2),\n");
        sql.push_str("    market_share_source INTEGER,\n");
        sql.push_str("    comments TEXT,\n");
        sql.push_str("    PRIMARY KEY (id_mo, year, mo_year),\n");
        sql.push_str("    FOREIGN KEY (id_mo) REFERENCES graphv3.mo_constant(id_mo)\n");
        sql.push_str(");\n\n");

        // sources_names table
        sql.push_str("CREATE TABLE IF NOT EXISTS graphv3.sources_names (\n");
        sql.push_str("    id_source INTEGER PRIMARY KEY,\n");
        sql.push_str("    source_name VARCHAR(100)\n");
        sql.push_str(");\n\n");

        // sectors table (for vocabulary)
        sql.push_str("CREATE TABLE IF NOT EXISTS graphv3.sectors (\n");
        sql.push_str("    id_sector INTEGER PRIMARY KEY,\n");
        sql.push_str("    sector_name VARCHAR(100)\n");
        sql.push_str(");\n\n");

        // distribution_areas table (for vocabulary)
        sql.push_str("CREATE TABLE IF NOT EXISTS graphv3.distribution_areas (\n");
        sql.push_str("    id_area INTEGER PRIMARY KEY,\n");
        sql.push_str("    area_name VARCHAR(100)\n");
        sql.push_str(");\n\n");

        Ok(sql)
    }

    /// Generate relationship tables
    fn generate_relationship_tables(&self) -> Result<String> {
        let mut sql = String::new();

        sql.push_str("-- Relationship Tables\n\n");

        // Define relationship table mappings
        let relationship_tables = vec![
            ("11_succession", "Succession relationships"),
            ("12_amalgamation", "Amalgamation relationships"),
            ("13_new_distribution_area", "New distribution area relationships"),
            ("14_new_sector", "New sector relationships"),
            ("15_interruption", "Interruption relationships"),
            ("16_split_off", "Split-off relationships"),
            ("17_merger", "Merger relationships"),
            ("18_offshoot", "Offshoot relationships"),
            ("21_main_media_outlet", "Main media outlet relationships"),
            ("22_umbrella", "Umbrella relationships"),
            ("23_collaboration", "Collaboration relationships"),
        ];

        for (table_name, description) in relationship_tables {
            sql.push_str(&format!("-- {}\n", description));
            sql.push_str(&format!("CREATE TABLE IF NOT EXISTS graphv3.{} (\n", table_name));
            
            if table_name.starts_with("1") {
                // Diachronic relationships
                sql.push_str("    id_pred INTEGER,\n");
                sql.push_str("    id_succ INTEGER,\n");
                sql.push_str("    e_s DATE,\n");
                sql.push_str("    e_e DATE,\n");
                sql.push_str("    PRIMARY KEY (id_pred, id_succ),\n");
                sql.push_str("    FOREIGN KEY (id_pred) REFERENCES graphv3.mo_constant(id_mo),\n");
                sql.push_str("    FOREIGN KEY (id_succ) REFERENCES graphv3.mo_constant(id_mo)\n");
            } else {
                // Synchronous relationships
                sql.push_str("    id_mo_1 INTEGER,\n");
                sql.push_str("    id_mo_2 INTEGER,\n");
                sql.push_str("    p_s DATE,\n");
                sql.push_str("    p_e DATE,\n");
                sql.push_str("    PRIMARY KEY (id_mo_1, id_mo_2),\n");
                sql.push_str("    FOREIGN KEY (id_mo_1) REFERENCES graphv3.mo_constant(id_mo),\n");
                sql.push_str("    FOREIGN KEY (id_mo_2) REFERENCES graphv3.mo_constant(id_mo)\n");
            }
            sql.push_str(");\n\n");
        }

        Ok(sql)
    }

    /// Generate insert statements for outlets
    fn generate_outlet_inserts(&self, ir: &IRProgram) -> Result<String> {
        let mut sql = String::new();

        sql.push_str("-- Media Outlet Data\n\n");

        // First, populate vocabularies if they exist
        for vocab in &ir.vocabularies {
            match vocab.name.as_str() {
                "SECTOR" => {
                    sql.push_str("-- Populate sectors\n");
                    for entry in &vocab.entries {
                        if let IRVocabularyKey::Number(id) = &entry.key {
                            sql.push_str(&format!(
                                "INSERT INTO graphv3.sectors (id_sector, sector_name) VALUES ({}, '{}') ON CONFLICT DO NOTHING;\n",
                                id,
                                entry.value.replace("'", "''")
                            ));
                        }
                    }
                    sql.push_str("\n");
                }
                "DISTRIBUTION_AREA" => {
                    sql.push_str("-- Populate distribution areas\n");
                    for entry in &vocab.entries {
                        if let IRVocabularyKey::Number(id) = &entry.key {
                            sql.push_str(&format!(
                                "INSERT INTO graphv3.distribution_areas (id_area, area_name) VALUES ({}, '{}') ON CONFLICT DO NOTHING;\n",
                                id,
                                entry.value.replace("'", "''")
                            ));
                        }
                    }
                    sql.push_str("\n");
                }
                _ => {}
            }
        }

        // Collect all sources used in market data
        let mut sources: HashMap<i32, String> = HashMap::new();
        let mut source_id = 1;

        // Scan for sources in data blocks
        for family in &ir.families {
            for data_block in &family.data_blocks {
                for year in &data_block.years {
                    for metric in &year.metrics {
                        if !sources.values().any(|s| s == &metric.source) {
                            sources.insert(source_id, metric.source.clone());
                            source_id += 1;
                        }
                    }
                }
            }
        }

        // Insert sources
        if !sources.is_empty() {
            sql.push_str("-- Populate sources\n");
            for (id, name) in &sources {
                sql.push_str(&format!(
                    "INSERT INTO graphv3.sources_names (id_source, source_name) VALUES ({}, '{}') ON CONFLICT DO NOTHING;\n",
                    id,
                    name.replace("'", "''")
                ));
            }
            sql.push_str("\n");
        }

        // Insert media outlets
        sql.push_str("-- Insert media outlets\n");
        for family in &ir.families {
            for outlet in &family.outlets {
                let mut id_mo = outlet.id.unwrap_or(0) as i32;
                let mut mo_title = outlet.name.clone();
                let mut id_sector: Option<i32> = None;
                let mut mandate: Option<i32> = None;
                let mut location: Option<String> = None;
                let mut primary_distr_area: Option<i32> = None;
                let mut local: Option<i32> = None;
                let mut language: Option<String> = None;
                let mut start_date: Option<String> = None;
                let mut end_date: Option<String> = None;
                let mut editorial_line_s: Option<String> = None;
                let mut comments: Option<String> = None;

                // Extract fields from outlet blocks
                for block in &outlet.blocks {
                    match block {
                        IROutletBlock::Identity(fields) => {
                            for field in fields {
                                match field.name.as_str() {
                                    "id" => {
                                        if let IRExpression::Number(n) = &field.value {
                                            id_mo = *n as i32;
                                        }
                                    }
                                    "name" => {
                                        if let IRExpression::String(s) = &field.value {
                                            mo_title = s.clone();
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        IROutletBlock::Characteristics(chars) => {
                            for char in chars {
                                match char.name.as_str() {
                                    "sector" => {
                                        if let IRExpression::Number(n) = &char.value {
                                            id_sector = Some(*n as i32);
                                        }
                                    }
                                    "mandate" => {
                                        if let IRExpression::Number(n) = &char.value {
                                            mandate = Some(*n as i32);
                                        }
                                    }
                                    "location" => {
                                        if let IRExpression::String(s) = &char.value {
                                            location = Some(s.clone());
                                        }
                                    }
                                    "primary_distribution_area" => {
                                        if let IRExpression::Number(n) = &char.value {
                                            primary_distr_area = Some(*n as i32);
                                        }
                                    }
                                    "local" => {
                                        if let IRExpression::Number(n) = &char.value {
                                            local = Some(*n as i32);
                                        }
                                    }
                                    "language" => {
                                        if let IRExpression::String(s) = &char.value {
                                            language = Some(s.clone());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        IROutletBlock::Lifecycle(statuses) => {
                            // Get the first lifecycle entry for dates
                            if let Some(status) = statuses.first() {
                                start_date = status.start_date.clone();
                                end_date = status.end_date.clone();
                            }
                        }
                        IROutletBlock::Metadata(meta) => {
                            for m in meta {
                                match m.name.as_str() {
                                    "editorial_line" => {
                                        if let IRExpression::String(s) = &m.value {
                                            editorial_line_s = Some(s.clone());
                                        }
                                    }
                                    "comments" => {
                                        if let IRExpression::String(s) = &m.value {
                                            comments = Some(s.clone());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                // Generate INSERT statement
                sql.push_str(&format!(
                    "INSERT INTO graphv3.mo_constant (id_mo, mo_title, id_sector, mandate, location, primary_distr_area, local, language, start_date, end_date, editorial_line_s, comments) VALUES ({}, '{}', {}, {}, {}, {}, {}, {}, {}, {}, {}, {});\n",
                    id_mo,
                    mo_title.replace("'", "''"),
                    id_sector.map(|v| v.to_string()).unwrap_or("NULL".to_string()),
                    mandate.map(|v| v.to_string()).unwrap_or("NULL".to_string()),
                    location.map(|v| format!("'{}'", v.replace("'", "''"))).unwrap_or("NULL".to_string()),
                    primary_distr_area.map(|v| v.to_string()).unwrap_or("NULL".to_string()),
                    local.map(|v| v.to_string()).unwrap_or("NULL".to_string()),
                    language.map(|v| format!("'{}'", v.replace("'", "''"))).unwrap_or("NULL".to_string()),
                    start_date.map(|v| format!("'{}'", v)).unwrap_or("NULL".to_string()),
                    end_date.map(|v| format!("'{}'", v)).unwrap_or("NULL".to_string()),
                    editorial_line_s.map(|v| format!("'{}'", v.replace("'", "''"))).unwrap_or("NULL".to_string()),
                    comments.map(|v| format!("'{}'", v.replace("'", "''"))).unwrap_or("NULL".to_string())
                ));
            }
        }

        Ok(sql)
    }

    /// Generate insert statements for market data
    fn generate_market_data_inserts(&self, ir: &IRProgram) -> Result<String> {
        let mut sql = String::new();

        sql.push_str("\n-- Market Data\n\n");

        // Debug: Check if we have any data blocks
        let mut total_data_blocks = 0;
        for family in &ir.families {
            total_data_blocks += family.data_blocks.len();
        }
        sql.push_str(&format!("-- Debug: Found {} data blocks across {} families\n", total_data_blocks, ir.families.len()));

        // Build source lookup
        let mut source_lookup: HashMap<String, i32> = HashMap::new();
        let mut source_id = 1;

        for family in &ir.families {
            for data_block in &family.data_blocks {
                for year_data in &data_block.years {
                    for metric in &year_data.metrics {
                        // Map metric to ANMI columns
                        let metric_name = metric.name.as_str();
                        let year = year_data.year;
                        let outlet_id = data_block.outlet_id;

                        // Get or create source ID
                        let source_id_val = if !source_lookup.contains_key(&metric.source) {
                            source_lookup.insert(metric.source.clone(), source_id);
                            let id = source_id;
                            source_id += 1;
                            id
                        } else {
                            *source_lookup.get(&metric.source).unwrap()
                        };

                        // Generate appropriate INSERT based on metric type
                        match metric_name {
                            "circulation" => {
                                sql.push_str(&format!(
                                    "INSERT INTO graphv3.mo_year (id_mo, year, mo_year, calc, circulation, circulation_source) VALUES ({}, {}, {}, 0, {}, {}) ON CONFLICT (id_mo, year, mo_year) DO UPDATE SET circulation = {}, circulation_source = {};\n",
                                    outlet_id, year, outlet_id * 10000 + year, // mo_year calculation
                                    metric.value as i32, source_id_val,
                                    metric.value as i32, source_id_val
                                ));
                            }
                            "unique_users" => {
                                sql.push_str(&format!(
                                    "INSERT INTO graphv3.mo_year (id_mo, year, mo_year, calc, unique_users, unique_users_source) VALUES ({}, {}, {}, 0, {}, {}) ON CONFLICT (id_mo, year, mo_year) DO UPDATE SET unique_users = {}, unique_users_source = {};\n",
                                    outlet_id, year, outlet_id * 10000 + year,
                                    metric.value as i32, source_id_val,
                                    metric.value as i32, source_id_val
                                ));
                            }
                            "reach_national" => {
                                sql.push_str(&format!(
                                    "INSERT INTO graphv3.mo_year (id_mo, year, mo_year, calc, reach_nat, reach_nat_source) VALUES ({}, {}, {}, 0, {}, {}) ON CONFLICT (id_mo, year, mo_year) DO UPDATE SET reach_nat = {}, reach_nat_source = {};\n",
                                    outlet_id, year, outlet_id * 10000 + year,
                                    metric.value, source_id_val,
                                    metric.value, source_id_val
                                ));
                            }
                            "reach_regional" => {
                                sql.push_str(&format!(
                                    "INSERT INTO graphv3.mo_year (id_mo, year, mo_year, calc, reach_reg, reach_reg_source) VALUES ({}, {}, {}, 0, {}, {}) ON CONFLICT (id_mo, year, mo_year) DO UPDATE SET reach_reg = {}, reach_reg_source = {};\n",
                                    outlet_id, year, outlet_id * 10000 + year,
                                    metric.value, source_id_val,
                                    metric.value, source_id_val
                                ));
                            }
                            "market_share" => {
                                sql.push_str(&format!(
                                    "INSERT INTO graphv3.mo_year (id_mo, year, mo_year, calc, market_share, market_share_source) VALUES ({}, {}, {}, 0, {}, {}) ON CONFLICT (id_mo, year, mo_year) DO UPDATE SET market_share = {}, market_share_source = {};\n",
                                    outlet_id, year, outlet_id * 10000 + year,
                                    metric.value, source_id_val,
                                    metric.value, source_id_val
                                ));
                            }
                            _ => {
                                // Handle as comments for unknown metrics
                                sql.push_str(&format!(
                                    "-- Metric '{}' = {} {} (source: {})\n",
                                    metric_name, metric.value, metric.unit, metric.source
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(sql)
    }

    /// Generate insert statements for relationships
    fn generate_relationship_inserts(&self, ir: &IRProgram) -> Result<String> {
        let mut sql = String::new();

        sql.push_str("\n-- Relationships\n\n");

        for family in &ir.families {
            for relationship in &family.relationships {
                match relationship {
                    IRRelationship::Diachronic(diachronic) => {
                        let table_name = match diachronic.relationship_type.as_str() {
                            "succession" => "11_succession",
                            "amalgamation" => "12_amalgamation",
                            "new_distribution_area" => "13_new_distribution_area",
                            "new_sector" => "14_new_sector",
                            "interruption" => "15_interruption",
                            "split_off" => "16_split_off",
                            "merger" => "17_merger",
                            "offshoot" => "18_offshoot",
                            _ => continue,
                        };

                        sql.push_str(&format!(
                            "INSERT INTO graphv3.{} (id_pred, id_succ, e_s, e_e) VALUES ({}, {}, {}, {});\n",
                            table_name,
                            diachronic.predecessor,
                            diachronic.successor,
                            diachronic.event_start_date.as_ref().map(|d| format!("'{}'", d)).unwrap_or("NULL".to_string()),
                            diachronic.event_end_date.as_ref().map(|d| format!("'{}'", d)).unwrap_or("NULL".to_string())
                        ));
                    }
                    IRRelationship::Synchronous(sync) => {
                        let table_name = match sync.relationship_type.as_str() {
                            "main_media_outlet" => "21_main_media_outlet",
                            "umbrella" => "22_umbrella",
                            "collaboration" => "23_collaboration",
                            _ => continue,
                        };

                        sql.push_str(&format!(
                            "INSERT INTO graphv3.{} (id_mo_1, id_mo_2, p_s, p_e) VALUES ({}, {}, {}, {});\n",
                            table_name,
                            sync.outlet_1.id,
                            sync.outlet_2.id,
                            sync.period_start.as_ref().map(|d| format!("'{}'", d)).unwrap_or("NULL".to_string()),
                            sync.period_end.as_ref().map(|d| format!("'{}'", d)).unwrap_or("NULL".to_string())
                        ));
                    }
                }
            }
        }

        Ok(sql)
    }
}