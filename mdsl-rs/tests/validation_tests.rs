//! Comprehensive tests for the MediaLanguage DSL validation system
//!
//! This test suite verifies that the semantic validator correctly identifies
//! and reports various types of validation issues.

use mdsl_rs::lexer::Scanner;
use mdsl_rs::parser::recursive_descent::Parser;
use mdsl_rs::semantic::{validate_program, ValidationSeverity};

/// Helper function to parse DSL content and validate it
fn validate_content(content: &str) -> mdsl_rs::semantic::ValidationResult {
    let mut scanner = Scanner::new(content);
    let tokens = scanner.scan_tokens().expect("Lexer should succeed");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parser should succeed");
    validate_program(&ast)
}

/// Helper function to count issues by severity
fn count_issues_by_severity(result: &mdsl_rs::semantic::ValidationResult) -> (usize, usize, usize) {
    let mut errors = 0;
    let mut warnings = 0;
    let mut info = 0;

    for issue in &result.issues {
        match issue.severity {
            ValidationSeverity::Error => errors += 1,
            ValidationSeverity::Warning => warnings += 1,
            ValidationSeverity::Info => info += 1,
        }
    }

    (errors, warnings, info)
}

#[test]
fn test_valid_minimal_program() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
                CHARACTERISTICS {
                    sector = "newspaper"
                }
            }
        }
    "#;

    let result = validate_content(content);
    assert!(result.passed, "Valid program should pass validation");
    assert_eq!(
        result.issues.len(),
        0,
        "Valid program should have no issues"
    );
}

#[test]
fn test_missing_outlet_id_error() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    title = "Test Outlet"
                }
            }
        }
    "#;

    let result = validate_content(content);
    assert!(!result.passed, "Program with missing outlet ID should fail");

    let (errors, _warnings, _info) = count_issues_by_severity(&result);
    assert!(errors > 0, "Should have at least one error");

    // Check for specific error
    let has_id_error = result
        .issues
        .iter()
        .any(|issue| issue.code == "IDENTITY_NO_ID" && issue.severity == ValidationSeverity::Error);
    assert!(has_id_error, "Should have IDENTITY_NO_ID error");
}

#[test]
fn test_missing_outlet_title_warning() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001
                }
            }
        }
    "#;

    let result = validate_content(content);

    let (_errors, warnings, _info) = count_issues_by_severity(&result);
    assert!(warnings > 0, "Should have at least one warning");

    // Check for specific warning
    let has_title_warning = result.issues.iter().any(|issue| {
        issue.code == "IDENTITY_NO_TITLE" && issue.severity == ValidationSeverity::Warning
    });
    assert!(has_title_warning, "Should have IDENTITY_NO_TITLE warning");
}

#[test]
fn test_empty_unit_error() {
    let content = r#"
        UNIT EmptyUnit {
        }
    "#;

    let result = validate_content(content);
    assert!(!result.passed, "Empty unit should fail validation");

    let has_empty_unit_error = result
        .issues
        .iter()
        .any(|issue| issue.code == "UNIT_EMPTY" && issue.severity == ValidationSeverity::Error);
    assert!(has_empty_unit_error, "Should have UNIT_EMPTY error");
}

#[test]
fn test_unit_no_primary_key_warning() {
    let content = r#"
        UNIT TestUnit {
            name: TEXT(100),
            description: TEXT(500)
        }
    "#;

    let result = validate_content(content);

    let has_no_pk_warning = result.issues.iter().any(|issue| {
        issue.code == "UNIT_NO_PRIMARY_KEY" && issue.severity == ValidationSeverity::Warning
    });
    assert!(has_no_pk_warning, "Should have UNIT_NO_PRIMARY_KEY warning");
}

#[test]
fn test_duplicate_field_names_error() {
    let content = r#"
        UNIT TestUnit {
            id: ID PRIMARY KEY,
            name: TEXT(100),
            name: TEXT(200)
        }
    "#;

    let result = validate_content(content);
    assert!(
        !result.passed,
        "Duplicate field names should fail validation"
    );

    let has_duplicate_field_error = result.issues.iter().any(|issue| {
        issue.code == "UNIT_FIELD_DUPLICATE" && issue.severity == ValidationSeverity::Error
    });
    assert!(
        has_duplicate_field_error,
        "Should have UNIT_FIELD_DUPLICATE error"
    );
}

#[test]
fn test_text_field_zero_length_error() {
    let content = r#"
        UNIT TestUnit {
            id: ID PRIMARY KEY,
            empty_text: TEXT(0)
        }
    "#;

    let result = validate_content(content);
    assert!(
        !result.passed,
        "TEXT field with zero length should fail validation"
    );

    let has_zero_length_error = result.issues.iter().any(|issue| {
        issue.code == "FIELD_TEXT_ZERO_LENGTH" && issue.severity == ValidationSeverity::Error
    });
    assert!(
        has_zero_length_error,
        "Should have FIELD_TEXT_ZERO_LENGTH error"
    );
}

#[test]
fn test_text_field_large_length_warning() {
    let content = r#"
        UNIT TestUnit {
            id: ID PRIMARY KEY,
            huge_text: TEXT(100000)
        }
    "#;

    let result = validate_content(content);

    let has_large_text_warning = result.issues.iter().any(|issue| {
        issue.code == "FIELD_TEXT_LARGE" && issue.severity == ValidationSeverity::Warning
    });
    assert!(
        has_large_text_warning,
        "Should have FIELD_TEXT_LARGE warning"
    );
}

#[test]
fn test_empty_category_field_error() {
    let content = r#"
        UNIT TestUnit {
            id: ID PRIMARY KEY,
            status: CATEGORY()
        }
    "#;

    let result = validate_content(content);
    assert!(
        !result.passed,
        "Empty CATEGORY field should fail validation"
    );

    let has_empty_category_error = result.issues.iter().any(|issue| {
        issue.code == "FIELD_CATEGORY_EMPTY" && issue.severity == ValidationSeverity::Error
    });
    assert!(
        has_empty_category_error,
        "Should have FIELD_CATEGORY_EMPTY error"
    );
}

#[test]
fn test_duplicate_category_values_error() {
    let content = r#"
        UNIT TestUnit {
            id: ID PRIMARY KEY,
            status: CATEGORY("active", "inactive", "active")
        }
    "#;

    let result = validate_content(content);
    assert!(
        !result.passed,
        "Duplicate CATEGORY values should fail validation"
    );

    let has_duplicate_category_error = result.issues.iter().any(|issue| {
        issue.code == "FIELD_CATEGORY_DUPLICATE" && issue.severity == ValidationSeverity::Error
    });
    assert!(
        has_duplicate_category_error,
        "Should have FIELD_CATEGORY_DUPLICATE error"
    );
}

#[test]
fn test_empty_vocabulary_error() {
    let content = r#"
        VOCABULARY EmptyVocab {
        }
    "#;

    let result = validate_content(content);
    assert!(!result.passed, "Empty vocabulary should fail validation");

    let has_empty_vocab_error = result
        .issues
        .iter()
        .any(|issue| issue.code == "VOCAB_EMPTY" && issue.severity == ValidationSeverity::Error);
    assert!(has_empty_vocab_error, "Should have VOCAB_EMPTY error");
}

#[test]
fn test_empty_vocabulary_body_warning() {
    let content = r#"
        VOCABULARY TestVocab {
            CODES {
            }
        }
    "#;

    let result = validate_content(content);

    let has_empty_body_warning = result.issues.iter().any(|issue| {
        issue.code == "VOCAB_BODY_EMPTY" && issue.severity == ValidationSeverity::Warning
    });
    assert!(
        has_empty_body_warning,
        "Should have VOCAB_BODY_EMPTY warning"
    );
}

#[test]
fn test_duplicate_vocabulary_keys_error() {
    let content = r#"
        VOCABULARY TestVocab {
            CODES {
                1: "First",
                2: "Second",
                1: "Duplicate"
            }
        }
    "#;

    let result = validate_content(content);
    assert!(
        !result.passed,
        "Duplicate vocabulary keys should fail validation"
    );

    let has_duplicate_key_error = result.issues.iter().any(|issue| {
        issue.code == "VOCAB_DUPLICATE_KEY" && issue.severity == ValidationSeverity::Error
    });
    assert!(
        has_duplicate_key_error,
        "Should have VOCAB_DUPLICATE_KEY error"
    );
}

#[test]
fn test_empty_family_warning() {
    let content = r#"
        FAMILY "Empty Family" {
        }
    "#;

    let result = validate_content(content);

    let has_empty_family_warning = result
        .issues
        .iter()
        .any(|issue| issue.code == "FAMILY_EMPTY" && issue.severity == ValidationSeverity::Warning);
    assert!(has_empty_family_warning, "Should have FAMILY_EMPTY warning");
}

#[test]
fn test_family_no_outlets_warning() {
    let content = r#"
        FAMILY "Test Family" {
            DIACHRONIC_LINK "test_link" {
                predecessor = 100001,
                successor = 100002,
                event_date = "2020-01-01"
            }
        }
    "#;

    let result = validate_content(content);

    let has_no_outlets_warning = result.issues.iter().any(|issue| {
        issue.code == "FAMILY_NO_OUTLETS" && issue.severity == ValidationSeverity::Warning
    });
    assert!(
        has_no_outlets_warning,
        "Should have FAMILY_NO_OUTLETS warning"
    );
}

#[test]
fn test_outlet_no_characteristics_warning() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
            }
        }
    "#;

    let result = validate_content(content);

    let has_no_characteristics_warning = result.issues.iter().any(|issue| {
        issue.code == "OUTLET_NO_CHARACTERISTICS" && issue.severity == ValidationSeverity::Warning
    });
    assert!(
        has_no_characteristics_warning,
        "Should have OUTLET_NO_CHARACTERISTICS warning"
    );
}

#[test]
fn test_empty_lifecycle_warning() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
                LIFECYCLE {
                }
            }
        }
    "#;

    let result = validate_content(content);

    let has_empty_lifecycle_warning = result.issues.iter().any(|issue| {
        issue.code == "LIFECYCLE_EMPTY" && issue.severity == ValidationSeverity::Warning
    });
    assert!(
        has_empty_lifecycle_warning,
        "Should have LIFECYCLE_EMPTY warning"
    );
}

#[test]
fn test_duplicate_lifecycle_status_warning() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
                LIFECYCLE {
                    status "active" from "2020-01-01" {
                        circulation = 50000;
                    }
                    status "active" from "2021-01-01" {
                        circulation = 60000;
                    }
                }
            }
        }
    "#;

    let result = validate_content(content);

    let has_duplicate_status_warning = result.issues.iter().any(|issue| {
        issue.code == "LIFECYCLE_DUPLICATE_STATUS" && issue.severity == ValidationSeverity::Warning
    });
    assert!(
        has_duplicate_status_warning,
        "Should have LIFECYCLE_DUPLICATE_STATUS warning"
    );
}

#[test]
fn test_empty_characteristics_warning() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
                CHARACTERISTICS {
                }
            }
        }
    "#;

    let result = validate_content(content);

    let has_empty_characteristics_warning = result.issues.iter().any(|issue| {
        issue.code == "CHARACTERISTICS_EMPTY" && issue.severity == ValidationSeverity::Warning
    });
    assert!(
        has_empty_characteristics_warning,
        "Should have CHARACTERISTICS_EMPTY warning"
    );
}

#[test]
fn test_duplicate_characteristics_warning() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
                CHARACTERISTICS {
                    sector = "newspaper",
                    sector = "magazine"
                }
            }
        }
    "#;

    let result = validate_content(content);

    let has_duplicate_characteristics_warning = result.issues.iter().any(|issue| {
        issue.code == "CHARACTERISTICS_DUPLICATE" && issue.severity == ValidationSeverity::Warning
    });
    assert!(
        has_duplicate_characteristics_warning,
        "Should have CHARACTERISTICS_DUPLICATE warning"
    );
}

#[test]
fn test_empty_data_declaration_warning() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
            }
            
            DATA FOR 100001 {
            }
        }
    "#;

    let result = validate_content(content);

    let has_empty_data_warning = result
        .issues
        .iter()
        .any(|issue| issue.code == "DATA_EMPTY" && issue.severity == ValidationSeverity::Warning);
    assert!(has_empty_data_warning, "Should have DATA_EMPTY warning");
}

#[test]
fn test_variable_redeclaration_error() {
    let content = r#"
        LET test_var = "first value";
        LET test_var = "second value";
        
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
            }
        }
    "#;

    let result = validate_content(content);
    assert!(
        !result.passed,
        "Variable redeclaration should fail validation"
    );

    let has_redeclaration_error = result
        .issues
        .iter()
        .any(|issue| issue.code == "VAR_REDECLARED" && issue.severity == ValidationSeverity::Error);
    assert!(has_redeclaration_error, "Should have VAR_REDECLARED error");
}

#[test]
fn test_template_redeclaration_error() {
    let content = r#"
        TEMPLATE "TestTemplate" {
            CHARACTERISTICS {
                type = "template"
            }
        }
        
        TEMPLATE "TestTemplate" {
            CHARACTERISTICS {
                type = "duplicate"
            }
        }
        
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
            }
        }
    "#;

    let result = validate_content(content);
    assert!(
        !result.passed,
        "Template redeclaration should fail validation"
    );

    let has_redeclaration_error = result.issues.iter().any(|issue| {
        issue.code == "TEMPLATE_REDECLARED" && issue.severity == ValidationSeverity::Error
    });
    assert!(
        has_redeclaration_error,
        "Should have TEMPLATE_REDECLARED error"
    );
}

#[test]
fn test_unit_redeclaration_error() {
    let content = r#"
        UNIT TestUnit {
            id: ID PRIMARY KEY,
            name: TEXT(100)
        }
        
        UNIT TestUnit {
            id: ID PRIMARY KEY,
            description: TEXT(200)
        }
    "#;

    let result = validate_content(content);
    assert!(!result.passed, "Unit redeclaration should fail validation");

    let has_redeclaration_error = result.issues.iter().any(|issue| {
        issue.code == "UNIT_REDECLARED" && issue.severity == ValidationSeverity::Error
    });
    assert!(has_redeclaration_error, "Should have UNIT_REDECLARED error");
}

#[test]
fn test_vocabulary_redeclaration_error() {
    let content = r#"
        VOCABULARY TestVocab {
            CODES {
                1: "First"
            }
        }
        
        VOCABULARY TestVocab {
            CODES {
                2: "Second"
            }
        }
    "#;

    let result = validate_content(content);
    assert!(
        !result.passed,
        "Vocabulary redeclaration should fail validation"
    );

    let has_redeclaration_error = result.issues.iter().any(|issue| {
        issue.code == "VOCAB_REDECLARED" && issue.severity == ValidationSeverity::Error
    });
    assert!(
        has_redeclaration_error,
        "Should have VOCAB_REDECLARED error"
    );
}

#[test]
fn test_family_redeclaration_error() {
    let content = r#"
        FAMILY "Test Family" {
            OUTLET "Test Outlet 1" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet 1"
                }
            }
        }
        
        FAMILY "Test Family" {
            OUTLET "Test Outlet 2" {
                IDENTITY {
                    id = 100002,
                    title = "Test Outlet 2"
                }
            }
        }
    "#;

    let result = validate_content(content);
    assert!(
        !result.passed,
        "Family redeclaration should fail validation"
    );

    let has_redeclaration_error = result.issues.iter().any(|issue| {
        issue.code == "FAMILY_REDECLARED" && issue.severity == ValidationSeverity::Error
    });
    assert!(
        has_redeclaration_error,
        "Should have FAMILY_REDECLARED error"
    );
}

#[test]
fn test_import_validation_warnings() {
    let content = r#"
        IMPORT "relative/path/../file";
        IMPORT "file_without_extension";
        
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    id = 100001,
                    title = "Test Outlet"
                }
            }
        }
    "#;

    let result = validate_content(content);

    let has_relative_path_info = result.issues.iter().any(|issue| {
        issue.code == "IMPORT_RELATIVE_PATH" && issue.severity == ValidationSeverity::Info
    });
    assert!(
        has_relative_path_info,
        "Should have IMPORT_RELATIVE_PATH info"
    );

    let has_no_extension_warning = result.issues.iter().any(|issue| {
        issue.code == "IMPORT_NO_EXTENSION" && issue.severity == ValidationSeverity::Warning
    });
    assert!(
        has_no_extension_warning,
        "Should have IMPORT_NO_EXTENSION warning"
    );
}

#[test]
fn test_validation_summary_counts() {
    let content = r#"
        UNIT EmptyUnit {
        }
        
        VOCABULARY EmptyVocab {
        }
        
        FAMILY "Test Family" {
            OUTLET "Test Outlet" {
                IDENTITY {
                    title = "Test Outlet"
                }
            }
        }
    "#;

    let result = validate_content(content);

    // Should have multiple errors
    assert!(result.summary.errors >= 3, "Should have at least 3 errors");
    assert_eq!(
        result.summary.total_constructs, 3,
        "Should count 3 constructs (unit, vocab, family)"
    );
    assert!(!result.passed, "Should fail validation");
}
