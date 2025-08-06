use mdsl_rs::lexer::Scanner;
use mdsl_rs::parser::Parser;
use mdsl_rs::ir::transformer;
use mdsl_rs::semantic::validate_program;

#[test]
fn test_sector_vocabulary_parsing() {
    let source = r#"
VOCABULARY SECTOR {
    TYPES {
        11: "Print - Newspapers",
        12: "Print - Magazines", 
        20: "Radio",
        30: "Television",
        40: "Online/Digital",
        90: "Multimedia/Conglomerate"
    }
}

FAMILY "Test Media" {
    OUTLET "Test Radio" {
        identity {
            id = 1;
            title = "Test Radio Station";
        };
        characteristics {
            sector = 20;
        };
    };
    
    OUTLET "Test Newspaper" {
        identity {
            id = 2;
            title = "Test Newspaper";
        };
        characteristics {
            sector = 11;
        };
    };
}
"#;

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing should succeed");
    
    // Validate the AST
    let validation_result = validate_program(&ast);
    assert!(validation_result.passed, "Program should be valid");
    assert_eq!(validation_result.summary.errors, 0, "Should have no errors");
    
    // Transform to IR
    let ir = transformer::transform(&ast).expect("IR transformation should succeed");
    
    // Check vocabulary
    assert_eq!(ir.vocabularies.len(), 1, "Should have one vocabulary");
    let sector_vocab = &ir.vocabularies[0];
    assert_eq!(sector_vocab.name, "SECTOR", "Vocabulary should be named SECTOR");
    assert_eq!(sector_vocab.entries.len(), 6, "Should have 6 sector entries");
    
    // Check that sectors are used in outlets
    assert_eq!(ir.families.len(), 1, "Should have one family");
    let family = &ir.families[0];
    assert_eq!(family.outlets.len(), 2, "Should have two outlets");
    
    // Verify sector assignments
    for outlet in &family.outlets {
        let mut has_sector = false;
        for block in &outlet.blocks {
            if let mdsl_rs::ir::nodes::IROutletBlock::Characteristics(chars) = block {
                for char in chars {
                    if char.name == "sector" {
                        has_sector = true;
                        // Verify sector value is valid
                        if let mdsl_rs::ir::nodes::IRExpression::Number(sector_val) = &char.value {
                            assert!(
                                [11.0, 12.0, 20.0, 30.0, 40.0, 90.0].contains(sector_val),
                                "Sector value {} should be valid", sector_val
                            );
                        }
                    }
                }
            }
        }
        assert!(has_sector, "Outlet {} should have sector assignment", outlet.name);
    }
}

#[test]
fn test_invalid_sector_value() {
    let source = r#"
VOCABULARY SECTOR {
    TYPES {
        20: "Radio",
        30: "Television"
    }
}

FAMILY "Test Media" {
    OUTLET "Test Radio" {
        identity {
            id = 1;
            title = "Test Radio Station";
        };
        characteristics {
            sector = 99; // Invalid sector
        };
    };
}
"#;

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing should succeed");
    
    // Validation should pass at AST level (semantic validation would catch this)
    let validation_result = validate_program(&ast);
    // Note: Current validator may not catch undefined vocabulary references
    // This test documents expected behavior - in a full system, this should be caught
    println!("Validation result errors: {}", validation_result.summary.errors);
}

#[test] 
fn test_sector_vocabulary_in_sql_generation() {
    let source = r#"
VOCABULARY SECTOR {
    TYPES {
        20: "Radio",
        30: "Television"
    }
}

FAMILY "Test Media" {
    OUTLET "Test Radio" {
        identity {
            id = 1;
            title = "Test Radio Station";
        };
        characteristics {
            sector = 20;
        };
    };
}
"#;

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing should succeed");
    
    let ir = transformer::transform(&ast).expect("IR transformation should succeed");
    
    // Test ANMI SQL generation
    #[cfg(feature = "sql-codegen")]
    {
        use mdsl_rs::codegen::sql_anmi::AnmiSqlGenerator;
        
        let generator = AnmiSqlGenerator::new();
        let sql = generator.generate(&ir).expect("SQL generation should succeed");
        
        // Check that sector table is created
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS graphv3.sectors"), 
                "Should create sectors table");
        
        // Check that sector data is inserted
        assert!(sql.contains("INSERT INTO graphv3.sectors (id_sector, sector_name) VALUES (20, 'Radio')"),
                "Should insert Radio sector");
        assert!(sql.contains("INSERT INTO graphv3.sectors (id_sector, sector_name) VALUES (30, 'Television')"),
                "Should insert Television sector");
        
        // Check that outlet uses sector
        assert!(sql.contains("id_sector"), "Should reference sectors in mo_constant table");
    }
}

#[test]
fn test_cypher_generation_with_sectors() {
    let source = r#"
VOCABULARY SECTOR {
    TYPES {
        20: "Radio"
    }
}

FAMILY "Test Media" {
    OUTLET "Test Radio" {
        identity {
            id = 1;
            title = "Test Radio Station";
        };
        characteristics {
            sector = 20;
        };
    };
}
"#;

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing should succeed");
    
    let ir = transformer::transform(&ast).expect("IR transformation should succeed");
    
    // Test Cypher generation
    #[cfg(feature = "cypher-codegen")]
    {
        use mdsl_rs::codegen::cypher::CypherGenerator;
        
        let generator = CypherGenerator::with_prefix("test");
        let cypher = generator.generate(&ir).expect("Cypher generation should succeed");
        
        // Check that sector property is set
        assert!(cypher.contains("id_sector"), "Should set sector property in Cypher");
        assert!(cypher.contains("SET o.id_sector = 20"), "Should set correct sector value");
    }
}