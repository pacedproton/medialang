//! Simplified SQL import functionality tests
//! Tests for database import functionality (feature-gated)

#[cfg(feature = "import")]
mod import_tests {
    use mdsl_rs::import::{DatabaseConfig, DatabaseType};
    use mdsl_rs::import::generator::MdslGenerator;
    use mdsl_rs::import::mapper::{DataMapper, MdslEntity};
    use std::collections::HashMap;

    #[test]
    fn test_database_config_creation() {
        let config = DatabaseConfig {
            db_type: DatabaseType::PostgreSQL,
            connection_string: "postgresql://user:pass@localhost:5432/testdb".to_string(),
            schema: None,
        };

        assert_eq!(config.connection_string, "postgresql://user:pass@localhost:5432/testdb");
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
        use mdsl_rs::import::{TableMapping, MdslEntityType};
        
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
        use mdsl_rs::import::{TableMapping, MdslEntityType, FieldMapping};
        
        let mut field_mappings = HashMap::new();
        field_mappings.insert("id".to_string(), FieldMapping {
            sql_column: "id".to_string(),
            mdsl_field: "id".to_string(),
            transform: None,
            required: true,
        });

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
        
        let entities = vec![
            MdslEntity::Unit(MdslUnit {
                name: "".to_string(), // Empty name
                fields: vec![],
            }),
        ];
        
        let result = generator.generate(&entities);
        // Should still work, just generate empty or handle gracefully
        assert!(result.is_ok());
    }

    #[test]
    fn test_complex_table_mapping() {
        use mdsl_rs::import::{TableMapping, MdslEntityType, FieldMapping};
        
        let mut field_mappings = HashMap::new();
        
        // Add multiple field mappings
        field_mappings.insert("id".to_string(), FieldMapping {
            sql_column: "outlet_id".to_string(),
            mdsl_field: "id".to_string(),
            transform: None,
            required: true,
        });
        
        field_mappings.insert("title".to_string(), FieldMapping {
            sql_column: "outlet_name".to_string(),
            mdsl_field: "title".to_string(),
            transform: None,
            required: false,
        });

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
}