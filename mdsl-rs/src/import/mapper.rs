//! Data mapping from SQL database to MDSL structures

use crate::error::Result;
use crate::import::connection::{DatabaseConnection, SchemaInfo, TableInfo};
use crate::import::{FieldTransform, MdslEntityType, MdslFieldType, TableMapping};
use std::collections::HashMap;

/// MDSL entity enum for unified handling
#[derive(Debug, Clone)]
pub enum MdslEntity {
    /// Unit entity (schema definition)
    Unit(MdslUnit),
    /// Vocabulary entity (enumeration)
    Vocabulary(MdslVocabulary),
    /// Family entity (media group)
    Family(MdslFamily),
    /// Outlet entity (media outlet)
    Outlet(MdslOutlet),
    /// Diachronic link entity (temporal relationship)
    DiachronicLink(MdslDiachronicLink),
    /// Synchronous link entity (contemporary relationship)
    SynchronousLink(MdslSynchronousLink),
    /// Data block entity (market data)
    DataBlock(MdslDataBlock),
}

/// Main container for MDSL data structures
#[derive(Debug, Clone)]
pub struct MdslData {
    /// Units (schema definitions)
    pub units: Vec<MdslUnit>,
    /// Vocabularies (enumerations)
    pub vocabularies: Vec<MdslVocabulary>,
    /// Families (media groups)
    pub families: Vec<MdslFamily>,
    /// Standalone outlets
    pub outlets: Vec<MdslOutlet>,
    /// Diachronic relationships
    pub diachronic_links: Vec<MdslDiachronicLink>,
    /// Synchronous relationships
    pub synchronous_links: Vec<MdslSynchronousLink>,
    /// Data blocks (market data)
    pub data_blocks: Vec<MdslDataBlock>,
}

impl MdslData {
    /// Convert to vector of entities for generation
    pub fn to_entities(&self) -> Vec<MdslEntity> {
        let mut entities = Vec::new();

        for unit in &self.units {
            entities.push(MdslEntity::Unit(unit.clone()));
        }

        for vocab in &self.vocabularies {
            entities.push(MdslEntity::Vocabulary(vocab.clone()));
        }

        for family in &self.families {
            entities.push(MdslEntity::Family(family.clone()));
        }

        for outlet in &self.outlets {
            entities.push(MdslEntity::Outlet(outlet.clone()));
        }

        for link in &self.diachronic_links {
            entities.push(MdslEntity::DiachronicLink(link.clone()));
        }

        for link in &self.synchronous_links {
            entities.push(MdslEntity::SynchronousLink(link.clone()));
        }

        for data in &self.data_blocks {
            entities.push(MdslEntity::DataBlock(data.clone()));
        }

        entities
    }
}

/// Data mapper that converts SQL database content to MDSL structures
pub struct DataMapper<'a> {
    table_mappings: &'a HashMap<String, TableMapping>,
}

/// MDSL Unit representation
#[derive(Debug, Clone)]
pub struct MdslUnit {
    /// Unit name
    pub name: String,
    /// Unit fields
    pub fields: Vec<MdslUnitField>,
}

/// MDSL Unit field
#[derive(Debug, Clone)]
pub struct MdslUnitField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: MdslFieldType,
    /// Whether this field is a primary key
    pub is_primary_key: bool,
    /// Optional field comment
    pub comment: Option<String>,
}

/// MDSL Vocabulary representation
#[derive(Debug, Clone)]
pub struct MdslVocabulary {
    /// Vocabulary name
    pub name: String,
    /// Vocabulary entries
    pub entries: Vec<MdslVocabularyEntry>,
}

/// MDSL Vocabulary entry
#[derive(Debug, Clone)]
pub struct MdslVocabularyEntry {
    /// Entry key
    pub key: String,
    /// Entry value
    pub value: String,
}

/// MDSL Family representation
#[derive(Debug, Clone)]
pub struct MdslFamily {
    /// Family name
    pub name: String,
    /// Optional family comment
    pub comment: Option<String>,
    /// Outlets belonging to this family
    pub outlets: Vec<MdslOutlet>,
    /// Diachronic links within this family
    pub diachronic_links: Vec<MdslDiachronicLink>,
    /// Synchronous links within this family
    pub synchronous_links: Vec<MdslSynchronousLink>,
    /// Data blocks for this family
    pub data_blocks: Vec<MdslDataBlock>,
}

/// MDSL Outlet representation
#[derive(Debug, Clone)]
pub struct MdslOutlet {
    /// Outlet ID
    pub id: Option<i64>,
    /// Outlet name
    pub name: String,
    /// Optional template extension
    pub extends_template: Option<String>,
    /// Optional base outlet ID
    pub based_on: Option<i64>,
    /// Identity attributes
    pub identity: HashMap<String, String>,
    /// Lifecycle statuses
    pub lifecycle: Vec<MdslLifecycleStatus>,
    /// Outlet characteristics
    pub characteristics: HashMap<String, String>,
    /// Outlet metadata
    pub metadata: HashMap<String, String>,
}

/// MDSL Lifecycle status
#[derive(Debug, Clone)]
pub struct MdslLifecycleStatus {
    /// Status name
    pub status: String,
    /// Start date
    pub start_date: Option<String>,
    /// End date
    pub end_date: Option<String>,
    /// Start date precision
    pub precision_start: Option<String>,
    /// End date precision
    pub precision_end: Option<String>,
    /// Status comment
    pub comment: Option<String>,
}

/// MDSL Diachronic link representation
#[derive(Debug, Clone)]
pub struct MdslDiachronicLink {
    /// Link name
    pub name: String,
    /// Predecessor outlet ID
    pub predecessor: i64,
    /// Successor outlet ID
    pub successor: i64,
    /// Event date
    pub event_date: Option<String>,
    /// Relationship type
    pub relationship_type: String,
    /// Optional comment
    pub comment: Option<String>,
    /// Optional mapping reference
    pub maps_to: Option<String>,
}

/// MDSL Synchronous link representation
#[derive(Debug, Clone)]
pub struct MdslSynchronousLink {
    /// Link name
    pub name: String,
    /// First outlet reference
    pub outlet_1: MdslOutletRef,
    /// Second outlet reference
    pub outlet_2: MdslOutletRef,
    /// Relationship type
    pub relationship_type: String,
    /// Period start date
    pub period_start: Option<String>,
    /// Period end date
    pub period_end: Option<String>,
    /// Additional details
    pub details: Option<String>,
    /// Optional mapping reference
    pub maps_to: Option<String>,
}

/// MDSL Outlet reference
#[derive(Debug, Clone)]
pub struct MdslOutletRef {
    /// Outlet ID
    pub id: i64,
    /// Outlet role in relationship
    pub role: Option<String>,
}

/// MDSL Data block representation
#[derive(Debug, Clone)]
pub struct MdslDataBlock {
    /// Target outlet ID
    pub outlet_id: i64,
    /// Aggregation settings
    pub aggregation: HashMap<String, String>,
    /// Yearly data
    pub years: Vec<MdslDataYear>,
    /// Optional mapping reference
    pub maps_to: Option<String>,
}

/// MDSL Data year
#[derive(Debug, Clone)]
pub struct MdslDataYear {
    /// Year value
    pub year: i32,
    /// Metrics for this year
    pub metrics: Vec<MdslMetric>,
    /// Optional comment
    pub comment: Option<String>,
}

/// MDSL Metric
#[derive(Debug, Clone)]
pub struct MdslMetric {
    /// Metric name
    pub name: String,
    /// Metric value
    pub value: f64,
    /// Metric unit
    pub unit: String,
    /// Data source
    pub source: String,
    /// Optional comment
    pub comment: Option<String>,
}

impl<'a> DataMapper<'a> {
    /// Create a new data mapper
    pub fn new(table_mappings: &'a HashMap<String, TableMapping>) -> Self {
        Self { table_mappings }
    }

    /// Map database content to MDSL structures
    pub async fn map_database_to_mdsl(
        &self,
        connection: &DatabaseConnection,
        schema_info: &SchemaInfo,
    ) -> Result<MdslData> {
        let mut mdsl_data = MdslData {
            units: Vec::new(),
            vocabularies: Vec::new(),
            families: Vec::new(),
            outlets: Vec::new(),
            diachronic_links: Vec::new(),
            synchronous_links: Vec::new(),
            data_blocks: Vec::new(),
        };

        // Process each table according to its mapping
        for (table_name, table_info) in &schema_info.tables {
            if let Some(mapping) = self.table_mappings.get(table_name) {
                match &mapping.entity_type {
                    MdslEntityType::Unit => {
                        let unit = self.map_table_to_unit(table_info, mapping).await?;
                        mdsl_data.units.push(unit);
                    }
                    MdslEntityType::Vocabulary => {
                        let vocab = self
                            .map_table_to_vocabulary(connection, table_info, mapping)
                            .await?;
                        mdsl_data.vocabularies.push(vocab);
                    }
                    MdslEntityType::Family => {
                        let family = self
                            .map_table_to_family(connection, table_info, mapping)
                            .await?;
                        mdsl_data.families.push(family);
                    }
                    MdslEntityType::Outlet => {
                        let outlets = self
                            .map_table_to_outlets(connection, table_info, mapping)
                            .await?;
                        mdsl_data.outlets.extend(outlets);
                    }
                    MdslEntityType::DiachronicLink => {
                        let links = self
                            .map_table_to_diachronic_links(connection, table_info, mapping)
                            .await?;
                        mdsl_data.diachronic_links.extend(links);
                    }
                    MdslEntityType::SynchronousLink => {
                        let links = self
                            .map_table_to_synchronous_links(connection, table_info, mapping)
                            .await?;
                        mdsl_data.synchronous_links.extend(links);
                    }
                    MdslEntityType::DataBlock => {
                        let data_blocks = self
                            .map_table_to_data_blocks(connection, table_info, mapping)
                            .await?;
                        mdsl_data.data_blocks.extend(data_blocks);
                    }
                }
            }
        }

        // Group outlets into families based on relationships
        self.organize_outlets_into_families(&mut mdsl_data)?;

        Ok(mdsl_data)
    }

    /// Map a table to a UNIT declaration
    async fn map_table_to_unit(
        &self,
        table_info: &TableInfo,
        mapping: &TableMapping,
    ) -> Result<MdslUnit> {
        let mut fields = Vec::new();

        for column in &table_info.columns {
            if let Some(field_mapping) = mapping.field_mappings.get(&column.name) {
                let field = MdslUnitField {
                    name: field_mapping.mdsl_field.clone(),
                    field_type: MdslFieldType::Text(None), // Default type, should be configured
                    is_primary_key: table_info.primary_keys.contains(&column.name),
                    comment: None,
                };
                fields.push(field);
            }
        }

        Ok(MdslUnit {
            name: format!("Unit_{}", table_info.name),
            fields,
        })
    }

    /// Map a table to a VOCABULARY declaration
    async fn map_table_to_vocabulary(
        &self,
        connection: &DatabaseConnection,
        table_info: &TableInfo,
        _mapping: &TableMapping,
    ) -> Result<MdslVocabulary> {
        // Assuming vocabulary tables have key-value structure
        let data = connection
            .query_table_data(&table_info.name, None, Some(100))
            .await?;

        let mut entries = Vec::new();
        for row in data {
            // Get first two values as key-value pair
            let values: Vec<_> = row.values().collect();
            if values.len() >= 2 {
                entries.push(MdslVocabularyEntry {
                    key: values[0].clone(),
                    value: values[1].clone(),
                });
            }
        }

        Ok(MdslVocabulary {
            name: format!("Vocab_{}", table_info.name),
            entries,
        })
    }

    /// Map a table to FAMILY declaration
    async fn map_table_to_family(
        &self,
        _connection: &DatabaseConnection,
        table_info: &TableInfo,
        _mapping: &TableMapping,
    ) -> Result<MdslFamily> {
        // Families are typically derived from groupings, not direct table mappings
        Ok(MdslFamily {
            name: format!("Family_{}", table_info.name),
            comment: None,
            outlets: Vec::new(),
            diachronic_links: Vec::new(),
            synchronous_links: Vec::new(),
            data_blocks: Vec::new(),
        })
    }

    /// Map a table to OUTLET declarations
    async fn map_table_to_outlets(
        &self,
        connection: &DatabaseConnection,
        table_info: &TableInfo,
        mapping: &TableMapping,
    ) -> Result<Vec<MdslOutlet>> {
        let data = connection
            .query_table_data(&table_info.name, None, None)
            .await?;

        let mut outlets = Vec::new();

        for row in data {
            let outlet = self.map_row_to_outlet(&row, mapping)?;
            outlets.push(outlet);
        }

        Ok(outlets)
    }

    /// Map a database row to an OUTLET
    fn map_row_to_outlet(
        &self,
        row: &std::collections::HashMap<String, String>,
        mapping: &TableMapping,
    ) -> Result<MdslOutlet> {
        let mut outlet = MdslOutlet {
            id: None,
            name: "Unknown".to_string(),
            extends_template: None,
            based_on: None,
            identity: HashMap::new(),
            lifecycle: Vec::new(),
            characteristics: HashMap::new(),
            metadata: HashMap::new(),
        };

        // Map fields according to mapping configuration
        for (column_name, value) in row {
            if let Some(field_mapping) = mapping.field_mappings.get(column_name) {
                if !value.is_empty() && value != "NULL" {
                    let transformed_value =
                        self.apply_field_transform(value, &field_mapping.transform)?;

                    match field_mapping.mdsl_field.as_str() {
                        "id" => {
                            outlet.id = transformed_value.parse().ok();
                        }
                        "title" | "name" => {
                            outlet.name = transformed_value;
                        }
                        field_name if field_name.starts_with("identity.") => {
                            let key = field_name.strip_prefix("identity.").unwrap_or(field_name);
                            outlet.identity.insert(key.to_string(), transformed_value);
                        }
                        field_name if field_name.starts_with("characteristics.") => {
                            let key = field_name
                                .strip_prefix("characteristics.")
                                .unwrap_or(field_name);
                            outlet
                                .characteristics
                                .insert(key.to_string(), transformed_value);
                        }
                        field_name if field_name.starts_with("metadata.") => {
                            let key = field_name.strip_prefix("metadata.").unwrap_or(field_name);
                            outlet.metadata.insert(key.to_string(), transformed_value);
                        }
                        _ => {
                            // Default to characteristics
                            outlet
                                .characteristics
                                .insert(field_mapping.mdsl_field.clone(), transformed_value);
                        }
                    }
                }
            }
        }

        // Create lifecycle status from date fields
        outlet.lifecycle = self.extract_lifecycle_from_row(row, mapping)?;

        Ok(outlet)
    }

    /// Extract lifecycle information from database row
    fn extract_lifecycle_from_row(
        &self,
        row: &std::collections::HashMap<String, String>,
        _mapping: &TableMapping,
    ) -> Result<Vec<MdslLifecycleStatus>> {
        // Look for common date patterns
        let mut start_date = None;
        let mut end_date = None;
        let mut status = "active".to_string();

        for (column_name, value) in row {
            if !value.is_empty() && value != "NULL" {
                match column_name.to_lowercase().as_str() {
                    "start_year" | "start_date" | "founded" | "launch_date" => {
                        start_date = Some(self.format_date_from_parts(value, None, None));
                    }
                    "end_year" | "end_date" | "closed" | "shutdown_date" => {
                        if value != "9999" && value != "current" && !value.is_empty() {
                            end_date = Some(self.format_date_from_parts(value, None, None));
                            status = "inactive".to_string();
                        }
                    }
                    "current_status_text" | "status" => {
                        status = value.clone();
                    }
                    _ => {}
                }
            }
        }

        let lifecycle_status = MdslLifecycleStatus {
            status,
            start_date,
            end_date: end_date.clone(),
            precision_start: Some("known".to_string()),
            precision_end: if end_date.is_some() {
                Some("known".to_string())
            } else {
                None
            },
            comment: None,
        };

        Ok(vec![lifecycle_status])
    }

    /// Format date from individual parts
    fn format_date_from_parts(&self, year: &str, month: Option<&str>, day: Option<&str>) -> String {
        match (month, day) {
            (Some(m), Some(d)) => format!(
                "{}-{:02}-{:02}",
                year,
                m.parse::<u32>().unwrap_or(1),
                d.parse::<u32>().unwrap_or(1)
            ),
            (Some(m), None) => format!("{}-{:02}-01", year, m.parse::<u32>().unwrap_or(1)),
            _ => format!("{}-01-01", year),
        }
    }

    /// Map a table to diachronic links
    async fn map_table_to_diachronic_links(
        &self,
        connection: &DatabaseConnection,
        table_info: &TableInfo,
        mapping: &TableMapping,
    ) -> Result<Vec<MdslDiachronicLink>> {
        let data = connection
            .query_table_data(&table_info.name, None, None)
            .await?;

        let mut links = Vec::new();

        for (index, row) in data.iter().enumerate() {
            let link = self.map_row_to_diachronic_link(row, mapping, index)?;
            links.push(link);
        }

        Ok(links)
    }

    /// Map a database row to a diachronic link
    fn map_row_to_diachronic_link(
        &self,
        row: &std::collections::HashMap<String, String>,
        _mapping: &TableMapping,
        index: usize,
    ) -> Result<MdslDiachronicLink> {
        let mut predecessor = 0;
        let mut successor = 0;
        let mut relationship_type = "Unknown".to_string();
        let mut event_date = None;
        let mut comment = None;

        for (column_name, value) in row {
            if !value.is_empty() && value != "NULL" {
                match column_name.to_lowercase().as_str() {
                    "id_mo_predecessor" | "predecessor_id" | "source_id" => {
                        predecessor = value.parse().unwrap_or(0);
                    }
                    "id_mo_successor" | "successor_id" | "target_id" => {
                        successor = value.parse().unwrap_or(0);
                    }
                    "relationship_type" | "type" => {
                        relationship_type = value.clone();
                    }
                    "event_date" | "date" => {
                        event_date = Some(value.clone());
                    }
                    "event_year" => {
                        event_date = Some(format!("{}-01-01", value));
                    }
                    "comments" | "comment" | "details" => {
                        comment = Some(value.clone());
                    }
                    _ => {}
                }
            }
        }

        Ok(MdslDiachronicLink {
            name: format!("diachronic_link_{}", index),
            predecessor,
            successor,
            event_date,
            relationship_type,
            comment,
            maps_to: None,
        })
    }

    /// Map a table to synchronous links
    async fn map_table_to_synchronous_links(
        &self,
        connection: &DatabaseConnection,
        table_info: &TableInfo,
        mapping: &TableMapping,
    ) -> Result<Vec<MdslSynchronousLink>> {
        let data = connection
            .query_table_data(&table_info.name, None, None)
            .await?;

        let mut links = Vec::new();

        for (index, row) in data.iter().enumerate() {
            let link = self.map_row_to_synchronous_link(row, mapping, index)?;
            links.push(link);
        }

        Ok(links)
    }

    /// Map a database row to a synchronous link
    fn map_row_to_synchronous_link(
        &self,
        row: &std::collections::HashMap<String, String>,
        _mapping: &TableMapping,
        index: usize,
    ) -> Result<MdslSynchronousLink> {
        let mut outlet_1_id = 0;
        let mut outlet_2_id = 0;
        let mut relationship_type = "Unknown".to_string();
        let mut period_start = None;
        let mut period_end = None;
        let mut details = None;

        for (column_name, value) in row {
            if !value.is_empty() && value != "NULL" {
                match column_name.to_lowercase().as_str() {
                    "id_mo_1" | "outlet_1_id" | "source_id" => {
                        outlet_1_id = value.parse().unwrap_or(0);
                    }
                    "id_mo_2" | "outlet_2_id" | "target_id" => {
                        outlet_2_id = value.parse().unwrap_or(0);
                    }
                    "relationship_type" | "type" => {
                        relationship_type = value.clone();
                    }
                    "start_year" | "period_start" => {
                        period_start = Some(format!("{}-01-01", value));
                    }
                    "end_year" | "period_end" => {
                        if value != "9999" && !value.is_empty() {
                            period_end = Some(format!("{}-12-31", value));
                        }
                    }
                    "comments" | "comment" | "details" => {
                        details = Some(value.clone());
                    }
                    _ => {}
                }
            }
        }

        Ok(MdslSynchronousLink {
            name: format!("synchronous_link_{}", index),
            outlet_1: MdslOutletRef {
                id: outlet_1_id,
                role: None,
            },
            outlet_2: MdslOutletRef {
                id: outlet_2_id,
                role: None,
            },
            relationship_type,
            period_start,
            period_end,
            details,
            maps_to: None,
        })
    }

    /// Map a table to data blocks
    async fn map_table_to_data_blocks(
        &self,
        _connection: &DatabaseConnection,
        _table_info: &TableInfo,
        _mapping: &TableMapping,
    ) -> Result<Vec<MdslDataBlock>> {
        // Data blocks are complex and would require multiple tables
        // This is a placeholder implementation
        Ok(Vec::new())
    }

    /// Apply field transformation
    fn apply_field_transform(
        &self,
        value: &str,
        transform: &Option<FieldTransform>,
    ) -> Result<String> {
        match transform {
            Some(FieldTransform::Direct) | None => Ok(value.to_string()),
            Some(FieldTransform::Date(format_str)) => {
                // Apply date formatting (use format_str or default)
                if format_str.is_empty() {
                    Ok(value.to_string())
                } else {
                    Ok(format_str.replace("{}", value))
                }
            }
            Some(FieldTransform::Lookup(_table)) => {
                // Lookup transformation would require database access
                Ok(value.to_string())
            }
            Some(FieldTransform::Concat(_fields)) => {
                // Concatenation would require multiple field values
                Ok(value.to_string())
            }
        }
    }

    /// Organize standalone outlets into families based on relationships
    fn organize_outlets_into_families(&self, mdsl_data: &mut MdslData) -> Result<()> {
        // This is a complex process that would analyze relationships
        // to group outlets into logical families
        // For now, we'll create a simple grouping

        if !mdsl_data.outlets.is_empty() {
            let default_family = MdslFamily {
                name: "Default Media Group".to_string(),
                comment: Some("Auto-generated family for imported outlets".to_string()),
                outlets: std::mem::take(&mut mdsl_data.outlets),
                diachronic_links: std::mem::take(&mut mdsl_data.diachronic_links),
                synchronous_links: std::mem::take(&mut mdsl_data.synchronous_links),
                data_blocks: std::mem::take(&mut mdsl_data.data_blocks),
            };

            mdsl_data.families.push(default_family);
        }

        Ok(())
    }
}
