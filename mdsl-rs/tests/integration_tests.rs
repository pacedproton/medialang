//! Integration tests for MediaLanguage DSL parser
//!
//! These tests verify that the parser can successfully parse all sample DSL files
//! and produce the expected AST structures.

use mdsl_rs::lexer::Scanner;
use mdsl_rs::parser::ast::*;
use mdsl_rs::parser::recursive_descent::Parser;
use std::fs;

/// Helper function to parse a DSL file and return the AST
fn parse_file(file_path: &str) -> Result<Program, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut scanner = Scanner::new(&content);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    Ok(ast)
}

/// Helper function to get the path to a MediaLanguage file
fn media_file_path(filename: &str) -> String {
    format!("../MediaLanguage/{}", filename)
}

#[test]
fn test_parse_sources_catalog() {
    let ast = parse_file(&media_file_path("sources.mdsl")).expect("Failed to parse sources.mdsl");

    // Should have import and catalog statements
    assert_eq!(ast.statements.len(), 2);

    // First statement should be import
    if let Statement::Import(import) = &ast.statements[0] {
        assert_eq!(import.path, "anmi_source_references.mdsl");
    } else {
        panic!("Expected import statement");
    }

    // Second statement should be catalog
    if let Statement::Catalog(catalog) = &ast.statements[1] {
        assert_eq!(catalog.name, "sources");
        assert_eq!(catalog.sources.len(), 3); // oeak, media_analyse, owa

        // Check first source
        let first_source = &catalog.sources[0];
        assert_eq!(first_source.name, "oeak");
        assert!(first_source.fields.len() > 0);
    } else {
        panic!("Expected catalog statement");
    }
}

#[test]
fn test_parse_nested_vocabularies() {
    let ast = parse_file(&media_file_path("anmi_common_codes.mdsl"))
        .expect("Failed to parse anmi_common_codes.mdsl");

    // Should have 3 vocabulary statements
    assert_eq!(ast.statements.len(), 3);

    // First should be the main vocabulary with nested bodies
    if let Statement::Vocabulary(vocab) = &ast.statements[0] {
        assert_eq!(vocab.name, "anmi_common_codes");
        assert_eq!(vocab.bodies.len(), 2); // LANGUAGE_CODES and REGION_CODES

        // Check LANGUAGE_CODES
        let lang_codes = &vocab.bodies[0];
        assert_eq!(lang_codes.name, "LANGUAGE_CODES");
        assert_eq!(lang_codes.entries.len(), 4); // Deutsch, Englisch, Andere, KA

        // Check REGION_CODES
        let region_codes = &vocab.bodies[1];
        assert_eq!(region_codes.name, "REGION_CODES");
        assert_eq!(region_codes.entries.len(), 11); // 1-10 plus 99
    } else {
        panic!("Expected vocabulary statement");
    }

    // Second should be DatePrecisionTypes
    if let Statement::Vocabulary(vocab) = &ast.statements[1] {
        assert_eq!(vocab.name, "DatePrecisionTypes");
        assert_eq!(vocab.bodies.len(), 1);
    } else {
        panic!("Expected DatePrecisionTypes vocabulary");
    }
}

#[test]
fn test_parse_core_entity_units() {
    let ast = parse_file(&media_file_path("anmi_core_entity_units.mdsl"))
        .expect("Failed to parse anmi_core_entity_units.mdsl");

    // Should contain unit declarations
    let unit_count = ast
        .statements
        .iter()
        .filter(|s| matches!(s, Statement::Unit(_)))
        .count();
    assert!(unit_count > 0, "Should contain unit declarations");

    // Check for specific units - should find MedienangeboteDiachroneBeziehungen which has RELATIONSHIP_TYPE
    let mut found_relationship_type = false;
    for statement in &ast.statements {
        if let Statement::Unit(unit) = statement {
            if unit.name == "MedienangeboteDiachroneBeziehungen" {
                // Should have fields including RELATIONSHIP_TYPE
                let has_relationship_type =
                    unit.fields.iter().any(|f| f.name == "RELATIONSHIP_TYPE");
                if has_relationship_type {
                    found_relationship_type = true;
                }
            }
        }
    }
    assert!(
        found_relationship_type,
        "Should contain unit with RELATIONSHIP_TYPE field"
    );
}

#[test]
fn test_parse_express_freeze3() {
    let ast = parse_file(&media_file_path("express_freeze3.mdsl"))
        .expect("Failed to parse express_freeze3.mdsl");

    // Should have imports, variables, and family
    let import_count = ast
        .statements
        .iter()
        .filter(|s| matches!(s, Statement::Import(_)))
        .count();
    assert!(import_count > 0, "Should have import statements");

    let variable_count = ast
        .statements
        .iter()
        .filter(|s| matches!(s, Statement::Variable(_)))
        .count();
    assert!(variable_count > 0, "Should have variable declarations");

    let family_count = ast
        .statements
        .iter()
        .filter(|s| matches!(s, Statement::Family(_)))
        .count();
    assert_eq!(family_count, 2, "Should have two family declarations");

    // Check family structure - should find families with expected names
    let mut family_names = Vec::new();
    let mut has_outlets = false;
    let mut has_data = false;

    for statement in &ast.statements {
        if let Statement::Family(family) = statement {
            family_names.push(&family.name);

            // Check if this family has outlets
            let outlet_count = family
                .members
                .iter()
                .filter(|m| matches!(m, FamilyMember::Outlet(_)))
                .count();
            if outlet_count > 0 {
                has_outlets = true;
            }

            // Check if this family has data members
            let data_count = family
                .members
                .iter()
                .filter(|m| matches!(m, FamilyMember::Data(_)))
                .count();
            if data_count > 0 {
                has_data = true;
            }
        }
    }

    // At least one family should have outlets and data
    assert!(has_outlets, "Should have at least one family with outlets");
    assert!(
        has_data,
        "Should have at least one family with data members"
    );

    // Check that we have the expected family names
    assert!(
        family_names
            .iter()
            .any(|name| name.as_str() == "Express FAMILY"),
        "Should contain 'Express FAMILY'"
    );
    assert!(
        family_names
            .iter()
            .any(|name| name.as_str() == "Express explorative digital extension"),
        "Should contain 'Express explorative digital extension'"
    );
}

#[test]
fn test_parse_all_successful_files() {
    let successful_files = vec![
        "anmi_common_codes.mdsl",
        "anmi_core_entity_units.mdsl",
        "anmi_main.mdsl",
        "anmi_mandate_types.mdsl",
        "anmi_media_sectors.mdsl",
        "express_freeze3.mdsl",
        "Medienangebot.mdsl",
        "MedienangeboteDiachroneBeziehungen.mdsl",
        "MedienangeboteSynchroneBeziehungen.mdsl",
        "Medienunternehmen.mdsl",
        "MedienunternehmenDiachroneBeziehungen.mdsl.mdsl",
        "MedienunternehmenSynchroneBeziehungenMitAnderenUnternehmen.mdsl",
        "MedienunternehmenSynchroneBeziehungenMitMedienangeboten.mdsl",
        "sources.mdsl",
    ];

    for file in successful_files {
        let result = parse_file(&media_file_path(file));
        assert!(
            result.is_ok(),
            "Failed to parse {}: {:?}",
            file,
            result.err()
        );

        let ast = result.unwrap();
        assert!(
            !ast.statements.is_empty(),
            "File {} produced empty AST",
            file
        );
    }
}

#[test]
fn test_vocabulary_entries_types() {
    let ast = parse_file(&media_file_path("anmi_common_codes.mdsl"))
        .expect("Failed to parse anmi_common_codes.mdsl");

    if let Statement::Vocabulary(vocab) = &ast.statements[0] {
        // LANGUAGE_CODES should have string keys
        let lang_codes = &vocab.bodies[0];
        for entry in &lang_codes.entries {
            assert!(
                matches!(entry.key, VocabularyKey::String(_)),
                "Language codes should have string keys"
            );
        }

        // REGION_CODES should have numeric keys
        let region_codes = &vocab.bodies[1];
        for entry in &region_codes.entries {
            assert!(
                matches!(entry.key, VocabularyKey::Number(_)),
                "Region codes should have numeric keys"
            );
        }
    }
}

#[test]
fn test_unit_field_types() {
    let ast = parse_file(&media_file_path("anmi_core_entity_units.mdsl"))
        .expect("Failed to parse anmi_core_entity_units.mdsl");

    let mut found_category_field = false;
    let mut found_id_field = false;
    let mut found_text_field = false;

    for statement in &ast.statements {
        if let Statement::Unit(unit) = statement {
            for field in &unit.fields {
                match &field.field_type {
                    FieldType::Id => found_id_field = true,
                    FieldType::Text(_) => found_text_field = true,
                    FieldType::Category(values) => {
                        found_category_field = true;
                        assert!(!values.is_empty(), "Category should have values");
                    }
                    _ => {}
                }
            }
        }
    }

    assert!(found_id_field, "Should find ID field type");
    assert!(found_text_field, "Should find TEXT field type");
    assert!(found_category_field, "Should find CATEGORY field type");
}

#[test]
fn test_family_outlet_structure() {
    let ast = parse_file(&media_file_path("express_freeze3.mdsl"))
        .expect("Failed to parse express_freeze3.mdsl");

    for statement in &ast.statements {
        if let Statement::Family(family) = statement {
            for member in &family.members {
                if let FamilyMember::Outlet(outlet) = member {
                    // Outlet should have blocks
                    assert!(!outlet.blocks.is_empty(), "Outlet should have blocks");

                    // Check for identity, lifecycle, characteristics, metadata blocks
                    let has_identity = outlet
                        .blocks
                        .iter()
                        .any(|b| matches!(b, OutletBlock::Identity(_)));
                    let has_lifecycle = outlet
                        .blocks
                        .iter()
                        .any(|b| matches!(b, OutletBlock::Lifecycle(_)));
                    let has_characteristics = outlet
                        .blocks
                        .iter()
                        .any(|b| matches!(b, OutletBlock::Characteristics(_)));
                    let has_metadata = outlet
                        .blocks
                        .iter()
                        .any(|b| matches!(b, OutletBlock::Metadata(_)));

                    assert!(
                        has_identity || has_lifecycle || has_characteristics || has_metadata,
                        "Outlet should have at least one block type"
                    );
                }
            }
        }
    }
}

#[test]
fn test_catalog_source_fields() {
    let ast = parse_file(&media_file_path("sources.mdsl")).expect("Failed to parse sources.mdsl");

    if let Statement::Catalog(catalog) = &ast.statements[1] {
        for source in &catalog.sources {
            // Each source should have fields
            assert!(!source.fields.is_empty(), "Source should have fields");

            // Check for nested assignments (anmi_source_id_components)
            let has_nested = source
                .fields
                .iter()
                .any(|f| matches!(f, SourceField::NestedAssignment { .. }));
            assert!(has_nested, "Source should have nested assignments");

            // Check for simple assignments
            let has_simple = source
                .fields
                .iter()
                .any(|f| matches!(f, SourceField::Assignment { .. }));
            assert!(has_simple, "Source should have simple assignments");

            // Check for annotations
            let has_annotation = source
                .fields
                .iter()
                .any(|f| matches!(f, SourceField::Annotation(_)));
            assert!(has_annotation, "Source should have annotations");
        }
    }
}

#[test]
fn test_period_parsing() {
    let ast = parse_file(&media_file_path("express_freeze3.mdsl"))
        .expect("Failed to parse express_freeze3.mdsl");

    // Look for PERIOD fields in object literals
    fn check_for_period(statements: &[Statement]) -> bool {
        for statement in statements {
            if let Statement::Family(family) = statement {
                for member in &family.members {
                    if let FamilyMember::Outlet(outlet) = member {
                        for block in &outlet.blocks {
                            if let OutletBlock::Identity(identity) = block {
                                for field in &identity.fields {
                                    if let IdentityField::ArrayAssignment { values, .. } = field {
                                        for obj in values {
                                            for obj_field in &obj.fields {
                                                if let ObjectField::Period { .. } = obj_field {
                                                    return true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    let found_period = check_for_period(&ast.statements);
    assert!(found_period, "Should find PERIOD field parsing");
}
