//! Binary for running MediaLanguage DSL parser tests
//!
//! Usage:
//!   cargo run --bin test_runner -- [OPTIONS]
//!
//! Options:
//!   --all                Test all .mdsl files in MediaLanguage directory
//!   --regression         Test only known successful files (default)
//!   --file <path>        Test a specific file
//!   --directory <path>   Test all files in a specific directory
//!   --help               Show this help message

use std::env;
use std::process;

fn print_help() {
    println!("MediaLanguage DSL Parser Test Runner");
    println!();
    println!("Usage:");
    println!("  cargo run --bin test_runner -- [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --all                Test all .mdsl files in MediaLanguage directory");
    println!("  --regression         Test only known successful files (default)");
    println!("  --file <path>        Test a specific file");
    println!("  --directory <path>   Test all files in a specific directory");
    println!("  --help               Show this help message");
    println!();
    println!("Examples:");
    println!("  cargo run --bin test_runner");
    println!("  cargo run --bin test_runner -- --all");
    println!("  cargo run --bin test_runner -- --file ../MediaLanguage/sources.mdsl");
    println!("  cargo run --bin test_runner -- --directory ../MediaLanguage");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // Default: run regression tests
        println!("Running regression tests on known successful files...");
        run_regression_tests();
        return;
    }

    for i in 1..args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--all" => {
                println!("Testing all .mdsl files in MediaLanguage directory...");
                run_all_tests();
                return;
            }
            "--regression" => {
                println!("Running regression tests on known successful files...");
                run_regression_tests();
                return;
            }
            "--file" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --file requires a file path");
                    process::exit(1);
                }
                let file_path = &args[i + 1];
                println!("Testing file: {}", file_path);
                run_single_file_test(file_path);
                return;
            }
            "--directory" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --directory requires a directory path");
                    process::exit(1);
                }
                let directory = &args[i + 1];
                println!("Testing all files in directory: {}", directory);
                run_directory_tests(directory);
                return;
            }
            _ => {
                eprintln!("Error: Unknown option '{}'", args[i]);
                eprintln!("Use --help for usage information");
                process::exit(1);
            }
        }
    }
}

fn run_regression_tests() {
    // Use the test_runner module from the tests directory
    // For now, we'll implement a simple version here
    let directory = "../MediaLanguage";
    let successful_files = [
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

    let mut total = 0;
    let mut successful = 0;
    let mut failed_files = Vec::new();

    println!("\n=== MediaLanguage DSL Parser Regression Tests ===");

    for file_name in &successful_files {
        let file_path = format!("{}/{}", directory, file_name);
        total += 1;

        match test_file(&file_path) {
            Ok(statements_count) => {
                successful += 1;
                println!("[PASS] {} ({} statements)", file_name, statements_count);
            }
            Err(error) => {
                failed_files.push((file_name, error));
                println!("[FAIL] {}", file_name);
            }
        }
    }

    let success_rate = (successful as f64 / total as f64) * 100.0;

    println!("\n=== Test Summary ===");
    println!("Total files tested: {}", total);
    println!("Successful: {} [PASS]", successful);
    println!("Failed: {} [FAIL]", failed_files.len());
    println!("Success rate: {:.1}%", success_rate);

    if !failed_files.is_empty() {
        println!("\n=== Failed Files ===");
        for (file_name, error) in &failed_files {
            println!("[FAIL] {}: {}", file_name, error);
        }
        process::exit(1);
    }
}

fn run_all_tests() {
    let directory = "../MediaLanguage";
    run_directory_tests(directory);
}

fn run_directory_tests(directory: &str) {
    use std::fs;

    let mut total = 0;
    let mut successful = 0;
    let mut failed_files = Vec::new();

    println!("\n=== Testing all .mdsl files in {} ===", directory);

    match fs::read_dir(directory) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "mdsl" {
                            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                                total += 1;

                                match test_file(&path.to_string_lossy()) {
                                    Ok(statements_count) => {
                                        successful += 1;
                                        println!(
                                            "[PASS] {} ({} statements)",
                                            file_name, statements_count
                                        );
                                    }
                                    Err(error) => {
                                        failed_files.push((file_name.to_string(), error));
                                        println!("[FAIL] {}", file_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory {}: {}", directory, e);
            process::exit(1);
        }
    }

    let success_rate = if total > 0 {
        (successful as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("\n=== Test Summary ===");
    println!("Total files tested: {}", total);
    println!("Successful: {} [PASS]", successful);
    println!("Failed: {} [FAIL]", failed_files.len());
    println!("Success rate: {:.1}%", success_rate);

    if !failed_files.is_empty() {
        println!("\n=== Failed Files ===");
        for (file_name, error) in &failed_files {
            println!("[FAIL] {}: {}", file_name, error);
        }
    }
}

fn run_single_file_test(file_path: &str) {
    println!("\n=== Testing {} ===", file_path);

    match test_file(file_path) {
        Ok(statements_count) => {
            println!("[PASS] Successfully parsed {} statements", statements_count);
        }
        Err(error) => {
            println!("[FAIL] Failed to parse: {}", error);
            process::exit(1);
        }
    }
}

fn test_file(file_path: &str) -> Result<usize, String> {
    use mdsl_rs::lexer::Scanner;
    use mdsl_rs::parser::recursive_descent::Parser;
    use std::fs;

    let content = fs::read_to_string(file_path).map_err(|e| format!("File read error: {}", e))?;

    let mut scanner = Scanner::new(&content);
    let tokens = scanner
        .scan_tokens()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("Parse error: {}", e))?;

    Ok(ast.statements.len())
}
