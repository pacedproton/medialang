//! Unit tests for individual parser components
//!
//! These tests verify the functionality of individual lexer and parser components
//! in isolation.

use mdsl_rs::lexer::{Keyword, Scanner, TokenKind};
use mdsl_rs::parser::ast::*;
use mdsl_rs::parser::recursive_descent::Parser;

/// Helper function to tokenize input and return tokens
fn tokenize(input: &str) -> Vec<mdsl_rs::lexer::Token> {
    let mut scanner = Scanner::new(input);
    scanner.scan_tokens().expect("Tokenization failed")
}

/// Helper function to parse tokens and return AST
fn parse_tokens(tokens: Vec<mdsl_rs::lexer::Token>) -> Program {
    let mut parser = Parser::new(tokens);
    parser.parse().expect("Parsing failed")
}

/// Helper function to parse input directly
fn parse_input(input: &str) -> Program {
    let tokens = tokenize(input);
    parse_tokens(tokens)
}

#[test]
fn test_lexer_keywords() {
    let input = "IMPORT UNIT VOCABULARY FAMILY OUTLET catalog source";
    let tokens = tokenize(input);

    let keywords = tokens
        .iter()
        .filter_map(|t| match &t.kind {
            TokenKind::Keyword(kw) => Some(kw),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(keywords.len(), 7);
    assert!(keywords.contains(&&Keyword::Import));
    assert!(keywords.contains(&&Keyword::Unit));
    assert!(keywords.contains(&&Keyword::Vocabulary));
    assert!(keywords.contains(&&Keyword::Family));
    assert!(keywords.contains(&&Keyword::Outlet));
    assert!(keywords.contains(&&Keyword::Catalog));
    assert!(keywords.contains(&&Keyword::Source));
}

#[test]
fn test_lexer_strings_and_numbers() {
    let input = r#""hello world" 42 3.14 "test""#;
    let tokens = tokenize(input);

    let strings = tokens
        .iter()
        .filter_map(|t| match &t.kind {
            TokenKind::String(s) => Some(s),
            _ => None,
        })
        .collect::<Vec<_>>();

    let numbers = tokens
        .iter()
        .filter_map(|t| match &t.kind {
            TokenKind::Number(n) => Some(n),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(strings.len(), 2);
    assert_eq!(strings[0], "hello world");
    assert_eq!(strings[1], "test");

    assert_eq!(numbers.len(), 2);
    assert_eq!(numbers[0], &42.0);
    assert_eq!(numbers[1], &3.14);
}

#[test]
fn test_lexer_identifiers() {
    let input = "test_var another_id RELATIONSHIP_TYPE";
    let tokens = tokenize(input);

    let identifiers = tokens
        .iter()
        .filter_map(|t| match &t.kind {
            TokenKind::Identifier(id) => Some(id),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(identifiers.len(), 2); // RELATIONSHIP_TYPE becomes a keyword
    assert_eq!(identifiers[0], "test_var");
    assert_eq!(identifiers[1], "another_id");
}

#[test]
fn test_parse_simple_import() {
    let input = r#"IMPORT "test.mdsl";"#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Import(import) = &ast.statements[0] {
        assert_eq!(import.path, "test.mdsl");
    } else {
        panic!("Expected import statement");
    }
}

#[test]
fn test_parse_variable_declaration() {
    let input = r#"LET test_var = "test value";"#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Variable(var) = &ast.statements[0] {
        assert_eq!(var.name, "test_var");
        if let Expression::String(value) = &var.value {
            assert_eq!(value, "test value");
        } else {
            panic!("Expected string expression");
        }
    } else {
        panic!("Expected variable declaration");
    }
}

#[test]
fn test_parse_simple_unit() {
    let input = r#"
    UNIT TestUnit {
        id: ID PRIMARY KEY,
        name: TEXT(100),
        count: NUMBER,
        active: BOOLEAN,
        category: CATEGORY("A", "B", "C")
    }
    "#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Unit(unit) = &ast.statements[0] {
        assert_eq!(unit.name, "TestUnit");
        assert_eq!(unit.fields.len(), 5);

        // Check ID field
        let id_field = &unit.fields[0];
        assert_eq!(id_field.name, "id");
        assert!(matches!(id_field.field_type, FieldType::Id));
        assert!(id_field.is_primary_key);

        // Check TEXT field
        let name_field = &unit.fields[1];
        assert_eq!(name_field.name, "name");
        if let FieldType::Text(Some(length)) = name_field.field_type {
            assert_eq!(length, 100);
        } else {
            panic!("Expected TEXT(100) field type");
        }

        // Check CATEGORY field
        let category_field = &unit.fields[4];
        assert_eq!(category_field.name, "category");
        if let FieldType::Category(values) = &category_field.field_type {
            assert_eq!(values.len(), 3);
            assert_eq!(values[0], "A");
            assert_eq!(values[1], "B");
            assert_eq!(values[2], "C");
        } else {
            panic!("Expected CATEGORY field type");
        }
    } else {
        panic!("Expected unit declaration");
    }
}

#[test]
fn test_parse_simple_vocabulary() {
    let input = r#"
    VOCABULARY TestVocab {
        CODES {
            "key1": "value1",
            "key2": "value2",
            42: "numeric_key"
        }
    }
    "#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Vocabulary(vocab) = &ast.statements[0] {
        assert_eq!(vocab.name, "TestVocab");
        assert_eq!(vocab.bodies.len(), 1);

        let body = &vocab.bodies[0];
        assert_eq!(body.name, "CODES");
        assert_eq!(body.entries.len(), 3);

        // Check string key entry
        let entry1 = &body.entries[0];
        if let VocabularyKey::String(key) = &entry1.key {
            assert_eq!(key, "key1");
            assert_eq!(entry1.value, "value1");
        } else {
            panic!("Expected string key");
        }

        // Check numeric key entry
        let entry3 = &body.entries[2];
        if let VocabularyKey::Number(key) = &entry3.key {
            assert_eq!(*key, 42.0);
            assert_eq!(entry3.value, "numeric_key");
        } else {
            panic!("Expected numeric key");
        }
    } else {
        panic!("Expected vocabulary declaration");
    }
}

#[test]
fn test_parse_catalog() {
    let input = r#"
    catalog test_catalog {
        source "test_source" {
            name = "Test Source";
            config {
                type_code = 1;
            }
        }
    }
    "#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Catalog(catalog) = &ast.statements[0] {
        assert_eq!(catalog.name, "test_catalog");
        assert_eq!(catalog.sources.len(), 1);

        let source = &catalog.sources[0];
        assert_eq!(source.name, "test_source");
        assert!(source.fields.len() > 0);

        // Check for simple assignment
        let has_simple = source
            .fields
            .iter()
            .any(|f| matches!(f, SourceField::Assignment { name, .. } if name == "name"));
        assert!(has_simple, "Should have simple assignment");

        // Check for nested assignment
        let has_nested = source
            .fields
            .iter()
            .any(|f| matches!(f, SourceField::NestedAssignment { name, .. } if name == "config"));
        assert!(has_nested, "Should have nested assignment");
    } else {
        panic!("Expected catalog declaration");
    }
}

#[test]
fn test_parse_family_with_outlet() {
    let input = r#"
    FAMILY "Test Family" {
        OUTLET "Test Outlet" {
            IDENTITY {
                title = "Test Title";
            }
        }
    }
    "#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Family(family) = &ast.statements[0] {
        assert_eq!(family.name, "Test Family");
        assert_eq!(family.members.len(), 1);

        if let FamilyMember::Outlet(outlet) = &family.members[0] {
            assert_eq!(outlet.name, "Test Outlet");
            assert_eq!(outlet.blocks.len(), 1);

            if let OutletBlock::Identity(identity) = &outlet.blocks[0] {
                assert_eq!(identity.fields.len(), 1);

                if let IdentityField::Assignment { name, value, .. } = &identity.fields[0] {
                    assert_eq!(name, "title");
                    if let Expression::String(val) = value {
                        assert_eq!(val, "Test Title");
                    } else {
                        panic!("Expected string value");
                    }
                } else {
                    panic!("Expected assignment field");
                }
            } else {
                panic!("Expected identity block");
            }
        } else {
            panic!("Expected outlet member");
        }
    } else {
        panic!("Expected family declaration");
    }
}

#[test]
fn test_parse_period_syntax() {
    let input = r#"
    FAMILY "Test" {
        OUTLET "Test" {
            IDENTITY {
                historical_titles = [
                    {
                        title = "Test";
                        PERIOD = "1950-01-01" TO "1955-12-31";
                    }
                ];
            }
        }
    }
    "#;
    let ast = parse_input(input);

    // Navigate to the period field
    if let Statement::Family(family) = &ast.statements[0] {
        if let FamilyMember::Outlet(outlet) = &family.members[0] {
            if let OutletBlock::Identity(identity) = &outlet.blocks[0] {
                if let IdentityField::ArrayAssignment { values, .. } = &identity.fields[0] {
                    let obj = &values[0];
                    let has_period = obj
                        .fields
                        .iter()
                        .any(|f| matches!(f, ObjectField::Period { .. }));
                    assert!(has_period, "Should have PERIOD field");
                } else {
                    panic!("Expected array assignment");
                }
            }
        }
    }
}

#[test]
fn test_error_recovery() {
    // Test with invalid syntax - should not panic
    let input = "INVALID_KEYWORD { broken syntax }";
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Should return an error, not panic
    assert!(result.is_err(), "Should return error for invalid syntax");
}

#[test]
fn test_case_insensitive_keywords() {
    let input = "import unit vocabulary family outlet";
    let tokens = tokenize(input);

    let keyword_count = tokens
        .iter()
        .filter(|t| matches!(t.kind, TokenKind::Keyword(_)))
        .count();

    assert_eq!(keyword_count, 5, "Should recognize lowercase keywords");
}

#[test]
fn test_trailing_commas() {
    let input = r#"
    UNIT TestUnit {
        id: ID PRIMARY KEY,
        name: TEXT,
    }
    "#;
    let ast = parse_input(input);

    // Should parse successfully despite trailing comma
    assert_eq!(ast.statements.len(), 1);
    if let Statement::Unit(unit) = &ast.statements[0] {
        assert_eq!(unit.fields.len(), 2);
    }
}

#[test]
fn test_comments_ignored() {
    let input = r#"
    // This is a comment
    UNIT TestUnit { // Another comment
        id: ID PRIMARY KEY, // Field comment
        /* Multi-line
           comment */
        name: TEXT
    }
    "#;
    let ast = parse_input(input);

    // Comments should be ignored during parsing
    assert_eq!(ast.statements.len(), 1);
    if let Statement::Unit(unit) = &ast.statements[0] {
        assert_eq!(unit.name, "TestUnit");
        assert_eq!(unit.fields.len(), 2);
    }
}

#[test]
fn test_parse_event_declaration() {
    let input = r#"
    EVENT express_acquisition_2010 {
        type = "acquisition";
        date = "2010-03-15";
        status = "completed";
        
        entities = {
            acquirer = { 
                id = 200001; 
                role = "buyer"; 
                stake_after = 100; 
            };
            target = { 
                id = 300014; 
                role = "seller"; 
                stake_before = 100; 
            };
        };
        
        impact = {
            transaction_value = 50000000;
            currency = "EUR";
        };
        
        metadata = {
            regulatory_approval = "Austrian_Media_Authority";
        };
        
        @source = "Reuters_2010_03_16"
    }
    "#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Event(event) = &ast.statements[0] {
        assert_eq!(event.name, "express_acquisition_2010");
        
        // Check for type field
        let has_type = event.fields.iter().any(|f| matches!(f, EventField::Type { value, .. } if value == "acquisition"));
        assert!(has_type, "Should have type field");
        
        // Check for date field
        let has_date = event.fields.iter().any(|f| matches!(f, EventField::Date { .. }));
        assert!(has_date, "Should have date field");
        
        // Check for status field
        let has_status = event.fields.iter().any(|f| matches!(f, EventField::Status { value, .. } if value == "completed"));
        assert!(has_status, "Should have status field");
        
        // Check for entities
        let has_entities = event.fields.iter().any(|f| {
            if let EventField::Entities { entities, .. } = f {
                entities.len() == 2 && 
                entities.iter().any(|e| e.name == "acquirer") &&
                entities.iter().any(|e| e.name == "target")
            } else {
                false
            }
        });
        assert!(has_entities, "Should have entities with acquirer and target");
        
        // Check for impact
        let has_impact = event.fields.iter().any(|f| matches!(f, EventField::Impact { .. }));
        assert!(has_impact, "Should have impact field");
        
        // Check for metadata
        let has_metadata = event.fields.iter().any(|f| matches!(f, EventField::Metadata { .. }));
        assert!(has_metadata, "Should have metadata field");
        
        // Check for annotation
        let has_annotation = event.fields.iter().any(|f| matches!(f, EventField::Annotation { name, .. } if name == "source"));
        assert!(has_annotation, "Should have @source annotation");
    } else {
        panic!("Expected event declaration");
    }
}

#[test]
fn test_parse_event_triggered_relationship() {
    let input = r#"
    DIACHRONIC_LINK ownership_change {
        predecessor = 300014;
        successor = 300014;
        event_date = "2010-03-15";
        relationship_type = "ownership_transfer";
        triggered_by_event = express_acquisition_2010;
    }
    "#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Relationship(RelationshipDeclaration::Diachronic(link)) = &ast.statements[0] {
        assert_eq!(link.name, "ownership_change");
        
        // Check for triggered_by_event field
        let has_triggered_by = link.fields.iter().any(|f| {
            matches!(f, DiachronicField::TriggeredByEvent { value, .. } if value == "express_acquisition_2010")
        });
        assert!(has_triggered_by, "Should have triggered_by_event field");
    } else {
        panic!("Expected diachronic link");
    }
}

#[test]
fn test_parse_event_created_synchronous_relationship() {
    let input = r#"
    SYNCHRONOUS_LINK partnership {
        outlet_1 = { id = 200001; role = "owner"; };
        outlet_2 = { id = 300014; role = "subsidiary"; };
        relationship_type = "ownership";
        created_by_event = express_acquisition_2010;
    }
    "#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Relationship(RelationshipDeclaration::Synchronous(link)) = &ast.statements[0] {
        assert_eq!(link.name, "partnership");
        
        // Check for created_by_event field
        let has_created_by = link.fields.iter().any(|f| {
            matches!(f, SynchronousField::CreatedByEvent { value, .. } if value == "express_acquisition_2010")
        });
        assert!(has_created_by, "Should have created_by_event field");
    } else {
        panic!("Expected synchronous link");
    }
}

#[test]
fn test_parse_lifecycle_block() {
    let input = r#"
    FAMILY "Test Family" {
        OUTLET "Test Outlet" {
            IDENTITY {
                id = 100001;
                title = "Test Outlet";
            }
            LIFECYCLE {
                status "active" from "2020-01-01" to "2021-12-31" {
                    circulation = 50000;
                    market_share = 15.2;
                    comment = "Peak performance period";
                }
                status "acquired" from "2022-01-01" {
                    new_owner = "Styria Media";
                    acquisition_value = 75000000;
                }
            }
        }
    }
    "#;
    let ast = parse_input(input);

    assert_eq!(ast.statements.len(), 1);
    if let Statement::Family(family) = &ast.statements[0] {
        assert_eq!(family.members.len(), 1);
        if let FamilyMember::Outlet(outlet) = &family.members[0] {
            assert_eq!(outlet.blocks.len(), 2);
            
            // Check lifecycle block
            if let OutletBlock::Lifecycle(lifecycle) = &outlet.blocks[1] {
                assert_eq!(lifecycle.entries.len(), 2);
                
                // Check first entry (active status)
                let entry1 = &lifecycle.entries[0];
                assert_eq!(entry1.status, "active");
                assert!(matches!(entry1.from, DateExpression::Literal(ref date) if date == "2020-01-01"));
                assert!(matches!(entry1.to, Some(DateExpression::Literal(ref date)) if date == "2021-12-31"));
                assert_eq!(entry1.attributes.len(), 3);
                
                // Check attributes
                let has_circulation = entry1.attributes.iter().any(|attr| {
                    matches!(attr, LifecycleAttribute::Assignment { name, value, .. } 
                        if name == "circulation" && matches!(value, Expression::Number(n) if *n == 50000.0))
                });
                assert!(has_circulation, "Should have circulation attribute");
                
                // Check second entry (acquired status)
                let entry2 = &lifecycle.entries[1];
                assert_eq!(entry2.status, "acquired");
                assert!(matches!(entry2.from, DateExpression::Literal(ref date) if date == "2022-01-01"));
                assert!(entry2.to.is_none());
                assert_eq!(entry2.attributes.len(), 2);
                
                let has_new_owner = entry2.attributes.iter().any(|attr| {
                    matches!(attr, LifecycleAttribute::Assignment { name, value, .. } 
                        if name == "new_owner" && matches!(value, Expression::String(s) if s == "Styria Media"))
                });
                assert!(has_new_owner, "Should have new_owner attribute");
            } else {
                panic!("Expected lifecycle block");
            }
        } else {
            panic!("Expected outlet");
        }
    } else {
        panic!("Expected family");
    }
}

#[test]
fn test_parse_lifecycle_current_date() {
    let input = r#"
    FAMILY "Test" {
        OUTLET "Test" {
            LIFECYCLE {
                status "active" from "2020-01-01" to CURRENT {
                    ongoing = true;
                }
            }
        }
    }
    "#;
    let ast = parse_input(input);

    if let Statement::Family(family) = &ast.statements[0] {
        if let FamilyMember::Outlet(outlet) = &family.members[0] {
            if let OutletBlock::Lifecycle(lifecycle) = &outlet.blocks[0] {
                let entry = &lifecycle.entries[0];
                assert!(matches!(entry.to, Some(DateExpression::Current)));
            }
        }
    }
}

#[test]
fn test_parse_lifecycle_no_to_date() {
    let input = r#"
    FAMILY "Test" {
        OUTLET "Test" {
            LIFECYCLE {
                status "active" from "2020-01-01" {
                    ongoing = true;
                }
            }
        }
    }
    "#;
    let ast = parse_input(input);

    if let Statement::Family(family) = &ast.statements[0] {
        if let FamilyMember::Outlet(outlet) = &family.members[0] {
            if let OutletBlock::Lifecycle(lifecycle) = &outlet.blocks[0] {
                let entry = &lifecycle.entries[0];
                assert!(entry.to.is_none());
                assert_eq!(entry.attributes.len(), 1);
            }
        }
    }
}

#[test]
fn test_parse_lifecycle_multiple_statuses() {
    let input = r#"
    FAMILY "Test" {
        OUTLET "Test" {
            LIFECYCLE {
                status "startup" from "2015-01-01" to "2017-12-31" {
                    funding_stage = "seed";
                }
                status "growth" from "2018-01-01" to "2020-12-31" {
                    funding_stage = "series_a";
                    employees = 50;
                }
                status "mature" from "2021-01-01" {
                    ipo_ready = true;
                }
            }
        }
    }
    "#;
    let ast = parse_input(input);

    if let Statement::Family(family) = &ast.statements[0] {
        if let FamilyMember::Outlet(outlet) = &family.members[0] {
            if let OutletBlock::Lifecycle(lifecycle) = &outlet.blocks[0] {
                assert_eq!(lifecycle.entries.len(), 3);
                
                // Check all status names
                let statuses: Vec<&String> = lifecycle.entries.iter().map(|e| &e.status).collect();
                assert_eq!(statuses, vec!["startup", "growth", "mature"]);
                
                // Check that middle entry has both from and to dates
                let growth_entry = &lifecycle.entries[1];
                assert!(matches!(growth_entry.from, DateExpression::Literal(ref date) if date == "2018-01-01"));
                assert!(matches!(growth_entry.to, Some(DateExpression::Literal(ref date)) if date == "2020-12-31"));
                assert_eq!(growth_entry.attributes.len(), 2);
            }
        }
    }
}

#[test]
fn test_parse_lifecycle_empty_attributes() {
    let input = r#"
    FAMILY "Test" {
        OUTLET "Test" {
            LIFECYCLE {
                status "inactive" from "2023-01-01" {
                }
            }
        }
    }
    "#;
    let ast = parse_input(input);

    if let Statement::Family(family) = &ast.statements[0] {
        if let FamilyMember::Outlet(outlet) = &family.members[0] {
            if let OutletBlock::Lifecycle(lifecycle) = &outlet.blocks[0] {
                let entry = &lifecycle.entries[0];
                assert_eq!(entry.status, "inactive");
                assert_eq!(entry.attributes.len(), 0);
            }
        }
    }
}

#[test]
fn test_parse_lifecycle_with_comments() {
    let input = r#"
    FAMILY "Test" {
        OUTLET "Test" {
            LIFECYCLE {
                status "active" from "2020-01-01" {
                    // This is a comment
                    circulation = 25000;
                    /* Multi-line comment
                       about market position */
                    market_position = "leading";
                }
            }
        }
    }
    "#;
    let ast = parse_input(input);

    if let Statement::Family(family) = &ast.statements[0] {
        if let FamilyMember::Outlet(outlet) = &family.members[0] {
            if let OutletBlock::Lifecycle(lifecycle) = &outlet.blocks[0] {
                let entry = &lifecycle.entries[0];
                // Should have 2 assignments plus 2 comments = 4 total attributes
                assert_eq!(entry.attributes.len(), 4);
                
                let comment_count = entry.attributes.iter()
                    .filter(|attr| matches!(attr, LifecycleAttribute::Comment(_)))
                    .count();
                assert_eq!(comment_count, 2);
            }
        }
    }
}
