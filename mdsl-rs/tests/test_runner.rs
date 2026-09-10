//! Test runner for MediaLanguage DSL parser tests
//!
//! This module provides utilities for running and reporting test results
//! for both integration and unit tests.

use mdsl_rs::lexer::Scanner;
use mdsl_rs::parser::recursive_descent::Parser;
use std::fs;
use std::path::Path;

/// Test result for a single file
#[derive(Debug, Clone)]
pub struct TestResult {
    pub file_name: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub statements_count: Option<usize>,
}

/// Summary of all test results
#[derive(Debug)]
pub struct TestSummary {
    pub total_files: usize,
    pub successful_files: usize,
    pub failed_files: usize,
    pub success_rate: f64,
    pub results: Vec<TestResult>,
}

impl TestSummary {
    pub fn new() -> Self {
        Self {
            total_files: 0,
            successful_files: 0,
            failed_files: 0,
            success_rate: 0.0,
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        self.total_files += 1;
        if result.success {
            self.successful_files += 1;
        } else {
            self.failed_files += 1;
        }
        self.success_rate = (self.successful_files as f64 / self.total_files as f64) * 100.0;
        self.results.push(result);
    }

    pub fn print_summary(&self) {
        println!("\n=== MediaLanguage DSL Parser Test Summary ===");
        println!("Total files tested: {}", self.total_files);
        println!("Successful: {} [PASS]", self.successful_files);
        println!("Failed: {} [FAIL]", self.failed_files);
        println!("Success rate: {:.1}%", self.success_rate);

        println!("\n=== Detailed Results ===");
        for result in &self.results {
            let status = if result.success { "[PASS]" } else { "[FAIL]" };
            let statements = result
                .statements_count
                .map(|c| format!(" ({} statements)", c))
                .unwrap_or_default();

            println!("{} {}{}", status, result.file_name, statements);

            if let Some(error) = &result.error_message {
                println!("   Error: {}", error);
            }
        }

        if self.failed_files > 0 {
            println!("\n=== Failed Files ===");
            for result in &self.results {
                if !result.success {
                    println!(
                        "[FAIL] {}: {}",
                        result.file_name,
                        result.error_message.as_deref().unwrap_or("Unknown error")
                    );
                }
            }
        }
    }
}

/// Test a single MediaLanguage DSL file
pub fn test_file(file_path: &str) -> TestResult {
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(file_path)
        .to_string();

    match fs::read_to_string(file_path) {
        Ok(content) => {
            let mut scanner = Scanner::new(&content);
            match scanner.scan_tokens() {
                Ok(tokens) => {
                    let mut parser = Parser::new(tokens);
                    match parser.parse() {
                        Ok(ast) => TestResult {
                            file_name,
                            success: true,
                            error_message: None,
                            statements_count: Some(ast.statements.len()),
                        },
                        Err(e) => TestResult {
                            file_name,
                            success: false,
                            error_message: Some(format!("Parse error: {}", e)),
                            statements_count: None,
                        },
                    }
                }
                Err(e) => TestResult {
                    file_name,
                    success: false,
                    error_message: Some(format!("Lexer error: {}", e)),
                    statements_count: None,
                },
            }
        }
        Err(e) => TestResult {
            file_name,
            success: false,
            error_message: Some(format!("File read error: {}", e)),
            statements_count: None,
        },
    }
}

/// Test all MediaLanguage DSL files in a directory
pub fn test_all_files(directory: &str) -> TestSummary {
    let mut summary = TestSummary::new();

    let paths = fs::read_dir(directory).expect("Failed to read directory");

    for path in paths {
        if let Ok(entry) = path {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "mdsl" {
                    let file_path = path.to_string_lossy();
                    let result = test_file(&file_path);
                    summary.add_result(result);
                }
            }
        }
    }

    summary
}

/// List of known successful files for regression testing
pub const KNOWN_SUCCESSFUL_FILES: &[&str] = &[
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

/// Test only the known successful files for regression testing
pub fn test_regression(directory: &str) -> TestSummary {
    let mut summary = TestSummary::new();

    for file_name in KNOWN_SUCCESSFUL_FILES {
        let file_path = format!("{}/{}", directory, file_name);
        let result = test_file(&file_path);
        summary.add_result(result);
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_calculation() {
        let mut summary = TestSummary::new();

        summary.add_result(TestResult {
            file_name: "test1.mdsl".to_string(),
            success: true,
            error_message: None,
            statements_count: Some(5),
        });

        summary.add_result(TestResult {
            file_name: "test2.mdsl".to_string(),
            success: false,
            error_message: Some("Error".to_string()),
            statements_count: None,
        });

        assert_eq!(summary.total_files, 2);
        assert_eq!(summary.successful_files, 1);
        assert_eq!(summary.failed_files, 1);
        assert_eq!(summary.success_rate, 50.0);
    }
}
