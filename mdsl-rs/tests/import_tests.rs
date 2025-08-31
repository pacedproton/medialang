//! Simplified SQL import functionality tests
//! Tests for database import functionality (feature-gated)

#[cfg(feature = "import")]
mod import_tests {
    use mdsl_rs::import::generator::MdslGenerator;
    use mdsl_rs::import::mapper::{DataMapper, MdslEntity};
    use mdsl_rs::import::{DatabaseConfig, DatabaseType};
    use std::collections::HashMap;

    #[test]
    fn test_database_config_creation() {
        let config = DatabaseConfig {
            db_type: DatabaseType::PostgreSQL,
            connection_string: "postgresql://user:pass@localhost:5432/testdb".to_string(),
            schema: None,
        };

        assert_eq!(
            config.connection_string,
            "postgresql://user:pass@localhost:5432/testdb"
        );
        assert_eq!(config.db_type, DatabaseType::PostgreSQL);
        assert!(config.schema.is_none());
    }

    #[test]
    fn test_database_config_with_schema() {
        let config = DatabaseConfig {
            db_type: DatabaseType::PostgreSQL,
            connection_string: "postgresql://localhost/test".to_string(),
            schema: Some("public".to_string()),
        };

        assert_eq!(config.connection_string, "postgresql://localhost/test");
        assert_eq!(config.schema, Some("public".to_string()));
        assert_eq!(config.db_type, DatabaseType::PostgreSQL);
    }

    #[test]
    fn test_database_config_mysql() {
        let config = DatabaseConfig {
            db_type: DatabaseType::MySQL,
            connection_string: "mysql://localhost/test".to_string(),
            schema: None,
        };

        assert_eq!(config.db_type, DatabaseType::MySQL);
    }

    #[test]
    fn test_database_config_sqlite() {
        let config = DatabaseConfig {
            db_type: DatabaseType::SQLite,
            connection_string: "sqlite:///tmp/test.db".to_string(),
            schema: None,
        };

        assert_eq!(config.db_type, DatabaseType::SQLite);
    }

    #[test]
    fn test_mdsl_generator_creation() {
        let generator = MdslGenerator::new();

        // Test with empty entities
        let empty_entities: Vec<MdslEntity> = Vec::new();
        let result = generator.generate(&empty_entities);
        assert!(result.is_ok());

        let mdsl = result.unwrap();
        assert!(!mdsl.is_empty()); // Should have at least header comments
    }

    #[test]
    fn test_mdsl_generator_with_entities() {
        let generator = MdslGenerator::new();

        // Create test entities
        use mdsl_rs::import::mapper::{MdslUnit, MdslVocabulary};

        let entities = vec![
            MdslEntity::Unit(MdslUnit {
                name: "TestUnit".to_string(),
                fields: vec![],
            }),
            MdslEntity::Vocabulary(MdslVocabulary {
                name: "TestVocab".to_string(),
                entries: vec![],
            }),
        ];

        let result = generator.generate(&entities);
        assert!(result.is_ok());

        let mdsl = result.unwrap();
        assert!(mdsl.contains("TestUnit"));
        assert!(mdsl.contains("TestVocab"));
    }

    #[test]
    fn test_data_mapper_creation() {
        use mdsl_rs::import::{MdslEntityType, TableMapping};

        let mut table_mappings = HashMap::new();
        let table_mapping = TableMapping {
            entity_type: MdslEntityType::Outlet,
            field_mappings: HashMap::new(),
            relationship_mapping: None,
            temporal_fields: None,
            is_anmi_schema: false,
        };
        table_mappings.insert("outlets".to_string(), table_mapping);

        let _mapper = DataMapper::new(&table_mappings);

        // Test that the mapper was created successfully (can't access private fields)
        // This is just testing that construction works
        assert_eq!(table_mappings.len(), 1);
        assert!(table_mappings.contains_key("outlets"));
    }

    #[test]
    fn test_multiple_database_types() {
        let configs = vec![
            DatabaseConfig {
                db_type: DatabaseType::PostgreSQL,
                connection_string: "postgresql://localhost/test".to_string(),
                schema: Some("public".to_string()),
            },
            DatabaseConfig {
                db_type: DatabaseType::MySQL,
                connection_string: "mysql://localhost/test".to_string(),
                schema: None,
            },
            DatabaseConfig {
                db_type: DatabaseType::SQLite,
                connection_string: "sqlite:///tmp/test.db".to_string(),
                schema: None,
            },
            DatabaseConfig {
                db_type: DatabaseType::SqlServer,
                connection_string: "sqlserver://localhost/test".to_string(),
                schema: Some("dbo".to_string()),
            },
        ];

        assert_eq!(configs.len(), 4);
        assert_eq!(configs[0].db_type, DatabaseType::PostgreSQL);
        assert_eq!(configs[1].db_type, DatabaseType::MySQL);
        assert_eq!(configs[2].db_type, DatabaseType::SQLite);
        assert_eq!(configs[3].db_type, DatabaseType::SqlServer);
    }

    #[test]
    fn test_import_workflow_basic() {
        // Test the basic workflow components work together
        let config = DatabaseConfig {
            db_type: DatabaseType::PostgreSQL,
            connection_string: "postgresql://localhost/test".to_string(),
            schema: Some("public".to_string()),
        };

        let table_mappings = HashMap::new();
        let _mapper = DataMapper::new(&table_mappings);
        let generator = MdslGenerator::new();

        // Test basic generation
        let entities: Vec<MdslEntity> = Vec::new();
        let result = generator.generate(&entities);

        assert!(result.is_ok());
        assert_eq!(config.db_type, DatabaseType::PostgreSQL);
    }

    #[test]
    fn test_field_mapping_creation() {
        use mdsl_rs::import::FieldMapping;

        let field_mapping = FieldMapping {
            sql_column: "name".to_string(),
            mdsl_field: "title".to_string(),
            transform: None,
            required: true,
        };

        assert_eq!(field_mapping.mdsl_field, "title");
        assert_eq!(field_mapping.sql_column, "name");
        assert!(field_mapping.required);
    }

    #[test]
    fn test_table_mapping_creation() {
        use mdsl_rs::import::{FieldMapping, MdslEntityType, TableMapping};

        let mut field_mappings = HashMap::new();
        field_mappings.insert(
            "id".to_string(),
            FieldMapping {
                sql_column: "id".to_string(),
                mdsl_field: "id".to_string(),
                transform: None,
                required: true,
            },
        );

        let table_mapping = TableMapping {
            entity_type: MdslEntityType::Outlet,
            field_mappings,
            relationship_mapping: None,
            temporal_fields: None,
            is_anmi_schema: true,
        };

        assert_eq!(table_mapping.entity_type, MdslEntityType::Outlet);
        assert!(table_mapping.is_anmi_schema);
        assert_eq!(table_mapping.field_mappings.len(), 1);
    }

    #[test]
    fn test_entity_types() {
        use mdsl_rs::import::MdslEntityType;

        let types = vec![
            MdslEntityType::Outlet,
            MdslEntityType::Family,
            MdslEntityType::Vocabulary,
            MdslEntityType::Unit,
        ];

        assert_eq!(types.len(), 4);
        assert_eq!(types[0], MdslEntityType::Outlet);
        assert_eq!(types[1], MdslEntityType::Family);
        assert_eq!(types[2], MdslEntityType::Vocabulary);
        assert_eq!(types[3], MdslEntityType::Unit);
    }

    #[test]
    fn test_generator_error_handling() {
        let generator = MdslGenerator::new();

        // Test with malformed entity (should handle gracefully)
        use mdsl_rs::import::mapper::MdslUnit;

        let entities = vec![MdslEntity::Unit(MdslUnit {
            name: "".to_string(), // Empty name
            fields: vec![],
        })];

        let result = generator.generate(&entities);
        // Should still work, just generate empty or handle gracefully
        assert!(result.is_ok());
    }

    #[test]
    fn test_complex_table_mapping() {
        use mdsl_rs::import::{FieldMapping, MdslEntityType, TableMapping};

        let mut field_mappings = HashMap::new();

        // Add multiple field mappings
        field_mappings.insert(
            "id".to_string(),
            FieldMapping {
                sql_column: "outlet_id".to_string(),
                mdsl_field: "id".to_string(),
                transform: None,
                required: true,
            },
        );

        field_mappings.insert(
            "title".to_string(),
            FieldMapping {
                sql_column: "outlet_name".to_string(),
                mdsl_field: "title".to_string(),
                transform: None,
                required: false,
            },
        );

        let table_mapping = TableMapping {
            entity_type: MdslEntityType::Outlet,
            field_mappings,
            relationship_mapping: None,
            temporal_fields: None,
            is_anmi_schema: true,
        };

        assert_eq!(table_mapping.field_mappings.len(), 2);
        assert!(table_mapping.field_mappings.contains_key("id"));
        assert!(table_mapping.field_mappings.contains_key("title"));

        let title_mapping = &table_mapping.field_mappings["title"];
        assert!(!title_mapping.required);
        let id_mapping = &table_mapping.field_mappings["id"];
        assert!(id_mapping.required);
    }

    #[test]
    fn test_time_series_data_integration() {
        use mdsl_rs::import::{DatabaseConfig, DatabaseType};
        use mdsl_rs::import::{MarketData, MediaOutletData, SqlImporter};

        // Create a SqlImporter instance
        let config = DatabaseConfig {
            db_type: DatabaseType::PostgreSQL,
            connection_string: "postgresql://test".to_string(),
            schema: None,
        };
        let importer = SqlImporter::new(config);

        // Create sample market data
        let market_data = vec![
            MarketData {
                id_mo: "200001".to_string(),
                year: "2020".to_string(),
                mo_year: "2020".to_string(),
                calc: "1".to_string(),
                circulation: "10000".to_string(),
                circulation_source: "official".to_string(),
                unique_users: "5000".to_string(),
                unique_users_source: "survey".to_string(),
                reach_nat: "25.5".to_string(),
                reach_nat_source: "media_analyzer".to_string(),
                reach_reg: "45.2".to_string(),
                reach_reg_source: "regional".to_string(),
                market_share: "15.3".to_string(),
                market_share_source: "industry".to_string(),
                comments: "Annual report data".to_string(),
            },
            MarketData {
                id_mo: "200001".to_string(),
                year: "2021".to_string(),
                mo_year: "2021".to_string(),
                calc: "1".to_string(),
                circulation: "11000".to_string(),
                circulation_source: "official".to_string(),
                unique_users: "5500".to_string(),
                unique_users_source: "survey".to_string(),
                reach_nat: "26.8".to_string(),
                reach_nat_source: "media_analyzer".to_string(),
                reach_reg: "46.1".to_string(),
                reach_reg_source: "regional".to_string(),
                market_share: "16.2".to_string(),
                market_share_source: "industry".to_string(),
                comments: "Annual report data".to_string(),
            },
            MarketData {
                id_mo: "200002".to_string(),
                year: "2020".to_string(),
                mo_year: "2020".to_string(),
                calc: "1".to_string(),
                circulation: "5000".to_string(),
                circulation_source: "official".to_string(),
                unique_users: "2500".to_string(),
                unique_users_source: "survey".to_string(),
                reach_nat: "12.3".to_string(),
                reach_nat_source: "media_analyzer".to_string(),
                reach_reg: "22.1".to_string(),
                reach_reg_source: "regional".to_string(),
                market_share: "7.8".to_string(),
                market_share_source: "industry".to_string(),
                comments: "Regional publication".to_string(),
            },
        ];

        // Create sample outlet data
        let outlet_data = vec![
            MediaOutletData {
                id_mo: "200001".to_string(),
                title: "ORF Radio Wien".to_string(),
                sector: "20".to_string(),
                mandate: "1".to_string(),
                location: "Wien".to_string(),
                distribution_area: "1".to_string(),
                local: "0".to_string(),
                language: "deutsch".to_string(),
                start_date: "1967-01-01".to_string(),
                start_fake_date: "".to_string(),
                end_date: "9999-01-01".to_string(),
                end_fake_date: "".to_string(),
                editorial_line_start: "Öffentlich-rechtlich".to_string(),
                editorial_line_end: "".to_string(),
                comments: "Major Austrian public broadcaster".to_string(),
            },
            MediaOutletData {
                id_mo: "200002".to_string(),
                title: "Kronen Zeitung".to_string(),
                sector: "11".to_string(),
                mandate: "2".to_string(),
                location: "Wien".to_string(),
                distribution_area: "1".to_string(),
                local: "0".to_string(),
                language: "deutsch".to_string(),
                start_date: "1959-01-01".to_string(),
                start_fake_date: "".to_string(),
                end_date: "9999-01-01".to_string(),
                end_fake_date: "".to_string(),
                editorial_line_start: "Privat".to_string(),
                editorial_line_end: "".to_string(),
                comments: "Leading Austrian newspaper".to_string(),
            },
        ];

        // Test the time series data generation
        let mut output = String::new();
        let result = importer.generate_time_series_data(&mut output, &market_data, &outlet_data);

        // Assert the method completed successfully
        assert!(result.is_ok());

        // Verify the output contains expected content
        assert!(output.contains("Time Series Data Integration"));
        assert!(output.contains("Total time series records: 3"));
        assert!(output.contains("ORF Radio Wien"));
        assert!(output.contains("Kronen Zeitung"));
        assert!(output.contains("circ=10000"));
        assert!(output.contains("reach=25.5"));
        assert!(output.contains("share=15.3"));
        assert!(output.contains("Annual report data"));
        assert!(output.contains("Regional publication"));
        assert!(output.contains("Time Series Summary"));
        assert!(output.contains("Total outlets with time series data: 2"));
    }
}
