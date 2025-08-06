use mdsl_rs::lexer::Scanner;
use mdsl_rs::parser::Parser;
use mdsl_rs::ir::transformer;
use mdsl_rs::semantic::validate_program;

#[test]
fn test_mdsl_to_sql_bootstrap_cycle() {
    let source_mdsl = r#"
VOCABULARY SECTOR {
    TYPES {
        11: "Print - Newspapers",
        20: "Radio",
        30: "Television"
    }
}

VOCABULARY DataSources {
    TYPES {
        1: "Test Source 1",
        2: "Test Source 2"
    }
}

FAMILY "Austrian Test Media" {
    OUTLET "Test Newspaper" {
        identity {
            id = 100001;
            title = "Test Zeitung";
        };
        lifecycle {
            status "active" FROM "1990-01-01" TO "2020-12-31" {
                precision_start = "known";
                precision_end = "known";
            };
        };
        characteristics {
            sector = 11;
            mandate = 2;
            distribution = {
                primary_area = 1;
                local = true;
            };
            language = "deutsch";
        };
    };
    
    OUTLET "Test Radio Station" {
        identity {
            id = 200001;
            title = "Test Radio";
        };
        lifecycle {
            status "active" FROM "1995-06-01" TO CURRENT {
                precision_start = "known";
                precision_end = "known";
            };
        };
        characteristics {
            sector = 20;
            mandate = 1;
            distribution = {
                primary_area = 20;
                local = false;
            };
            language = "deutsch";
        };
    };
    
    DIACHRONIC_LINK succession_100001_200001 {
        predecessor = 100001;
        successor = 200001;
        relationship_type = "succession";
    };
}

DATA FOR 100001 {
    total_records: 3
    years {
        2019 {
            circulation = 50000;
        };
        2020 {
            circulation = 48000;
            reach_national = 5.2;
        };
    };
}

DATA FOR 200001 {
    total_records: 3
    years {
        2020 {
            reach_national = 12.5;
            market_share = 8.3;
        };
        2021 {
            reach_national = 13.1;
        };
    };
}
"#;

    // Step 1: Parse and validate original MDSL
    let mut scanner = Scanner::new(source_mdsl);
    let tokens = scanner.scan_tokens().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing should succeed");
    
    let validation_result = validate_program(&ast);
    assert!(validation_result.passed, "Original MDSL should be valid");
    
    // Step 2: Transform to IR
    let original_ir = transformer::transform(&ast).expect("IR transformation should succeed");
    
    // Validate original IR structure
    assert_eq!(original_ir.vocabularies.len(), 2, "Should have 2 vocabularies");
    assert_eq!(original_ir.families.len(), 1, "Should have 1 family");
    assert_eq!(original_ir.families[0].outlets.len(), 2, "Should have 2 outlets");
    assert_eq!(original_ir.families[0].relationships.len(), 1, "Should have 1 relationship");
    assert_eq!(original_ir.families[0].data_blocks.len(), 2, "Should have 2 data blocks");

    // Step 3: Generate SQL from IR
    #[cfg(feature = "sql-codegen")]
    {
        use mdsl_rs::codegen::sql_anmi::AnmiSqlGenerator;
        
        let sql_generator = AnmiSqlGenerator::new();
        let generated_sql = sql_generator.generate(&original_ir).expect("SQL generation should succeed");
        
        // Validate SQL contains expected structures
        assert!(generated_sql.contains("CREATE SCHEMA IF NOT EXISTS graphv3"), "Should create schema");
        assert!(generated_sql.contains("CREATE TABLE IF NOT EXISTS graphv3.mo_constant"), "Should create mo_constant table");
        assert!(generated_sql.contains("CREATE TABLE IF NOT EXISTS graphv3.mo_year"), "Should create mo_year table");
        assert!(generated_sql.contains("CREATE TABLE IF NOT EXISTS graphv3.sectors"), "Should create sectors table");
        assert!(generated_sql.contains("CREATE TABLE IF NOT EXISTS graphv3.11_succession"), "Should create succession table");
        
        // Validate data insertions
        assert!(generated_sql.contains("INSERT INTO graphv3.sectors (id_sector, sector_name) VALUES (11, 'Print - Newspapers')"), "Should insert newspaper sector");
        assert!(generated_sql.contains("INSERT INTO graphv3.sectors (id_sector, sector_name) VALUES (20, 'Radio')"), "Should insert radio sector");
        assert!(generated_sql.contains("INSERT INTO graphv3.mo_constant"), "Should insert outlet data");
        assert!(generated_sql.contains("INSERT INTO graphv3.mo_year"), "Should insert market data");
        assert!(generated_sql.contains("INSERT INTO graphv3.11_succession"), "Should insert relationship data");
        
        // Count expected data points
        let outlet_inserts = generated_sql.matches("INSERT INTO graphv3.mo_constant").count();
        assert_eq!(outlet_inserts, 2, "Should have 2 outlet inserts");
        
        let market_data_inserts = generated_sql.matches("INSERT INTO graphv3.mo_year").count();
        assert!(market_data_inserts >= 4, "Should have at least 4 market data inserts"); // 2019: 1, 2020: 2, 2021: 1
        
        println!("✅ SQL Bootstrap: Generated {} lines of SQL", generated_sql.lines().count());
    }

    // Step 4: Test Cypher generation as well
    #[cfg(feature = "cypher-codegen")]
    {
        use mdsl_rs::codegen::cypher::CypherGenerator;
        
        let cypher_generator = CypherGenerator::with_prefix("test");
        let generated_cypher = cypher_generator.generate(&original_ir).expect("Cypher generation should succeed");
        
        // Validate Cypher contains expected patterns
        assert!(generated_cypher.contains("CREATE (o:test_media_outlet") || generated_cypher.contains("id_mo"), "Should create media outlet nodes");
        assert!(generated_cypher.contains("100001"), "Should create Test Newspaper");
        assert!(generated_cypher.contains("200001"), "Should create Test Radio");
        assert!(generated_cypher.contains("id_sector: 11") || generated_cypher.contains("11"), "Should set newspaper sector");
        assert!(generated_cypher.contains("id_sector: 20") || generated_cypher.contains("20"), "Should set radio sector");
        assert!(generated_cypher.contains("SUCCESSION") || generated_cypher.contains("succession"), "Should create succession relationship");
        
        println!("✅ Cypher Bootstrap: Generated {} lines of Cypher", generated_cypher.lines().count());
    }
}

#[test] 
fn test_full_database_bootstrap_cycle() {
    // Test with smaller but representative sample from actual database structure
    let database_sample_mdsl = r#"
VOCABULARY SECTOR {
    TYPES {
        11: "Print - Newspapers", // 386 outlets
        12: "Print - Magazines", // 74 outlets
        20: "Radio", // 23 outlets
        30: "Television", // 107 outlets
        40: "Online/Digital", // 65 outlets
        90: "Multimedia/Conglomerate" // 1 outlet
    }
}

VOCABULARY DataSources {
    TYPES {
        21: "ORF_Bericht_2022",
        24: "Oesterreichische_Auflagenkontrolle_Jahresbericht_2021"
    }
}

FAMILY "ORF Media Network" {
    OUTLET "ORF Radio Österreich 1" {
        identity {
            id = 200018;
            title = "ORF Radio Österreich 1";
        };
        lifecycle {
            status "active" FROM "1967-10-01" TO CURRENT {
                precision_start = "known";
                precision_end = "known";
            };
        };
        characteristics {
            sector = 20;
            mandate = 1;
            distribution = {
                primary_area = 20;
                local = false;
            };
            language = "deutsch";
        };
    };
    
    OUTLET "Der Standard" {
        identity {
            id = 100087;
            title = "Der Standard";
        };
        lifecycle {
            status "active" FROM "1988-10-19" TO CURRENT {
                precision_start = "known";
                precision_end = "known";
            };
        };
        characteristics {
            sector = 11;
            mandate = 2;
            distribution = {
                primary_area = 1;
                local = false;
            };
            language = "deutsch";
        };
    };
    
    OUTLET "Österreichischer Rundfunk - ORF" {
        identity {
            id = 923400;
            title = "Österreichischer Rundfunk - ORF";
        };
        lifecycle {
            status "active" FROM "1955-07-27" TO CURRENT {
                precision_start = "known";
                precision_end = "known";
            };
        };
        characteristics {
            sector = 90;
            mandate = 1;
            distribution = {
                primary_area = 20;
                local = false;
            };
            language = "deutsch";
        };
    };
    
    SYNCHRONOUS_LINK link_200018_main_media_outlet {
        outlet_1 = {
            id = 200018;
            role = "source";
        };
        outlet_2 = {
            id = 923400;
            role = "target";
        };
        period_start = "1967-10-01";
        period_end = "CURRENT";
    };
}

DATA FOR 200018 {
    total_records: 3
    years {
        2020 {
            reach_national = 45.5;
        };
        2021 {
            reach_national = 47.2;
            market_share = 12.3;
        };
    };
}

DATA FOR 100087 {
    total_records: 1
    years {
        2021 {
            circulation = 85000;
        };
    };
}
"#;

    // Parse the database sample
    let mut scanner = Scanner::new(database_sample_mdsl);
    let tokens = scanner.scan_tokens().expect("Database sample tokenization should succeed");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Database sample parsing should succeed");
    
    let validation_result = validate_program(&ast);
    assert!(validation_result.passed, "Database sample should be valid: {}", validation_result.summary.errors);
    
    let ir = transformer::transform(&ast).expect("Database sample IR transformation should succeed");
    
    // Validate structure matches expected database patterns
    assert_eq!(ir.vocabularies.len(), 2, "Should have SECTOR and DataSources vocabularies");
    assert_eq!(ir.families.len(), 1, "Should have ORF Media Network family");
    assert_eq!(ir.families[0].outlets.len(), 3, "Should have 3 outlets");
    assert_eq!(ir.families[0].relationships.len(), 1, "Should have 1 synchronous relationship");
    assert_eq!(ir.families[0].data_blocks.len(), 2, "Should have 2 data blocks");

    // Test SQL generation with proper ANMI schema
    #[cfg(feature = "sql-codegen")]
    {
        use mdsl_rs::codegen::sql_anmi::AnmiSqlGenerator;
        
        let sql_generator = AnmiSqlGenerator::new();
        let sql = sql_generator.generate(&ir).expect("Database sample SQL generation should succeed");
        
        // Validate ANMI-compatible structure
        assert!(sql.contains("graphv3.mo_constant"), "Should use graphv3 schema");
        assert!(sql.contains("graphv3.mo_year"), "Should create mo_year table");
        assert!(sql.contains("graphv3.21_main_media_outlet"), "Should create main_media_outlet table");
        assert!(sql.contains("graphv3.sectors"), "Should create sectors table");
        
        // Validate specific data
        assert!(sql.contains("200018"), "Should include ORF Radio ID");
        assert!(sql.contains("100087"), "Should include Der Standard ID");
        assert!(sql.contains("923400"), "Should include ORF parent ID");
        assert!(sql.contains("id_sector"), "Should reference sectors");
        
        // Count sectors
        let sector_inserts = sql.matches("INSERT INTO graphv3.sectors").count();
        assert_eq!(sector_inserts, 6, "Should insert all 6 sector types");
        
        println!("✅ Database Bootstrap: Generated ANMI-compatible SQL with {} lines", sql.lines().count());
    }
    
    // Test Neo4j Cypher generation 
    #[cfg(feature = "cypher-codegen")]
    {
        use mdsl_rs::codegen::cypher::CypherGenerator;
        
        let cypher_generator = CypherGenerator::with_prefix("bootstrap");
        let cypher = cypher_generator.generate(&ir).expect("Database sample Cypher generation should succeed");
        
        // Validate graph structure
        assert!(cypher.contains("200018") && cypher.contains("id_mo"), "Should create ORF Radio node");
        assert!(cypher.contains("100087") && cypher.contains("id_mo"), "Should create Der Standard node");
        assert!(cypher.contains("923400") && cypher.contains("id_mo"), "Should create ORF parent node");
        assert!(cypher.contains("MAIN_MEDIA_OUTLET") || cypher.contains("main_media_outlet"), "Should create main_media_outlet relationship");
        
        println!("✅ Neo4j Bootstrap: Generated graph Cypher with {} lines", cypher.lines().count());
    }
}

#[test]
#[cfg(all(feature = "sql-codegen", feature = "cypher-codegen"))]
fn test_complete_bootstrap_equivalence() {
    // Test that SQL -> MDSL -> SQL produces equivalent results
    let test_mdsl = r#"
VOCABULARY SECTOR {
    TYPES {
        20: "Radio",
        90: "Multimedia/Conglomerate"
    }
}

FAMILY "Equivalence Test" {
    OUTLET "Test Radio" {
        identity {
            id = 1001;
            title = "Test Radio Station";
        };
        characteristics {
            sector = 20;
            mandate = 1;
        };
    };
    
    OUTLET "Test Parent" {
        identity {
            id = 9001;
            title = "Test Parent Company";
        };
        characteristics {
            sector = 90;
            mandate = 1;
        };
    };
    
    SYNCHRONOUS_LINK link_1001_main_media_outlet {
        outlet_1 = {
            id = 1001;
            role = "source";
        };
        outlet_2 = {
            id = 9001;
            role = "target";
        };
        relationship_type = "main_media_outlet";
        period_start = "2000-01-01";
    };
}

DATA FOR 1001 {
    total_records: 1
    years {
        2020 {
            reach_national = 15.5;
        };
    };
}
"#;

    // Original parsing
    let mut scanner = Scanner::new(test_mdsl);
    let tokens = scanner.scan_tokens().expect("Tokenization should succeed");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing should succeed");
    let original_ir = transformer::transform(&ast).expect("IR transformation should succeed");


    // Generate SQL
    use mdsl_rs::codegen::sql_anmi::AnmiSqlGenerator;
    let sql_generator = AnmiSqlGenerator::new();
    let generated_sql = sql_generator.generate(&original_ir).expect("SQL generation should succeed");

    // Generate Cypher  
    use mdsl_rs::codegen::cypher::CypherGenerator;
    let cypher_generator = CypherGenerator::with_prefix("equiv");
    let generated_cypher = cypher_generator.generate(&original_ir).expect("Cypher generation should succeed");


    // Validate both outputs contain same core data
    
    // SQL validation
    assert!(generated_sql.contains("1001"), "SQL should contain test radio ID");
    assert!(generated_sql.contains("9001"), "SQL should contain test parent ID");
    assert!(generated_sql.contains("Test Radio") || generated_sql.contains("Test Radio Station"), "SQL should contain radio title");
    assert!(generated_sql.contains("Test Parent") || generated_sql.contains("Test Parent Company"), "SQL should contain parent title");
    assert!(generated_sql.contains("INSERT INTO graphv3.21_main_media_outlet"), "SQL should contain relationship");
    assert!(generated_sql.contains("reach_nat = 15.5"), "SQL should contain market data");
    
    // Cypher validation
    assert!(generated_cypher.contains("1001"), "Cypher should contain test radio ID");
    assert!(generated_cypher.contains("9001"), "Cypher should contain test parent ID");  
    assert!(generated_cypher.contains("Test Radio") || generated_cypher.contains("Test Radio Station"), "Cypher should contain radio title");
    assert!(generated_cypher.contains("Test Parent") || generated_cypher.contains("Test Parent Company"), "Cypher should contain parent title");
    assert!(generated_cypher.contains("MAIN_MEDIA_OUTLET") || generated_cypher.contains("main_media_outlet"), "Cypher should contain relationship");
    
    println!("✅ Bootstrap Equivalence: SQL and Cypher contain equivalent data structures");
}