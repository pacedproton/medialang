//! MediaLanguage DSL Compiler CLI
//!
//! Command-line interface for the MediaLanguage DSL compiler.

use mdsl_rs::{
    lexer::Scanner,
    parser::recursive_descent::Parser,
    semantic::{validate_program, ValidationReporter},
};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [file] [options]", args[0]);
        eprintln!();
        eprintln!("Commands:");
        eprintln!(
            "  validate <file> [--format=text|json|csv] [--no-color] - Validate MediaLanguage file"
        );
        eprintln!("  lex <file>                                            - Tokenize a MediaLanguage file");
        eprintln!("  parse <file>                                          - Parse a MediaLanguage file to AST");
        eprintln!("  sql <file>                                            - Generate SQL from MediaLanguage file");
        eprintln!("  sql-anmi <file>                                      - Generate ANMI-compatible SQL from MediaLanguage file");
        eprintln!("  cypher <file>                                         - Generate Cypher from MediaLanguage file");
        eprintln!("  neo4j-test <file> [--url=URL]                        - Test Cypher generation against Neo4j");
        eprintln!(
            "  test                                                  - Run tests on sample input"
        );
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --format=FORMAT   Output format for validation (text, json, csv)");
        eprintln!("  --no-color        Disable colored output");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "validate" => {
            if args.len() < 3 {
                eprintln!("Error: validate command requires a file argument");
                process::exit(1);
            }
            validate_file(&args[2], &args[3..]);
        }
        "lex" => {
            if args.len() < 3 {
                eprintln!("Error: lex command requires a file argument");
                process::exit(1);
            }
            lex_file(&args[2]);
        }
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: parse command requires a file argument");
                process::exit(1);
            }
            parse_file(&args[2]);
        }
        "sql" => {
            if args.len() < 3 {
                eprintln!("Error: sql command requires a file argument");
                process::exit(1);
            }
            generate_sql(&args[2]);
        }
        "sql-anmi" => {
            if args.len() < 3 {
                eprintln!("Error: sql-anmi command requires a file argument");
                process::exit(1);
            }
            generate_sql_anmi(&args[2]);
        }
        "cypher" => {
            if args.len() < 3 {
                eprintln!("Error: cypher command requires a file argument");
                process::exit(1);
            }
            generate_cypher(&args[2]);
        }
        "neo4j-test" => {
            if args.len() < 3 {
                eprintln!("Error: neo4j-test command requires a file argument");
                process::exit(1);
            }
            test_neo4j(&args[2], &args[3..]);
        }
        "test" => {
            run_tests();
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            process::exit(1);
        }
    }
}

/// Validate a MediaLanguage file and report issues
fn validate_file(filename: &str, options: &[String]) {
    // Parse options
    let mut format = "text";
    let mut use_color = true;

    for option in options {
        if option.starts_with("--format=") {
            format = &option[9..];
        } else if option == "--no-color" {
            use_color = false;
        }
    }

    // Read file
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    // Parse the file
    let mut scanner = Scanner::new(&source);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexer error in '{}': {}", filename, err);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Parse error in '{}': {}", filename, err);
            process::exit(1);
        }
    };

    // Validate the AST
    let validation_result = validate_program(&ast);

    // Output results based on format
    match format {
        "json" => {
            println!("{}", ValidationReporter::format_json(&validation_result));
        }
        "csv" => {
            println!("{}", ValidationReporter::format_csv(&validation_result));
        }
        "text" | _ => {
            if use_color && atty::is(atty::Stream::Stdout) {
                ValidationReporter::print_colored_report(&validation_result, Some(filename));
            } else {
                println!(
                    "{}",
                    ValidationReporter::format_report(&validation_result, Some(filename))
                );
            }
        }
    }

    // Exit with appropriate code
    if !validation_result.passed {
        process::exit(1);
    }
}

/// Tokenize a file and print the tokens
fn lex_file(filename: &str) {
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    println!("Tokenizing file: {}", filename);
    println!("Source code:");
    println!("{}", "=".repeat(50));
    println!("{}", source);
    println!("{}", "=".repeat(50));

    let mut scanner = Scanner::new(&source);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            println!("\nTokens ({} total):", tokens.len());
            println!("{}", "-".repeat(50));
            for (i, token) in tokens.iter().enumerate() {
                println!("{:3}: {:?}", i, token);
            }
        }
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            process::exit(1);
        }
    }
}

/// Parse a file and print the AST
fn parse_file(filename: &str) {
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    println!("Parsing file: {}", filename);

    let mut scanner = Scanner::new(&source);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("AST:");
            println!("{:#?}", ast);
        }
        Err(err) => {
            eprintln!("Parse error: {}", err);
            process::exit(1);
        }
    }
}

/// Generate SQL from a file
fn generate_sql(filename: &str) {
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    println!("Generating SQL from file: {}", filename);

    #[cfg(feature = "sql-codegen")]
    {
        use mdsl_rs::{codegen::sql::SqlGenerator, ir::transformer};

        let mut scanner = Scanner::new(&source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("Lexer error: {}", err);
                process::exit(1);
            }
        };

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(err) => {
                eprintln!("Parse error: {}", err);
                process::exit(1);
            }
        };

        let ir = match transformer::transform(&ast) {
            Ok(ir) => ir,
            Err(err) => {
                eprintln!("IR transformation error: {}", err);
                process::exit(1);
            }
        };

        let generator = SqlGenerator::new();
        match generator.generate(&ir) {
            Ok(sql) => {
                println!("Generated SQL:");
                println!("{}", sql);
            }
            Err(err) => {
                eprintln!("SQL generation error: {}", err);
                process::exit(1);
            }
        }
    }

    #[cfg(not(feature = "sql-codegen"))]
    {
        eprintln!("SQL code generation not enabled (requires 'sql-codegen' feature)");
        process::exit(1);
    }
}

/// Generate ANMI-compatible SQL from a file
fn generate_sql_anmi(filename: &str) {
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    println!("Generating ANMI-compatible SQL from file: {}", filename);

    #[cfg(feature = "sql-codegen")]
    {
        use mdsl_rs::{codegen::sql_anmi::AnmiSqlGenerator, ir::transformer};

        let mut scanner = Scanner::new(&source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("Lexer error: {}", err);
                process::exit(1);
            }
        };

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(err) => {
                eprintln!("Parse error: {}", err);
                process::exit(1);
            }
        };

        let ir = match transformer::transform(&ast) {
            Ok(ir) => ir,
            Err(err) => {
                eprintln!("IR transformation error: {}", err);
                process::exit(1);
            }
        };

        let generator = AnmiSqlGenerator::new();
        match generator.generate(&ir) {
            Ok(sql) => {
                println!("Generated ANMI SQL:");
                println!("{}", sql);
            }
            Err(err) => {
                eprintln!("ANMI SQL generation error: {}", err);
                process::exit(1);
            }
        }
    }

    #[cfg(not(feature = "sql-codegen"))]
    {
        eprintln!("SQL code generation not enabled (requires 'sql-codegen' feature)");
        process::exit(1);
    }
}

/// Generate Cypher from a file
fn generate_cypher(filename: &str) {
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    println!("Generating Cypher from file: {}", filename);

    #[cfg(feature = "cypher-codegen")]
    {
        use mdsl_rs::{codegen::cypher::CypherGenerator, ir::transformer};

        let mut scanner = Scanner::new(&source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("Lexer error: {}", err);
                process::exit(1);
            }
        };

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(err) => {
                eprintln!("Parse error: {}", err);
                process::exit(1);
            }
        };

        let ir = match transformer::transform(&ast) {
            Ok(ir) => ir,
            Err(err) => {
                eprintln!("IR transformation error: {}", err);
                process::exit(1);
            }
        };

        let generator = CypherGenerator::with_prefix("mdsl2");
        match generator.generate(&ir) {
            Ok(cypher) => {
                println!("Generated Cypher:");
                println!("{}", cypher);
            }
            Err(err) => {
                eprintln!("Cypher generation error: {}", err);
                process::exit(1);
            }
        }
    }

    #[cfg(not(feature = "cypher-codegen"))]
    {
        eprintln!("Cypher code generation not enabled (requires 'cypher-codegen' feature)");
        process::exit(1);
    }
}

/// Run tests on sample input
fn run_tests() {
    println!("Running lexer tests...");

    // Test simple unit declaration
    let test_input = r#"
UNIT MediaOutlet {
  id: ID PRIMARY KEY,
  name: TEXT(120),
  sector: NUMBER
}
"#;

    println!("\nTest input:");
    println!("{}", test_input);

    let mut scanner = Scanner::new(test_input);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            println!("\nTokens:");
            for token in &tokens {
                println!("  {:?}", token);
            }

            println!("\nTesting parser...");
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    println!("AST: {:#?}", ast);

                    // Test validation
                    println!("\nTesting validation...");
                    let validation_result = validate_program(&ast);
                    ValidationReporter::print_colored_report(
                        &validation_result,
                        Some("test_input"),
                    );
                }
                Err(err) => {
                    eprintln!("Parse error: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Lexer error: {}", err);
        }
    }
}

/// Test MDSL Cypher generation against Neo4j database
#[cfg(feature = "neo4j")]
fn test_neo4j(filename: &str, options: &[String]) {
    use mdsl_rs::neo4j::Neo4jValidator;
    
    // Parse options
    let mut neo4j_url = "http://100.77.115.86:7475";
    for option in options {
        if option.starts_with("--url=") {
            neo4j_url = &option[6..];
        }
    }
    
    // Read and compile the MDSL file
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };
    
    // Generate Cypher
    let cypher = match mdsl_rs::compile_to_cypher(&source) {
        Ok(cypher) => cypher,
        Err(err) => {
            eprintln!("Error generating Cypher: {}", err);
            process::exit(1);
        }
    };
    
    println!("Generated Cypher:\n{}\n", cypher);
    
    // Test against Neo4j
    let validator = Neo4jValidator::new(neo4j_url);
    
    println!("Testing connection to Neo4j at {}...", neo4j_url);
    match validator.test_connection() {
        Ok(true) => println!("✓ Connected successfully"),
        Ok(false) => {
            eprintln!("✗ Connection test failed");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("✗ Connection error: {}", e);
            process::exit(1);
        }
    }
    
    // Get schema info
    println!("\nFetching database schema...");
    match validator.get_schema() {
        Ok(schema) => {
            println!("Node labels: {:?}", schema.node_labels);
            println!("Relationship types: {:?}", schema.relationship_types);
        }
        Err(e) => {
            eprintln!("Error fetching schema: {}", e);
        }
    }
    
    // Validate the generated Cypher
    println!("\nValidating generated Cypher...");
    match validator.validate_cypher(&cypher) {
        Ok(report) => {
            if report.is_valid {
                println!("✓ Cypher validation passed");
            } else {
                println!("✗ Cypher validation failed");
                if !report.missing_labels.is_empty() {
                    println!("  Missing node labels: {:?}", report.missing_labels);
                }
                if !report.missing_relationships.is_empty() {
                    println!("  Missing relationship types: {:?}", report.missing_relationships);
                }
                if !report.warnings.is_empty() {
                    println!("  Warnings: {:?}", report.warnings);
                }
            }
        }
        Err(e) => {
            eprintln!("Error validating Cypher: {}", e);
        }
    }
}

#[cfg(not(feature = "neo4j"))]
fn test_neo4j(_filename: &str, _options: &[String]) {
    eprintln!("Neo4j testing not enabled (requires 'neo4j' feature)");
    eprintln!("Run with: cargo run --features neo4j --bin mdsl -- neo4j-test <file>");
    process::exit(1);
}
