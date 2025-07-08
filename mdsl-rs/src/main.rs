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
        eprintln!("  cypher <file>                                         - Generate Cypher from MediaLanguage file");
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
        "cypher" => {
            if args.len() < 3 {
                eprintln!("Error: cypher command requires a file argument");
                process::exit(1);
            }
            generate_cypher(&args[2]);
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

        let generator = CypherGenerator::new();
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
