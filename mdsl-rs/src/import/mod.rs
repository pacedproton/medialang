//! SQL database import functionality for MDSL
//!
//! This module provides functionality to import data from SQL databases
//! and convert it to MDSL format, including both diachronic (temporal)
//! and synchronic (contemporary) relationships.

use crate::error::{Error, Result};
use std::collections::HashMap;

pub mod connection;
pub mod generator;
pub mod mapper;

/// Database configuration for SQL import
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database type identifier
    pub db_type: DatabaseType,
    /// Connection string for the database
    pub connection_string: String,
    /// Optional schema name
    pub schema: Option<String>,
}

/// Supported database types for import
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseType {
    /// PostgreSQL database
    PostgreSQL,
    /// MySQL database
    MySQL,
    /// SQLite database
    SQLite,
    /// Microsoft SQL Server database
    SqlServer,
}

/// Table mapping configuration for SQL to MDSL conversion
#[derive(Debug, Clone)]
pub struct TableMapping {
    /// Target MDSL entity type for this table
    pub entity_type: MdslEntityType,
    /// Field mappings from SQL columns to MDSL fields
    pub field_mappings: HashMap<String, FieldMapping>,
    /// Optional relationship mapping configuration
    pub relationship_mapping: Option<RelationshipMapping>,
    /// Optional temporal field configuration
    pub temporal_fields: Option<TemporalFields>,
    /// Whether this table represents an ANMI schema pattern
    pub is_anmi_schema: bool,
}

/// MDSL entity type classification
#[derive(Debug, Clone, PartialEq)]
pub enum MdslEntityType {
    /// Unit (schema definition)
    Unit,
    /// Vocabulary (enumeration)
    Vocabulary,
    /// Family (media group)
    Family,
    /// Outlet (media outlet)
    Outlet,
    /// Diachronic relationship (temporal link)
    DiachronicLink,
    /// Synchronous relationship (contemporary link)
    SynchronousLink,
    /// Data block (market data)
    DataBlock,
}

/// Field mapping from SQL column to MDSL field
#[derive(Debug, Clone)]
pub struct FieldMapping {
    /// SQL column name
    pub sql_column: String,
    /// Target MDSL field name
    pub mdsl_field: String,
    /// Optional field transformation
    pub transform: Option<FieldTransform>,
    /// Whether this field is required
    pub required: bool,
}

/// Field transformation types
#[derive(Debug, Clone)]
pub enum FieldTransform {
    /// Direct mapping (no transformation)
    Direct,
    /// Date formatting transformation
    Date(String),
    /// String concatenation transformation
    Concat(Vec<String>),
    /// Lookup transformation using a mapping table
    Lookup(String),
}

/// MDSL field type specification
#[derive(Debug, Clone)]
pub enum MdslFieldType {
    /// ID field type
    Id,
    /// Text field with optional length limit
    Text(Option<usize>),
    /// Numeric field type
    Number,
    /// Boolean field type
    Boolean,
    /// Category field with predefined values
    Category(Vec<String>),
}

/// Vocabulary mapping configuration
#[derive(Debug, Clone)]
pub struct VocabularyMapping {
    /// Source table name
    pub table: String,
    /// Column containing the key values
    pub key_field: String,
    /// Column containing the display values
    pub value_field: String,
}

/// Relationship mapping configuration
#[derive(Debug, Clone)]
pub struct RelationshipMapping {
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Source entity field
    pub source_field: String,
    /// Target entity field
    pub target_field: String,
    /// Optional relationship metadata fields
    pub metadata_fields: HashMap<String, String>,
}

/// Relationship type classification
#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipType {
    /// Diachronic (temporal) relationship
    Diachronic,
    /// Synchronous (contemporary) relationship
    Synchronous,
}

/// Temporal field configuration for lifecycle tracking
#[derive(Debug, Clone)]
pub struct TemporalFields {
    /// Start date field name
    pub start_date: Option<String>,
    /// End date field name
    pub end_date: Option<String>,
    /// Status field name
    pub status_field: Option<String>,
}

/// Main SQL importer
pub struct SqlImporter {
    /// Database configuration
    pub config: DatabaseConfig,
    /// Table mappings for conversion
    pub table_mappings: HashMap<String, TableMapping>,
}

impl SqlImporter {
    /// Create a new SQL importer with configuration
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            config,
            table_mappings: HashMap::new(),
        }
    }

    /// Add a table mapping for SQL to MDSL conversion
    pub fn add_table_mapping(&mut self, table_name: String, mapping: TableMapping) {
        self.table_mappings.insert(table_name, mapping);
    }

    /// Import database content and return MDSL data structures
    pub async fn import_database(&self) -> Result<mapper::MdslData> {
        // Create and connect to database
        let mut connection = connection::DatabaseConnection::new(self.config.clone())?;
        connection.connect().await?;

        // Extract schema from connection string if present
        let schema = if let Some(schema_param) = self.config.connection_string.split('?').nth(1) {
            schema_param.split('&').find_map(|param| {
                if param.starts_with("schema=") {
                    Some(&param[7..])
                } else {
                    None
                }
            })
        } else {
            None
        };

        // Analyze schema
        let schema_info = connection.analyze_schema(schema).await?;

        // Map data to MDSL structures
        let mapper = mapper::DataMapper::new(&self.table_mappings);
        mapper.map_database_to_mdsl(&connection, &schema_info).await
    }

    /// Import database content and generate MDSL code in one step
    pub async fn import_and_generate(&self) -> Result<String> {
        let mdsl_data = self.import_database().await?;
        let generator = generator::MdslGenerator::new();
        generator.generate(&mdsl_data.to_entities())
    }

    /// Import from SQL database and return MDSL data structures
    pub async fn import_to_structures(&self) -> Result<mapper::MdslData> {
        // Create and connect to database
        let mut connection = connection::DatabaseConnection::new(self.config.clone())?;
        connection.connect().await?;

        // Extract schema from connection string if present
        let schema = if let Some(schema_param) = self.config.connection_string.split('?').nth(1) {
            schema_param.split('&').find_map(|param| {
                if param.starts_with("schema=") {
                    Some(&param[7..])
                } else {
                    None
                }
            })
        } else {
            None
        };

        let schema_info = connection.analyze_schema(schema).await?;

        let mapper = mapper::DataMapper::new(&self.table_mappings);
        let mdsl_data = mapper
            .map_database_to_mdsl(&connection, &schema_info)
            .await?;

        Ok(mdsl_data)
    }

    /// Generate MDSL code from existing data structures
    pub fn generate_mdsl(&self, mdsl_data: &mapper::MdslData) -> Result<String> {
        let generator = generator::MdslGenerator::new();
        generator.generate(&mdsl_data.to_entities())
    }

    /// Auto-detect ANMI schema patterns in database
    pub async fn auto_detect_anmi_patterns(&self) -> Result<Vec<TableMapping>> {
        // Create and connect to database
        let mut connection = connection::DatabaseConnection::new(self.config.clone())?;
        connection.connect().await?;

        // Extract schema from connection string or use default
        let schema = if let Some(schema_param) = self.config.connection_string.split('?').nth(1) {
            schema_param.split('&').find_map(|param| {
                if param.starts_with("schema=") {
                    Some(&param[7..])
                } else {
                    None
                }
            })
        } else {
            self.config.schema.as_deref()
        };

        let schema_info = connection.analyze_schema(schema).await?;

        let mut mappings = Vec::new();

        // Look for ANMI patterns (tables with id_mo, mo_title, etc.)
        for (table_name, _table_info) in &schema_info.tables {
            if self.is_anmi_table(table_name) {
                let mapping = self.create_anmi_mapping(table_name)?;
                mappings.push(mapping);
            }
        }

        Ok(mappings)
    }

    /// Check if a table follows ANMI naming patterns (updated for real schema)
    fn is_anmi_table(&self, table_name: &str) -> bool {
        // Real ANMI table patterns from graphv3 schema
        match table_name {
            "mo_constant" => true,
            "mo_year" => true,
            "sources_names" => true,
            // Numbered relationship tables
            _ if table_name.starts_with("11_") => true, // succession
            _ if table_name.starts_with("12_") => true, // amalgamation
            _ if table_name.starts_with("13_") => true, // new_distribution_area
            _ if table_name.starts_with("14_") => true, // new_sector
            _ if table_name.starts_with("19_") => true, // interruption
            _ if table_name.starts_with("21_") => true, // split_off
            _ if table_name.starts_with("22_") => true, // offshoot
            _ if table_name.starts_with("23_") => true, // merger
            _ if table_name.starts_with("31_") => true, // main_media_outlet
            _ if table_name.starts_with("33_") => true, // umbrella
            _ if table_name.starts_with("34_") => true, // collaboration
            _ => false,
        }
    }

    /// Create ANMI table mapping based on real schema patterns
    fn create_anmi_mapping(&self, table_name: &str) -> Result<TableMapping> {
        let (entity_type, field_mappings) = match table_name {
            "mo_constant" => {
                let mut mappings = HashMap::new();

                // Real mo_constant table mappings
                mappings.insert(
                    "id".to_string(),
                    FieldMapping {
                        sql_column: "id_mo".to_string(),
                        mdsl_field: "id".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: true,
                    },
                );

                mappings.insert(
                    "title".to_string(),
                    FieldMapping {
                        sql_column: "mo_title".to_string(),
                        mdsl_field: "identity.title".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: true,
                    },
                );

                mappings.insert(
                    "sector".to_string(),
                    FieldMapping {
                        sql_column: "id_sector".to_string(),
                        mdsl_field: "characteristics.sector".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: false,
                    },
                );

                mappings.insert(
                    "mandate".to_string(),
                    FieldMapping {
                        sql_column: "mandate".to_string(),
                        mdsl_field: "characteristics.mandate".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: false,
                    },
                );

                mappings.insert(
                    "location".to_string(),
                    FieldMapping {
                        sql_column: "location".to_string(),
                        mdsl_field: "characteristics.location".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: false,
                    },
                );

                mappings.insert(
                    "start_date".to_string(),
                    FieldMapping {
                        sql_column: "start_date".to_string(),
                        mdsl_field: "lifecycle.start_date".to_string(),
                        transform: Some(FieldTransform::Date("".to_string())),
                        required: false,
                    },
                );

                mappings.insert(
                    "end_date".to_string(),
                    FieldMapping {
                        sql_column: "end_date".to_string(),
                        mdsl_field: "lifecycle.end_date".to_string(),
                        transform: Some(FieldTransform::Date("".to_string())),
                        required: false,
                    },
                );

                mappings.insert(
                    "editorial_line_s".to_string(),
                    FieldMapping {
                        sql_column: "editorial_line_s".to_string(),
                        mdsl_field: "characteristics.editorial_line_self".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: false,
                    },
                );

                mappings.insert(
                    "editorial_line_e".to_string(),
                    FieldMapping {
                        sql_column: "editorial_line_e".to_string(),
                        mdsl_field: "characteristics.editorial_line_external".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: false,
                    },
                );

                (MdslEntityType::Outlet, mappings)
            }

            "mo_year" => {
                let mut mappings = HashMap::new();

                mappings.insert(
                    "id_mo".to_string(),
                    FieldMapping {
                        sql_column: "id_mo".to_string(),
                        mdsl_field: "outlet_id".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: true,
                    },
                );

                mappings.insert(
                    "year".to_string(),
                    FieldMapping {
                        sql_column: "year".to_string(),
                        mdsl_field: "year".to_string(),
                        transform: Some(FieldTransform::Date("".to_string())),
                        required: true,
                    },
                );

                mappings.insert(
                    "circulation".to_string(),
                    FieldMapping {
                        sql_column: "circulation".to_string(),
                        mdsl_field: "metrics.circulation".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: false,
                    },
                );

                mappings.insert(
                    "reach_nat".to_string(),
                    FieldMapping {
                        sql_column: "reach_nat".to_string(),
                        mdsl_field: "metrics.reach_national".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: false,
                    },
                );

                mappings.insert(
                    "market_share".to_string(),
                    FieldMapping {
                        sql_column: "market_share".to_string(),
                        mdsl_field: "metrics.market_share".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: false,
                    },
                );

                (MdslEntityType::DataBlock, mappings)
            }

            "sources_names" => {
                let mut mappings = HashMap::new();

                mappings.insert(
                    "id_source".to_string(),
                    FieldMapping {
                        sql_column: "id_source".to_string(),
                        mdsl_field: "id".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: true,
                    },
                );

                mappings.insert(
                    "source_name".to_string(),
                    FieldMapping {
                        sql_column: "source_name".to_string(),
                        mdsl_field: "name".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: true,
                    },
                );

                (MdslEntityType::Vocabulary, mappings)
            }

            // Diachronic relationship tables (11_, 12_, 13_, 14_, 19_, 21_, 22_, 23_)
            _ if table_name.starts_with("11_")
                || table_name.starts_with("12_")
                || table_name.starts_with("13_")
                || table_name.starts_with("14_")
                || table_name.starts_with("19_")
                || table_name.starts_with("21_")
                || table_name.starts_with("22_")
                || table_name.starts_with("23_") =>
            {
                let mut mappings = HashMap::new();

                mappings.insert(
                    "source_id".to_string(),
                    FieldMapping {
                        sql_column: "id_mo".to_string(),
                        mdsl_field: "source_outlet".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: true,
                    },
                );

                // Extract relationship type from table name
                let rel_type = table_name.split('_').skip(1).collect::<Vec<_>>().join("_");
                mappings.insert(
                    "target_id".to_string(),
                    FieldMapping {
                        sql_column: rel_type,
                        mdsl_field: "target_outlet".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: true,
                    },
                );

                (MdslEntityType::DiachronicLink, mappings)
            }

            // Synchronous relationship tables (31_, 33_, 34_)
            _ if table_name.starts_with("31_")
                || table_name.starts_with("33_")
                || table_name.starts_with("34_") =>
            {
                let mut mappings = HashMap::new();

                mappings.insert(
                    "source_id".to_string(),
                    FieldMapping {
                        sql_column: "id_mo".to_string(),
                        mdsl_field: "source_outlet".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: true,
                    },
                );

                // Extract relationship type from table name
                let rel_type = table_name.split('_').skip(1).collect::<Vec<_>>().join("_");
                mappings.insert(
                    "target_id".to_string(),
                    FieldMapping {
                        sql_column: rel_type,
                        mdsl_field: "target_outlet".to_string(),
                        transform: Some(FieldTransform::Direct),
                        required: true,
                    },
                );

                mappings.insert(
                    "start_rel".to_string(),
                    FieldMapping {
                        sql_column: "start_rel".to_string(),
                        mdsl_field: "start_date".to_string(),
                        transform: Some(FieldTransform::Date("".to_string())),
                        required: false,
                    },
                );

                mappings.insert(
                    "end_rel".to_string(),
                    FieldMapping {
                        sql_column: "end_rel".to_string(),
                        mdsl_field: "end_date".to_string(),
                        transform: Some(FieldTransform::Date("".to_string())),
                        required: false,
                    },
                );

                (MdslEntityType::SynchronousLink, mappings)
            }

            _ => {
                return Err(Error::NotImplemented(format!(
                    "Unknown ANMI table pattern: {}",
                    table_name
                )));
            }
        };

        Ok(TableMapping {
            entity_type,
            field_mappings,
            relationship_mapping: None,
            temporal_fields: None,
            is_anmi_schema: true,
        })
    }

    /// Generate complete MDSL from ANMI database with proper entity relationships
    pub async fn generate_complete_mdsl(&self) -> Result<String> {
        println!("🔄 Starting comprehensive MDSL generation from ANMI database...");

        // Connect to database
        let mut connection = connection::DatabaseConnection::new(self.config.clone())?;
        connection.connect().await?;

        println!("✅ Connected to ANMI database");

        // Extract and process all data
        let mo_constant_data = self.extract_media_outlets(&connection).await?;
        let mo_year_data = self.extract_market_data(&connection).await?;
        let sources_data = self.extract_sources(&connection).await?;
        let relationships_data = self.extract_relationships(&connection).await?;

        println!(
            "✅ Extracted {} media outlets, {} market data records, {} sources, {} relationships",
            mo_constant_data.len(),
            mo_year_data.len(),
            sources_data.len(),
            relationships_data.len()
        );

        // Generate MDSL structures
        let mut output = String::new();

        // Generate header and imports
        self.generate_mdsl_header(&mut output)?;

        // Generate vocabulary from sources
        self.generate_vocabulary_from_sources(&mut output, &sources_data)?;

        // Generate media outlet units and families
        self.generate_media_outlets_mdsl(&mut output, &mo_constant_data)?;

        // Generate relationships
        self.generate_relationships_mdsl(&mut output, &relationships_data)?;

        // Generate data blocks from market data, filtering for outlets that exist
        self.generate_data_blocks_mdsl(&mut output, &mo_year_data, &mo_constant_data)?;

        println!("🎉 Generated complete MDSL file");
        Ok(output)
    }

    /// Extract media outlet data from mo_constant table
    async fn extract_media_outlets(
        &self,
        connection: &connection::DatabaseConnection,
    ) -> Result<Vec<MediaOutletData>> {
        // Query to find all outlets connected to ORF outlets through any relationship
        let query = r#"
        WITH orf_outlets AS (
            SELECT id_mo
            FROM graphv3.mo_constant
            WHERE mo_title ILIKE '%orf%'
        ),
        connected_outlets AS (
            -- Include direct ORF outlets
            SELECT id_mo as outlet_id FROM orf_outlets
            
            UNION
            
            -- From contemporary relationships (bilateral)
            SELECT DISTINCT unnest(ARRAY[id_mo, main_media_outlet]) as outlet_id
            FROM graphv3."31_main_media_outlet" 
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets) 
               OR main_media_outlet IN (SELECT id_mo FROM orf_outlets)
            
            UNION
            
            SELECT DISTINCT unnest(ARRAY[id_mo, umbrella]) as outlet_id
            FROM graphv3."33_umbrella"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR umbrella IN (SELECT id_mo FROM orf_outlets)
               
            UNION
            
            SELECT DISTINCT unnest(ARRAY[id_mo, collaboration]) as outlet_id
            FROM graphv3."34_collaboration"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR collaboration IN (SELECT id_mo FROM orf_outlets)
               
            UNION
            
            -- From historical relationships (bilateral)
            SELECT DISTINCT unnest(ARRAY[id_mo, succession]) as outlet_id
            FROM graphv3."11_succession"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR succession IN (SELECT id_mo FROM orf_outlets)
               
            UNION
            
            SELECT DISTINCT unnest(ARRAY[id_mo, amalgamation]) as outlet_id
            FROM graphv3."12_amalgamation"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR amalgamation IN (SELECT id_mo FROM orf_outlets)
               
            UNION
            
            SELECT DISTINCT unnest(ARRAY[id_mo, new_distribution_area]) as outlet_id
            FROM graphv3."13_new_distribution_area"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR new_distribution_area IN (SELECT id_mo FROM orf_outlets)
               
            UNION
            
            SELECT DISTINCT unnest(ARRAY[id_mo, new_sector]) as outlet_id
            FROM graphv3."14_new_sector"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR new_sector IN (SELECT id_mo FROM orf_outlets)
               
            UNION
            
            SELECT DISTINCT unnest(ARRAY[id_mo, interruption]) as outlet_id
            FROM graphv3."19_interruption"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR interruption IN (SELECT id_mo FROM orf_outlets)
               
            UNION
            
            SELECT DISTINCT unnest(ARRAY[id_mo, split_off]) as outlet_id
            FROM graphv3."21_split_off"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR split_off IN (SELECT id_mo FROM orf_outlets)
               
            UNION
            
            SELECT DISTINCT unnest(ARRAY[id_mo, offshoot]) as outlet_id
            FROM graphv3."22_offshoot"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR offshoot IN (SELECT id_mo FROM orf_outlets)
               
            UNION
            
            SELECT DISTINCT unnest(ARRAY[id_mo, merger]) as outlet_id
            FROM graphv3."23_merger"
            WHERE id_mo IN (SELECT id_mo FROM orf_outlets)
               OR merger IN (SELECT id_mo FROM orf_outlets)
        )
        SELECT mo.id_mo, mo.mo_title, mo.id_sector, mo.mandate, mo.location, mo.primary_distr_area, mo.local, mo.language, mo.start_date, mo.start_fake_date, mo.end_date, mo.end_fake_date, mo.editorial_line_s, mo.editorial_line_e, mo.comments 
        FROM graphv3.mo_constant mo
        JOIN connected_outlets co ON mo.id_mo = co.outlet_id
        WHERE mo.mo_title IS NOT NULL
        ORDER BY mo.mo_title
        "#;

        let rows = connection.execute_query(query).await?;
        let mut outlets = Vec::new();

        for row in rows {
            if row.len() >= 15 {
                outlets.push(MediaOutletData {
                    id_mo: row[0].clone(),
                    title: row[1].clone(),
                    sector: row[2].clone(),
                    mandate: row[3].clone(),
                    location: row[4].clone(),
                    distribution_area: row[5].clone(),
                    local: row[6].clone(),
                    language: row[7].clone(),
                    start_date: row[8].clone(),
                    start_fake_date: row[9].clone(),
                    end_date: row[10].clone(),
                    end_fake_date: row[11].clone(),
                    editorial_line_start: row[12].clone(),
                    editorial_line_end: row[13].clone(),
                    comments: row[14].clone(),
                });
            }
        }

        Ok(outlets)
    }

    /// Extract market data from mo_year table (include all records to show data patterns)
    async fn extract_market_data(
        &self,
        connection: &connection::DatabaseConnection,
    ) -> Result<Vec<MarketData>> {
        let query = "SELECT id_mo, year, mo_year, calc, circulation, circulation_source, unique_users, unique_users_source, reach_nat, reach_nat_source, reach_reg, reach_reg_source, market_share, market_share_source, comments FROM graphv3.mo_year ORDER BY year, calc, comments";

        let rows = connection.execute_query(query).await?;
        let mut market_data = Vec::new();

        for row in rows {
            if row.len() >= 15 {
                market_data.push(MarketData {
                    id_mo: row[0].clone(),
                    year: row[1].clone(),
                    mo_year: row[2].clone(),
                    calc: row[3].clone(),
                    circulation: row[4].clone(),
                    circulation_source: row[5].clone(),
                    unique_users: row[6].clone(),
                    unique_users_source: row[7].clone(),
                    reach_nat: row[8].clone(),
                    reach_nat_source: row[9].clone(),
                    reach_reg: row[10].clone(),
                    reach_reg_source: row[11].clone(),
                    market_share: row[12].clone(),
                    market_share_source: row[13].clone(),
                    comments: row[14].clone(),
                });
            }
        }

        Ok(market_data)
    }

    /// Extract source reference data
    async fn extract_sources(
        &self,
        connection: &connection::DatabaseConnection,
    ) -> Result<Vec<SourceData>> {
        let query = "SELECT id_source, source_name FROM graphv3.sources_names WHERE source_name IS NOT NULL ORDER BY source_name";

        let rows = connection.execute_query(query).await?;
        let mut sources = Vec::new();

        for row in rows {
            if row.len() >= 2 {
                sources.push(SourceData {
                    id_source: row[0].clone(),
                    source_name: row[1].clone(),
                });
            }
        }

        Ok(sources)
    }

    /// Extract relationship data from all relationship tables
    async fn extract_relationships(
        &self,
        connection: &connection::DatabaseConnection,
    ) -> Result<Vec<RelationshipData>> {
        let mut relationships = Vec::new();

        // Contemporary relationships (with temporal bounds)
        let contemporary_tables = vec![
            ("31_main_media_outlet", "main_media_outlet"),
            ("33_umbrella", "umbrella"),
            ("34_collaboration", "collaboration"),
        ];

        for (table_name, relation_type) in contemporary_tables {
            let query = format!(
                r#"WITH orf_outlets AS (
                    SELECT id_mo
                    FROM graphv3.mo_constant
                    WHERE mo_title ILIKE '%orf%'
                ),
                connected_outlets AS (
                    -- Include direct ORF outlets
                    SELECT id_mo as outlet_id FROM orf_outlets
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, main_media_outlet]) as outlet_id
                    FROM graphv3."31_main_media_outlet" 
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR main_media_outlet IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, umbrella]) as outlet_id
                    FROM graphv3."33_umbrella"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR umbrella IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, collaboration]) as outlet_id
                    FROM graphv3."34_collaboration"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR collaboration IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, succession]) as outlet_id
                    FROM graphv3."11_succession"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR succession IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, amalgamation]) as outlet_id
                    FROM graphv3."12_amalgamation"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR amalgamation IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, new_distribution_area]) as outlet_id
                    FROM graphv3."13_new_distribution_area"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR new_distribution_area IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, new_sector]) as outlet_id
                    FROM graphv3."14_new_sector"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR new_sector IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, interruption]) as outlet_id
                    FROM graphv3."19_interruption"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR interruption IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, split_off]) as outlet_id
                    FROM graphv3."21_split_off"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR split_off IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, offshoot]) as outlet_id
                    FROM graphv3."22_offshoot"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR offshoot IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, merger]) as outlet_id
                    FROM graphv3."23_merger"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR merger IN (SELECT id_mo FROM orf_outlets)
                )
                SELECT r.id_mo, r.{}, r.start_rel, r.end_rel 
                FROM graphv3."{}" r
                WHERE r.{} IS NOT NULL
                AND r.id_mo IN (SELECT outlet_id FROM connected_outlets)
                AND r.{} IN (SELECT outlet_id FROM connected_outlets)"#,
                relation_type, table_name, relation_type, relation_type
            );
            let rows = connection.execute_query(&query).await?;

            for row in rows {
                if row.len() >= 4 {
                    relationships.push(RelationshipData {
                        source_id: row[0].clone(),
                        target_id: row[1].clone(),
                        relationship_type: relation_type.to_string(),
                        start_date: row[2].clone(),
                        end_date: row[3].clone(),
                        is_temporal: true,
                    });
                }
            }
        }

        // Historical relationships (without temporal bounds)
        let historical_tables = vec![
            ("11_succession", "succession"),
            ("12_amalgamation", "amalgamation"),
            ("13_new_distribution_area", "new_distribution_area"),
            ("14_new_sector", "new_sector"),
            ("19_interruption", "interruption"),
            ("21_split_off", "split_off"),
            ("22_offshoot", "offshoot"),
            ("23_merger", "merger"),
        ];

        for (table_name, relation_type) in historical_tables {
            let query = format!(
                r#"WITH orf_outlets AS (
                    SELECT id_mo
                    FROM graphv3.mo_constant
                    WHERE mo_title ILIKE '%orf%'
                ),
                connected_outlets AS (
                    -- Include direct ORF outlets
                    SELECT id_mo as outlet_id FROM orf_outlets
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, main_media_outlet]) as outlet_id
                    FROM graphv3."31_main_media_outlet" 
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR main_media_outlet IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, umbrella]) as outlet_id
                    FROM graphv3."33_umbrella"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR umbrella IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, collaboration]) as outlet_id
                    FROM graphv3."34_collaboration"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR collaboration IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, succession]) as outlet_id
                    FROM graphv3."11_succession"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR succession IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, amalgamation]) as outlet_id
                    FROM graphv3."12_amalgamation"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR amalgamation IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, new_distribution_area]) as outlet_id
                    FROM graphv3."13_new_distribution_area"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR new_distribution_area IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, new_sector]) as outlet_id
                    FROM graphv3."14_new_sector"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR new_sector IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, interruption]) as outlet_id
                    FROM graphv3."19_interruption"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR interruption IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, split_off]) as outlet_id
                    FROM graphv3."21_split_off"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR split_off IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, offshoot]) as outlet_id
                    FROM graphv3."22_offshoot"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR offshoot IN (SELECT id_mo FROM orf_outlets)
                    UNION
                    SELECT DISTINCT unnest(ARRAY[id_mo, merger]) as outlet_id
                    FROM graphv3."23_merger"
                    WHERE id_mo IN (SELECT id_mo FROM orf_outlets) OR merger IN (SELECT id_mo FROM orf_outlets)
                )
                SELECT r.id_mo, r.{} 
                FROM graphv3."{}" r
                WHERE r.{} IS NOT NULL
                AND r.id_mo IN (SELECT outlet_id FROM connected_outlets)
                AND r.{} IN (SELECT outlet_id FROM connected_outlets)"#,
                relation_type, table_name, relation_type, relation_type
            );
            let rows = connection.execute_query(&query).await?;

            for row in rows {
                if row.len() >= 2 {
                    relationships.push(RelationshipData {
                        source_id: row[0].clone(),
                        target_id: row[1].clone(),
                        relationship_type: relation_type.to_string(),
                        start_date: "NULL".to_string(),
                        end_date: "NULL".to_string(),
                        is_temporal: false,
                    });
                }
            }
        }

        Ok(relationships)
    }

    /// Generate MDSL file header and imports
    fn generate_mdsl_header(&self, output: &mut String) -> Result<()> {
        use std::fmt::Write;

        writeln!(output, "// Generated MDSL from ANMI SQL database")?;
        writeln!(
            output,
            "// Database: {}",
            self.config
                .connection_string
                .split('@')
                .last()
                .unwrap_or("unknown")
        )?;
        writeln!(
            output,
            "// Generated at: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )?;
        writeln!(output)?;

        // Standard ANMI imports
        writeln!(output, "IMPORT \"anmi_common_codes.mdsl\";")?;
        writeln!(output, "IMPORT \"anmi_media_sectors.mdsl\";")?;
        writeln!(output, "IMPORT \"anmi_mandate_types.mdsl\";")?;
        writeln!(output, "IMPORT \"anmi_source_references.mdsl\";")?;
        writeln!(output)?;

        Ok(())
    }

    /// Generate vocabulary from sources data
    fn generate_vocabulary_from_sources(
        &self,
        output: &mut String,
        sources: &[SourceData],
    ) -> Result<()> {
        use std::fmt::Write;

        writeln!(output, "VOCABULARY DataSources {{")?;
        writeln!(output, "    TYPES {{")?;

        for (i, source) in sources.iter().enumerate() {
            let comma = if i < sources.len() - 1 { "," } else { "" };
            // Use numeric keys starting from 1, like existing MDSL vocabularies
            // Clean the source name to remove Unicode characters for MDSL compatibility
            let cleaned_name = self.clean_identifier(&source.source_name);
            writeln!(
                output,
                "        {}: \"{}\"{}",
                i + 1,
                cleaned_name.replace('"', "\\\""),
                comma
            )?;
        }

        writeln!(output, "    }}")?;
        writeln!(output, "}}")?;
        writeln!(output)?;

        Ok(())
    }

    /// Generate media outlets MDSL with actual outlets and families
    fn generate_media_outlets_mdsl(
        &self,
        output: &mut String,
        outlets: &[MediaOutletData],
    ) -> Result<()> {
        use std::fmt::Write;

        writeln!(output, "// Media Outlet Schema")?;
        writeln!(output, "// ===================")?;
        writeln!(output)?;

        // Generate a single UNIT schema definition for media outlets
        writeln!(output, "UNIT MediaOutlet {{")?;
        writeln!(output, "    id_mo: ID PRIMARY KEY,")?;
        writeln!(output, "    mo_title: TEXT(200),")?;
        writeln!(output, "    id_sector: NUMBER,")?;
        writeln!(output, "    mandate: NUMBER,")?;
        writeln!(output, "    location: TEXT(100),")?;
        writeln!(output, "    primary_distr_area: NUMBER,")?;
        writeln!(output, "    local: NUMBER,")?;
        writeln!(output, "    language: TEXT(50),")?;
        writeln!(output, "    start_date: TEXT(20),")?;
        writeln!(output, "    start_fake_date: NUMBER,")?;
        writeln!(output, "    end_date: TEXT(20),")?;
        writeln!(output, "    end_fake_date: NUMBER,")?;
        writeln!(output, "    editorial_line_s: TEXT(50),")?;
        writeln!(output, "    editorial_line_e: TEXT(50),")?;
        writeln!(output, "    comments: TEXT(1000)")?;
        writeln!(output, "}}")?;
        writeln!(output)?;

        // Generate sample data comments showing actual data
        writeln!(output, "// Sample Media Outlets:")?;
        let sample_count = outlets.len().min(5);
        for outlet in outlets.iter().take(sample_count) {
            writeln!(
                output,
                "// - {}: {} ({} - {})",
                outlet.title, outlet.location, outlet.start_date, outlet.end_date
            )?;
        }
        writeln!(
            output,
            "// ... and {} more",
            outlets.len().saturating_sub(sample_count)
        )?;
        writeln!(output)?;

        // NEW: Generate actual FAMILY and OUTLET structures for ORF outlets
        writeln!(output, "// Generated Media Outlets")?;
        writeln!(output, "// =======================")?;
        writeln!(output)?;

        // Filter for ORF outlets
        let orf_outlets: Vec<_> = outlets
            .iter()
            .filter(|outlet| outlet.title.to_lowercase().contains("orf"))
            .collect();

        if !orf_outlets.is_empty() {
            writeln!(output, "FAMILY \"ORF Media Group\" {{")?;
            writeln!(
                output,
                "    @comment \"Österreichischer Rundfunk and related outlets\""
            )?;
            writeln!(output)?;

            for outlet in orf_outlets {
                self.generate_outlet_from_data(output, outlet)?;
                writeln!(output)?;
            }

            writeln!(output, "}}")?;
            writeln!(output)?;
        }

        // Generate other outlets as standalone (non-ORF)
        let other_outlets: Vec<_> = outlets
            .iter()
            .filter(|outlet| !outlet.title.to_lowercase().contains("orf"))
            .take(20) // Limit to first 20 non-ORF outlets for demo
            .collect();

        if !other_outlets.is_empty() {
            writeln!(output, "FAMILY \"Other Media Outlets\" {{")?;
            writeln!(
                output,
                "    @comment \"Sample of other media outlets from database\""
            )?;
            writeln!(output)?;

            for outlet in other_outlets {
                self.generate_outlet_from_data(output, outlet)?;
                writeln!(output)?;
            }

            writeln!(output, "}}")?;
            writeln!(output)?;
        }

        Ok(())
    }

    /// Generate an OUTLET definition from MediaOutletData
    fn generate_outlet_from_data(
        &self,
        output: &mut String,
        outlet: &MediaOutletData,
    ) -> Result<()> {
        use std::fmt::Write;

        // Create a safe outlet name for MDSL
        let _safe_name = outlet
            .title
            .replace(" ", "_")
            .replace("-", "_")
            .replace(".", "_");

        writeln!(output, "    OUTLET \"{}\" {{", outlet.title)?;

        writeln!(output, "        identity {{")?;
        // Parse ID - handle both string and numeric IDs and put inside identity block
        if let Ok(id) = outlet.id_mo.parse::<i64>() {
            writeln!(output, "            id = {};", id)?;
        }
        writeln!(
            output,
            "            title = \"{}\";",
            outlet.title.replace("\"", "\\\"")
        )?;
        writeln!(output, "        }};")?;

        writeln!(output, "        lifecycle {{")?;
        let status = if outlet.end_date == "9999-01-01" || outlet.end_date.is_empty() {
            "active"
        } else {
            "inactive"
        };

        let end_date = if outlet.end_date == "9999-01-01" || outlet.end_date.is_empty() {
            "CURRENT".to_string()
        } else {
            format!("\"{}\"", outlet.end_date)
        };

        writeln!(
            output,
            "            status \"{}\" FROM \"{}\" TO {} {{",
            status, outlet.start_date, end_date
        )?;
        writeln!(output, "                precision_start = \"known\";")?;
        writeln!(output, "                precision_end = \"known\";")?;
        writeln!(output, "            }};")?;
        writeln!(output, "        }};")?;

        writeln!(output, "        characteristics {{")?;
        writeln!(output, "            sector = \"{}\";", outlet.sector)?;
        writeln!(output, "            mandate = \"{}\";", outlet.mandate)?;
        writeln!(output, "            distribution = {{")?;
        writeln!(
            output,
            "                primary_area = \"{}\";",
            outlet.distribution_area
        )?;
        let local_bool = outlet.local == "1" || outlet.local.to_lowercase() == "true";
        writeln!(output, "                local = {};", local_bool)?;
        writeln!(output, "            }};")?;
        writeln!(output, "            language = \"{}\";", outlet.language)?;
        writeln!(output, "        }};")?;

        writeln!(output, "        metadata {{")?;
        writeln!(output, "            steward = \"imported\";")?;
        writeln!(output, "            verified = \"2024-01-01\";")?;
        if !outlet.comments.is_empty() && outlet.comments != "NULL" {
            writeln!(
                output,
                "            comment = \"{}\";",
                outlet.comments.replace("\"", "\\\"")
            )?;
        }
        writeln!(output, "        }};")?;

        writeln!(output, "    }};")?;

        Ok(())
    }

    /// Generate relationships as DIACHRONIC_LINK and SYNCHRONOUS_LINK
    fn generate_relationships_mdsl(
        &self,
        output: &mut String,
        relationships: &[RelationshipData],
    ) -> Result<()> {
        use std::fmt::Write;

        if relationships.is_empty() {
            return Ok(());
        }

        writeln!(output, "// Relationships")?;
        writeln!(output, "// =============")?;
        writeln!(output)?;

        // Group relationships by type
        let mut temporal_rels = Vec::new();
        let mut historical_rels = Vec::new();

        for rel in relationships {
            if rel.is_temporal {
                temporal_rels.push(rel);
            } else {
                historical_rels.push(rel);
            }
        }

        // Generate temporal relationships (SYNCHRONOUS_LINK)
        for rel in temporal_rels {
            if rel.source_id == "NULL" || rel.target_id == "NULL" {
                continue;
            }

            let link_name = format!(
                "link_{}_{}", 
                self.clean_identifier(&rel.source_id),
                self.clean_identifier(&rel.relationship_type)
            );

            writeln!(output, "SYNCHRONOUS_LINK {} {{", link_name)?;
            writeln!(output, "    outlet_1 = {{")?;
            writeln!(output, "        id = {};", rel.source_id)?;
            writeln!(output, "        role = \"source\";")?;
            writeln!(output, "    }};")?;
            writeln!(output, "    outlet_2 = {{")?;
            writeln!(output, "        id = {};", rel.target_id)?;
            writeln!(output, "        role = \"target\";")?;
            writeln!(output, "    }};")?;
            writeln!(
                output,
                "    relationship_type = \"{}\";",
                rel.relationship_type
            )?;

            if rel.start_date != "NULL" && rel.end_date != "NULL" {
                writeln!(output, "    period_start = \"{}\";", rel.start_date)?;
                writeln!(output, "    period_end = \"{}\";", rel.end_date)?;
            }

            writeln!(output, "}};")?;
            writeln!(output)?;
        }

        // Generate historical relationships (DIACHRONIC_LINK)
        for rel in historical_rels {
            if rel.source_id == "NULL" || rel.target_id == "NULL" {
                continue;
            }

            let link_name = format!(
                "evolution_{}_{}", 
                self.clean_identifier(&rel.source_id),
                self.clean_identifier(&rel.relationship_type)
            );

            writeln!(output, "DIACHRONIC_LINK {} {{", link_name)?;
            writeln!(output, "    predecessor = {};", rel.source_id)?;
            writeln!(output, "    successor = {};", rel.target_id)?;
            writeln!(
                output,
                "    relationship_type = \"{}\";",
                rel.relationship_type
            )?;
            if rel.start_date != "NULL" {
                writeln!(output, "    event_date = \"{}\";", rel.start_date)?;
            }
            writeln!(output, "}};")?;
            writeln!(output)?;
        }

        Ok(())
    }

    /// Generate DATA blocks from market data showing available time series patterns
    fn generate_data_blocks_mdsl(
        &self,
        output: &mut String,
        market_data: &[MarketData],
        outlet_data: &[MediaOutletData],
    ) -> Result<()> {
        use std::collections::HashMap;
        use std::fmt::Write;

        if market_data.is_empty() {
            return Ok(());
        }

        writeln!(output, "// Market Data & Time Series")?;
        writeln!(output, "// =========================")?;
        writeln!(
            output,
            "// Comprehensive time series data from mo_year table"
        )?;
        writeln!(
            output,
            "// Shows data availability patterns and calculation methods by year"
        )?;
        writeln!(output)?;

        // Group by year for time series analysis
        let mut data_by_year: HashMap<String, Vec<&MarketData>> = HashMap::new();
        let mut records_with_outlet_id = 0;
        let mut records_with_data = 0;

        for data in market_data {
            if data.year == "NULL" || data.year.is_empty() {
                continue;
            }

            data_by_year
                .entry(data.year.clone())
                .or_insert_with(Vec::new)
                .push(data);

            if data.id_mo != "NULL" && !data.id_mo.is_empty() {
                records_with_outlet_id += 1;
            }

            if data.calc != "NULL" || data.comments != "NULL" || data.circulation != "NULL" {
                records_with_data += 1;
            }
        }

        // Generate summary first
        writeln!(output, "// Time Series Data Summary")?;
        writeln!(output, "// Total records: {}", market_data.len())?;
        writeln!(
            output,
            "// Records with outlet ID: {}",
            records_with_outlet_id
        )?;
        writeln!(output, "// Records with data: {}", records_with_data)?;
        writeln!(output, "// Years covered: {}", data_by_year.len())?;
        writeln!(output)?;

        // Create set of valid outlet IDs that are actually generated
        let valid_outlet_ids: std::collections::HashSet<String> = outlet_data
            .iter()
            .map(|outlet| outlet.id_mo.clone())
            .collect();

        // Group data by outlet_id for proper DATA FOR blocks, filtering for valid outlets only
        let mut data_by_outlet: HashMap<String, Vec<&MarketData>> = HashMap::new();
        
        for data in market_data {
            if data.id_mo != "NULL" && !data.id_mo.is_empty() && valid_outlet_ids.contains(&data.id_mo) {
                data_by_outlet
                    .entry(data.id_mo.clone())
                    .or_insert_with(Vec::new)
                    .push(data);
            }
        }

        // Generate DATA blocks for each outlet with meaningful data
        for (outlet_id, outlet_data) in data_by_outlet {
            if outlet_data.is_empty() {
                continue;
            }

            writeln!(output, "DATA FOR {} {{", outlet_id)?;
            writeln!(output, "    // Time series data for outlet {}", outlet_id)?;
            writeln!(output, "    total_records: {}", outlet_data.len())?;
            
            // Simple years block with basic metrics
            writeln!(output, "    years {{")?;
            for data in outlet_data {
                if data.year != "NULL" && !data.year.is_empty() {
                    let clean_year = data.year.split('-').next().unwrap_or(&data.year);
                    writeln!(output, "        {} {{", clean_year)?;
                    
                    if data.circulation != "NULL" && data.circulation != "99" {
                        writeln!(output, "            circulation = {};", data.circulation)?;
                    }
                    if data.reach_nat != "NULL" && data.reach_nat != "99.0" {
                        writeln!(output, "            reach_national = {};", data.reach_nat)?;
                    }
                    if data.market_share != "NULL" && data.market_share != "99.0" {
                        writeln!(output, "            market_share = {};", data.market_share)?;
                    }
                    
                    writeln!(output, "        }};")?;
                }
            }
            writeln!(output, "    }};")?;
            
            writeln!(output, "}}")?;
            writeln!(output)?;
        }

        Ok(())
    }

    /// Clean identifier for MDSL usage
    fn clean_identifier(&self, input: &str) -> String {
        input
            .replace(' ', "_")
            .replace(',', "")
            .replace('[', "_")
            .replace(']', "_")
            .replace('(', "_")
            .replace(')', "_")
            .replace('-', "_")
            .replace('.', "_")
            .replace('"', "_")
            .replace(':', "_")
            .replace('/', "_")
            .replace('\\', "_")
            // Handle common German/Austrian Unicode characters
            .replace('ä', "ae")
            .replace('ö', "oe")
            .replace('ü', "ue")
            .replace('Ä', "Ae")
            .replace('Ö', "Oe")
            .replace('Ü', "Ue")
            .replace('ß', "ss")
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect::<String>()
            .trim_matches('_')
            .to_string()
    }
}

/// Extracted media outlet data from mo_constant table
#[derive(Debug, Clone)]
pub struct MediaOutletData {
    /// Media outlet ID
    pub id_mo: String,
    /// Media outlet title
    pub title: String,
    /// Sector ID
    pub sector: String,
    /// Mandate type
    pub mandate: String,
    /// Geographic location
    pub location: String,
    /// Primary distribution area
    pub distribution_area: String,
    /// Local media indicator
    pub local: String,
    /// Publication language
    pub language: String,
    /// Start date
    pub start_date: String,
    /// Start date precision flag
    pub start_fake_date: String,
    /// End date
    pub end_date: String,
    /// End date precision flag
    pub end_fake_date: String,
    /// Editorial line at start
    pub editorial_line_start: String,
    /// Editorial line at end
    pub editorial_line_end: String,
    /// Additional comments
    pub comments: String,
}

/// Extracted market data from mo_year table
#[derive(Debug, Clone)]
pub struct MarketData {
    /// Media outlet ID
    pub id_mo: String,
    /// Data year
    pub year: String,
    /// Media outlet year ID
    pub mo_year: String,
    /// Calculation flag
    pub calc: String,
    /// Circulation data
    pub circulation: String,
    /// Circulation data source
    pub circulation_source: String,
    /// Unique users data
    pub unique_users: String,
    /// Unique users data source
    pub unique_users_source: String,
    /// National reach data
    pub reach_nat: String,
    /// National reach data source
    pub reach_nat_source: String,
    /// Regional reach data
    pub reach_reg: String,
    /// Regional reach data source
    pub reach_reg_source: String,
    /// Market share data
    pub market_share: String,
    /// Market share data source
    pub market_share_source: String,
    /// Additional comments
    pub comments: String,
}

/// Extracted source reference data
#[derive(Debug, Clone)]
pub struct SourceData {
    /// Source ID
    pub id_source: String,
    /// Source name
    pub source_name: String,
}

/// Extracted relationship data from relationship tables
#[derive(Debug, Clone)]
pub struct RelationshipData {
    /// Source entity ID
    pub source_id: String,
    /// Target entity ID
    pub target_id: String,
    /// Type of relationship
    pub relationship_type: String,
    /// Relationship start date
    pub start_date: String,
    /// Relationship end date
    pub end_date: String,
    /// Whether relationship has temporal bounds
    pub is_temporal: bool,
}
