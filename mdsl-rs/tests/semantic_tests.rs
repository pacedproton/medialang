//! Semantic analysis tests
//! Tests for validation and advanced semantic checking

use mdsl_rs::parse;
use mdsl_rs::semantic::{validate_program, ValidationSeverity};

#[test]
fn test_semantic_validation_basic() {
    let source = r#"
        LET test_var = "value";
        
        UNIT TestUnit { 
            id: ID PRIMARY KEY;
            name: TEXT(100)
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should validate successfully
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    assert!(errors.is_empty(), "Expected no errors in basic valid program");
}

#[test]
fn test_semantic_validation_duplicate_units() {
    let source = r#"
        UNIT TestUnit { 
            id: ID PRIMARY KEY;
            name: TEXT(100)
        }
        
        UNIT TestUnit { 
            id2: ID PRIMARY KEY;
            name2: TEXT(100) 
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should have error for duplicate unit
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message.contains("already declared") || e.message.contains("redeclared")));
}

#[test]
fn test_semantic_validation_duplicate_vocabularies() {
    let source = r#"
        VOCABULARY TestVocab { 
            CODES { 
                1: "test"
            } 
        }
        
        VOCABULARY TestVocab { 
            CODES { 
                2: "test2"
            } 
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should have error for duplicate vocabulary
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message.contains("already declared") || e.message.contains("redeclared")));
}

#[test]
fn test_semantic_validation_duplicate_families() {
    let source = r#"
        FAMILY "TestFamily" { 
            OUTLET "Outlet1" { 
                id = 1;
                identity { 
                    title = "Outlet1"; 
                } 
            } 
        }
        
        FAMILY "TestFamily" { 
            OUTLET "Outlet2" { 
                id = 2;
                identity { 
                    title = "Outlet2"; 
                } 
            } 
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should have error for duplicate family
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message.contains("already declared") || e.message.contains("redeclared")));
}

#[test]
fn test_semantic_validation_missing_outlet_id() {
    let source = r#"
        FAMILY "TestFamily" {
            OUTLET "TestOutlet" {
                identity {
                    title = "Test Outlet";
                }
            }
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should have error for missing outlet ID
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message.contains("missing") && e.message.contains("id")));
}

#[test]
fn test_semantic_validation_outlet_id_conflicts() {
    let source = r#"
        FAMILY "TestFamily" {
            OUTLET "Outlet1" {
                id = 100;
                identity { 
                    title = "First"; 
                }
            }
            OUTLET "Outlet2" {
                id = 100;
                identity { 
                    title = "Second"; 
                }
            }
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should have error for duplicate outlet ID
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message.contains("already used") || e.message.contains("duplicate")));
}

#[test]
fn test_semantic_validation_empty_structures() {
    let source = r#"
        UNIT EmptyUnit { }
        VOCABULARY EmptyVocab { CODES { } }
        FAMILY "EmptyFamily" { }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should have errors/warnings for empty structures
    let issues: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error || issue.severity == ValidationSeverity::Warning)
        .collect();
    assert!(!issues.is_empty());
    assert!(issues.iter().any(|i| i.message.contains("empty") || i.message.contains("no fields") || i.message.contains("no entries")));
}

#[test]
fn test_semantic_validation_text_field_constraints() {
    let source = r#"
        UNIT TestUnit {
            valid_text: TEXT(255),
            zero_length: TEXT(0),
            huge_text: TEXT(100000)
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should have error for zero length and warning for huge text
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    let warnings: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Warning)
        .collect();
    
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message.contains("zero") || e.message.contains("length")));
    
    assert!(!warnings.is_empty());
    assert!(warnings.iter().any(|w| w.message.contains("large") || w.message.contains("100000")));
}

#[test]
fn test_semantic_validation_category_constraints() {
    let source = r#"
        UNIT TestUnit {
            empty_category: CATEGORY(),
            duplicate_category: CATEGORY("A", "B", "A"),
            valid_category: CATEGORY("X", "Y", "Z")
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should have errors for empty and duplicate categories
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message.contains("empty") || e.message.contains("duplicate")));
}

#[test]
fn test_semantic_validation_vocabulary_key_uniqueness() {
    let source = r#"
        VOCABULARY TestVocab {
            CODES {
                1: "First",
                2: "Second",
                1: "Duplicate"
            }
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Should have error for duplicate vocabulary keys
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message.contains("duplicate") && e.message.contains("key")));
}

#[test]
#[ignore] // Parser doesn't support template syntax (TEMPLATE OUTLET, extends template)
fn test_semantic_validation_comprehensive() {
    let source = r#"
        LET global_var = "test";
        
        UNIT MediaOutlet {
            id: ID PRIMARY KEY,
            name: TEXT(255),
            active: BOOLEAN,
            type: CATEGORY("print", "digital", "broadcast")
        }
        
        VOCABULARY MediaTypes {
            CODES {
                1: "Newspaper",
                2: "Magazine",
                3: "Website"
            }
        }
        
        TEMPLATE OUTLET BaseOutlet {
            CHARACTERISTICS {
                sector = "media";
                active = true;
            }
        }
        
        FAMILY "Media Group" {
            OUTLET "Main Outlet" {
                id = 100;
                identity {
                    title = "Main Media Outlet";
                }
                lifecycle {
                    status "active" FROM "2020-01-01" TO "2021-12-31" {
                        precision_start = "known";
                        precision_end = "known";
                    };
                }
                characteristics {
                    language = "english";
                    sector = "print";
                }
                metadata {
                    notes = "Primary outlet of the group";
                }
            }
            
            OUTLET "Secondary Outlet" {
                id = 200;
                identity {
                    title = "Secondary Outlet";
                }
            }
            
            DATA FOR 100 {
                aggregation = { circulation = "monthly" };
                year 2020 {
                    metrics {
                        circulation = { value = 50000, unit = "copies", source = "audit" };
                        revenue = { value = 1250000, unit = "EUR", source = "accounting" };
                    };
                }
            }
            
            DIACHRONIC_LINK acquisition {
                predecessor = 100;
                successor = 200;
                event_date = "2022-01-01 TO 2022-01-01";
            }
            
            SYNCHRONOUS_LINKS partnership {
                link type = "partnership";
                outlet_1 {
                    ma_id = 100;
                    type = 0;
                }
                outlet_2 {
                    ma_id = 200;
                    type = 0;
                }
                period = "2020-01-01 TO 2021-12-31";
            }
        }
    "#;
    let ast = parse(source).unwrap();
    let result = validate_program(&ast);
    
    // Comprehensive valid program should have minimal issues
    let errors: Vec<_> = result.issues.iter()
        .filter(|issue| issue.severity == ValidationSeverity::Error)
        .collect();
    
    // Should be no critical errors in this well-formed program
    assert!(errors.is_empty(), "Expected no errors, but found: {:#?}", errors);
    
    // May have some warnings, but overall should be valid
    println!("Validation summary: {} total issues", result.issues.len());
    for issue in &result.issues {
        println!("  {:?}: {}", issue.severity, issue.message);
    }
}