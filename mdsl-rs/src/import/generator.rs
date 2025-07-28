//! MDSL code generator for imported data

use crate::error::Result;
use crate::import::mapper::{
    MdslDataBlock, MdslDiachronicLink, MdslEntity, MdslFamily, MdslOutlet, MdslSynchronousLink,
    MdslUnit, MdslVocabulary,
};
use crate::import::MdslFieldType;
use std::fmt::Write;

/// MDSL code generator for imported data
#[derive(Debug, Clone)]
pub struct MdslGenerator;

impl MdslGenerator {
    /// Create a new MDSL generator
    pub fn new() -> Self {
        Self
    }

    /// Generate MDSL code from mapped entities
    pub fn generate(&self, entities: &[MdslEntity]) -> Result<String> {
        let mut output = String::new();

        writeln!(output, "// Generated MDSL from SQL database import")?;
        writeln!(output)?;

        // Generate header comments
        writeln!(
            output,
            "// This file was automatically generated from SQL database content"
        )?;
        writeln!(output)?;

        // Generate sections
        let vocabularies: Vec<_> = entities
            .iter()
            .filter_map(|e| match e {
                MdslEntity::Vocabulary(v) => Some(v),
                _ => None,
            })
            .collect();

        if !vocabularies.is_empty() {
            writeln!(output, "// VOCABULARIES")?;
            for vocab in vocabularies {
                self.generate_vocabulary(&mut output, vocab)?;
                writeln!(output)?;
            }
        }

        let units: Vec<_> = entities
            .iter()
            .filter_map(|e| match e {
                MdslEntity::Unit(u) => Some(u),
                _ => None,
            })
            .collect();

        if !units.is_empty() {
            writeln!(output, "// UNITS (Schema Definitions)")?;
            for unit in units {
                self.generate_unit(&mut output, unit)?;
                writeln!(output)?;
            }
        }

        let families: Vec<_> = entities
            .iter()
            .filter_map(|e| match e {
                MdslEntity::Family(f) => Some(f),
                _ => None,
            })
            .collect();

        if !families.is_empty() {
            writeln!(output, "// FAMILIES AND OUTLETS")?;
            for family in families {
                self.generate_family(&mut output, family)?;
                writeln!(output)?;
            }
        }

        let outlets: Vec<_> = entities
            .iter()
            .filter_map(|e| match e {
                MdslEntity::Outlet(o) => Some(o),
                _ => None,
            })
            .collect();

        if !outlets.is_empty() {
            writeln!(output, "// STANDALONE OUTLETS")?;
            for outlet in outlets {
                self.generate_standalone_outlet(&mut output, outlet)?;
                writeln!(output)?;
            }
        }

        Ok(output)
    }

    fn generate_vocabulary(&self, output: &mut String, vocab: &MdslVocabulary) -> Result<()> {
        writeln!(output, "VOCABULARY {} {{", vocab.name)?;
        writeln!(output, "    TYPES {{")?;

        for (index, entry) in vocab.entries.iter().enumerate() {
            let comma = if index < vocab.entries.len() - 1 {
                ","
            } else {
                ""
            };
            writeln!(
                output,
                "        {} = \"{}\"{}",
                self.sanitize_identifier(&entry.key),
                self.escape_string(&entry.value),
                comma
            )?;
        }

        writeln!(output, "    }}")?;
        writeln!(output, "}}")?;
        Ok(())
    }

    fn generate_unit(&self, output: &mut String, unit: &MdslUnit) -> Result<()> {
        writeln!(output, "UNIT {} {{", unit.name)?;

        for field in &unit.fields {
            let primary_key = if field.is_primary_key {
                " PRIMARY KEY"
            } else {
                ""
            };
            let type_str = self.format_field_type(&field.field_type);
            let comment = if let Some(comment) = &field.comment {
                format!(" // {}", comment)
            } else {
                String::new()
            };

            writeln!(
                output,
                "    {}: {}{}{}",
                field.name, type_str, primary_key, comment
            )?;
        }

        writeln!(output, "}}")?;
        Ok(())
    }

    fn generate_family(&self, output: &mut String, family: &MdslFamily) -> Result<()> {
        writeln!(output, "FAMILY \"{}\" {{", family.name)?;

        if let Some(comment) = &family.comment {
            writeln!(output, "    @comment \"{}\"", self.escape_string(comment))?;
            writeln!(output)?;
        }

        // Generate outlets within the family
        for outlet in &family.outlets {
            self.generate_outlet_content(output, outlet, "    ")?;
            writeln!(output)?;
        }

        // Generate diachronic links
        for link in &family.diachronic_links {
            self.generate_diachronic_link(output, link, "    ")?;
            writeln!(output)?;
        }

        // Generate synchronous links
        for link in &family.synchronous_links {
            self.generate_synchronous_link(output, link, "    ")?;
            writeln!(output)?;
        }

        // Generate data blocks
        for data_block in &family.data_blocks {
            self.generate_data_block(output, data_block, "    ")?;
            writeln!(output)?;
        }

        writeln!(output, "}}")?;
        Ok(())
    }

    fn generate_outlet_content(
        &self,
        output: &mut String,
        outlet: &MdslOutlet,
        indent: &str,
    ) -> Result<()> {
        let mut header = format!("OUTLET \"{}\"", outlet.name);

        if let Some(template) = &outlet.extends_template {
            header.push_str(&format!(" EXTENDS TEMPLATE \"{}\"", template));
        } else if let Some(base_id) = outlet.based_on {
            header.push_str(&format!(" BASED_ON {}", base_id));
        }

        writeln!(output, "{}{} {{", indent, header)?;

        // Generate id
        if let Some(id) = outlet.id {
            writeln!(output, "{}    id = {};", indent, id)?;
        }

        // Generate identity block
        if !outlet.identity.is_empty() {
            writeln!(output, "{}    identity {{", indent)?;
            for (key, value) in &outlet.identity {
                writeln!(
                    output,
                    "{}        {} = \"{}\";",
                    indent,
                    key,
                    self.escape_string(value)
                )?;
            }
            writeln!(output, "{}    }};", indent)?;
        }

        // Generate lifecycle block
        if !outlet.lifecycle.is_empty() {
            writeln!(output, "{}    lifecycle {{", indent)?;
            for status in &outlet.lifecycle {
                self.generate_lifecycle_status(output, status, indent)?;
            }
            writeln!(output, "{}    }};", indent)?;
        }

        // Generate characteristics block
        if !outlet.characteristics.is_empty() {
            writeln!(output, "{}    characteristics {{", indent)?;
            for (key, value) in &outlet.characteristics {
                if self.is_numeric_value(value) {
                    writeln!(output, "{}        {} = {};", indent, key, value)?;
                } else {
                    writeln!(
                        output,
                        "{}        {} = \"{}\";",
                        indent,
                        key,
                        self.escape_string(value)
                    )?;
                }
            }
            writeln!(output, "{}    }};", indent)?;
        }

        // Generate metadata block
        if !outlet.metadata.is_empty() {
            writeln!(output, "{}    metadata {{", indent)?;
            for (key, value) in &outlet.metadata {
                writeln!(
                    output,
                    "{}        {} = \"{}\";",
                    indent,
                    key,
                    self.escape_string(value)
                )?;
            }
            writeln!(output, "{}    }};", indent)?;
        }

        writeln!(output, "{}}};", indent)?;
        Ok(())
    }

    fn generate_lifecycle_status(
        &self,
        output: &mut String,
        status: &crate::import::mapper::MdslLifecycleStatus,
        indent: &str,
    ) -> Result<()> {
        let start_date = status.start_date.as_deref().unwrap_or("unknown");
        let end_date = status.end_date.as_deref().unwrap_or("CURRENT");

        writeln!(
            output,
            "{}        status \"{}\" FROM \"{}\" TO {} {{",
            indent, status.status, start_date, end_date
        )?;

        if let Some(precision_start) = &status.precision_start {
            writeln!(
                output,
                "{}            precision_start = \"{}\";",
                indent, precision_start
            )?;
        }

        if let Some(precision_end) = &status.precision_end {
            writeln!(
                output,
                "{}            precision_end = \"{}\";",
                indent, precision_end
            )?;
        }

        if let Some(comment) = &status.comment {
            writeln!(
                output,
                "{}            comment = \"{}\";",
                indent,
                self.escape_string(comment)
            )?;
        }

        writeln!(output, "{}        }};", indent)?;
        Ok(())
    }

    fn generate_standalone_outlet(&self, output: &mut String, outlet: &MdslOutlet) -> Result<()> {
        self.generate_outlet_content(output, outlet, "")?;
        Ok(())
    }

    fn generate_diachronic_link(
        &self,
        output: &mut String,
        link: &MdslDiachronicLink,
        indent: &str,
    ) -> Result<()> {
        writeln!(output, "{}DIACHRONIC_LINK {} {{", indent, link.name)?;
        writeln!(output, "{}    predecessor = {};", indent, link.predecessor)?;
        writeln!(output, "{}    successor = {};", indent, link.successor)?;

        if let Some(event_date) = &link.event_date {
            writeln!(output, "{}    event_date = \"{}\";", indent, event_date)?;
        }

        writeln!(
            output,
            "{}    relationship_type = \"{}\";",
            indent,
            self.escape_string(&link.relationship_type)
        )?;

        if let Some(comment) = &link.comment {
            writeln!(
                output,
                "{}    @comment \"{}\";",
                indent,
                self.escape_string(comment)
            )?;
        }

        if let Some(maps_to) = &link.maps_to {
            writeln!(
                output,
                "{}    @maps_to \"{}\";",
                indent,
                self.escape_string(maps_to)
            )?;
        }

        writeln!(output, "{}}};", indent)?;
        Ok(())
    }

    fn generate_synchronous_link(
        &self,
        output: &mut String,
        link: &MdslSynchronousLink,
        indent: &str,
    ) -> Result<()> {
        writeln!(output, "{}SYNCHRONOUS_LINK {} {{", indent, link.name)?;

        writeln!(output, "{}    outlet_1 = {{", indent)?;
        writeln!(output, "{}        id = {};", indent, link.outlet_1.id)?;
        if let Some(role) = &link.outlet_1.role {
            writeln!(
                output,
                "{}        role = \"{}\";",
                indent,
                self.escape_string(role)
            )?;
        }
        writeln!(output, "{}    }};", indent)?;

        writeln!(output, "{}    outlet_2 = {{", indent)?;
        writeln!(output, "{}        id = {};", indent, link.outlet_2.id)?;
        if let Some(role) = &link.outlet_2.role {
            writeln!(
                output,
                "{}        role = \"{}\";",
                indent,
                self.escape_string(role)
            )?;
        }
        writeln!(output, "{}    }};", indent)?;

        writeln!(
            output,
            "{}    relationship_type = \"{}\";",
            indent,
            self.escape_string(&link.relationship_type)
        )?;

        if let Some(period_start) = &link.period_start {
            let period_end = link.period_end.as_deref().unwrap_or("CURRENT");
            writeln!(
                output,
                "{}    period = \"{}\" TO \"{}\";",
                indent, period_start, period_end
            )?;
        }

        if let Some(details) = &link.details {
            writeln!(
                output,
                "{}    details = \"{}\";",
                indent,
                self.escape_string(details)
            )?;
        }

        if let Some(maps_to) = &link.maps_to {
            writeln!(
                output,
                "{}    @maps_to \"{}\";",
                indent,
                self.escape_string(maps_to)
            )?;
        }

        writeln!(output, "{}}};", indent)?;
        Ok(())
    }

    fn generate_data_block(
        &self,
        output: &mut String,
        data_block: &MdslDataBlock,
        indent: &str,
    ) -> Result<()> {
        writeln!(output, "{}DATA FOR {} {{", indent, data_block.outlet_id)?;

        // Generate aggregation settings
        if !data_block.aggregation.is_empty() {
            writeln!(output, "{}    aggregation = {{", indent)?;
            for (key, value) in &data_block.aggregation {
                writeln!(
                    output,
                    "{}        {} = \"{}\";",
                    indent,
                    key,
                    self.escape_string(value)
                )?;
            }
            writeln!(output, "{}    }};", indent)?;
            writeln!(output)?;
        }

        // Generate year data
        for year_data in &data_block.years {
            writeln!(output, "{}    YEAR {} {{", indent, year_data.year)?;

            if !year_data.metrics.is_empty() {
                writeln!(output, "{}        metrics {{", indent)?;
                for metric in &year_data.metrics {
                    writeln!(output, "{}            {} = {{", indent, metric.name)?;
                    writeln!(
                        output,
                        "{}                value = {};",
                        indent, metric.value
                    )?;
                    writeln!(
                        output,
                        "{}                unit = \"{}\";",
                        indent,
                        self.escape_string(&metric.unit)
                    )?;
                    writeln!(
                        output,
                        "{}                source = \"{}\";",
                        indent,
                        self.escape_string(&metric.source)
                    )?;
                    if let Some(comment) = &metric.comment {
                        writeln!(
                            output,
                            "{}                comment = \"{}\";",
                            indent,
                            self.escape_string(comment)
                        )?;
                    }
                    writeln!(output, "{}            }};", indent)?;
                }
                writeln!(output, "{}        }};", indent)?;
            }

            if let Some(comment) = &year_data.comment {
                writeln!(
                    output,
                    "{}        comment = \"{}\";",
                    indent,
                    self.escape_string(comment)
                )?;
            }

            writeln!(output, "{}    }};", indent)?;
        }

        if let Some(maps_to) = &data_block.maps_to {
            writeln!(
                output,
                "{}    @maps_to \"{}\";",
                indent,
                self.escape_string(maps_to)
            )?;
        }

        writeln!(output, "{}}};", indent)?;
        Ok(())
    }

    fn format_field_type(&self, field_type: &MdslFieldType) -> String {
        match field_type {
            MdslFieldType::Id => "ID".to_string(),
            MdslFieldType::Text(Some(length)) => format!("TEXT({})", length),
            MdslFieldType::Text(None) => "TEXT".to_string(),
            MdslFieldType::Number => "NUMBER".to_string(),
            MdslFieldType::Boolean => "BOOLEAN".to_string(),
            MdslFieldType::Category(values) => {
                let formatted_values: Vec<String> = values
                    .iter()
                    .map(|v| format!("\"{}\"", self.escape_string(v)))
                    .collect();
                format!("CATEGORY({})", formatted_values.join(", "))
            }
        }
    }

    /// Check if a value should be treated as numeric (not quoted)
    fn is_numeric_value(&self, value: &str) -> bool {
        value.parse::<f64>().is_ok() || value == "true" || value == "false"
    }

    fn escape_string(&self, s: &str) -> String {
        s.replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t")
    }

    /// Sanitize identifier to be valid MDSL identifier
    fn sanitize_identifier(&self, s: &str) -> String {
        let result: String = s.chars()
            .map(|c| match c {
                // Replace problematic characters
                'ä' | 'Ä' => "ae".to_string(),
                'ö' | 'Ö' => "oe".to_string(), 
                'ü' | 'Ü' => "ue".to_string(),
                'ß' => "ss".to_string(),
                '"' => "_QUOTE_".to_string(),
                ',' => "_COMMA_".to_string(),
                ' ' => "_".to_string(),
                '(' | ')' => "_".to_string(),
                '-' => "_".to_string(),
                '/' => "_".to_string(),
                '.' => "_".to_string(),
                ':' => "_".to_string(),
                // Keep alphanumeric and underscore
                c if c.is_alphanumeric() || c == '_' => c.to_string(),
                // Replace anything else with underscore
                _ => "_".to_string(),
            })
            .collect();
        
        // Ensure doesn't start with number
        if result.chars().next().map_or(false, |c| c.is_numeric()) {
            format!("_{}", result)
        } else {
            result
        }
    }
}
