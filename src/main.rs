//! # MDSL-RS - Media Description Specification Language in Rust
//!
//! A tutorial-focused implementation of a domain-specific language for media analysis.
//! This project demonstrates hand-crafted lexer/parser implementation in Rust.

mod ast;
mod lexer;
// mod parser;  // We'll implement this next

use lexer::{Lexer, TokenKind};

fn main() {
    println!("[START] MDSL-RS Tutorial - Hand-crafted Lexer Demo");
    println!("===============================================\n");

    // Demo 1: Basic MDSL constructs
    demo_basic_lexing();

    // Demo 2: Real MDSL from freeze3 files
    demo_real_mdsl();

    // Demo 3: Error handling
    demo_error_handling();

    // Demo 4: Complete file parsing
    demo_file_parsing();
}

fn demo_basic_lexing() {
    println!("[DEMO] Demo 1: Basic MDSL Lexing");
    println!("-----------------------------");

    let mdsl_code = r#"
        IMPORT "anmi_common_codes.mdsl";
        LET austria_region = "Österreich gesamt";
        TEMPLATE OUTLET "AustrianNewspaper" {
            characteristics {
                language = "de";
            };
        };
    "#;

    println!("Input MDSL code:");
    println!("{}", mdsl_code);
    println!("\nTokens produced:");

    let lexer = Lexer::new(mdsl_code);
    let mut token_count = 0;

    for token_result in lexer {
        match token_result {
            Ok(token) => {
                println!("  {:?} at {}", token.kind, token.position);
                token_count += 1;
                if token_count > 20 {
                    // Limit output for demo
                    println!("  ... (truncated)");
                    break;
                }
            }
            Err(e) => {
                println!("  [ERROR] Error: {}", e);
                break;
            }
        }
    }
    println!();
}

fn demo_real_mdsl() {
    println!("[DEMO] Demo 2: Real MDSL from Kronen Zeitung");
    println!("----------------------------------------");

    let mdsl_snippet = r#"
        FAMILY "Kronen Zeitung Family" {
            @comment "Austria's largest daily newspaper group";
            
            OUTLET "Kronen Zeitung" EXTENDS TEMPLATE "AustrianNewspaper" {
                id = 200001;
                lifecycle {
                    status "active" FROM "1959-01-01" TO CURRENT {
                        precision_start = "known";
                    };
                };
            };
            
            DATA FOR 200001 {
                YEAR 2021 {
                    metrics {
                        circulation = { 
                            value = 700000; 
                            unit = "copies"; 
                        };
                    };
                };
            };
        };
    "#;

    println!("Input MDSL snippet:");
    println!("{}", mdsl_snippet);
    println!("\nKey tokens identified:");

    let lexer = Lexer::new(mdsl_snippet);

    for token_result in lexer {
        match token_result {
            Ok(token) => {
                match &token.kind {
                    TokenKind::Family => {
                        println!("  [FAMILY] FAMILY keyword at {}", token.position)
                    }
                    TokenKind::Comment(content) => {
                        println!("  [COMMENT] Comment: '{}' at {}", content, token.position)
                    }
                    TokenKind::Outlet => {
                        println!("  [OUTLET] OUTLET keyword at {}", token.position)
                    }
                    TokenKind::Extends => {
                        println!("  [EXTENDS] EXTENDS keyword at {}", token.position)
                    }
                    TokenKind::Current => {
                        println!("  [CURRENT] CURRENT keyword at {}", token.position)
                    }
                    TokenKind::Variable(name) => {
                        println!("  [VAR] Variable: ${} at {}", name, token.position)
                    }
                    TokenKind::NumberLiteral(n) => {
                        println!("  [NUM] Number: {} at {}", n, token.position)
                    }
                    _ => {} // Skip other tokens for brevity
                }
            }
            Err(e) => {
                println!("  [ERROR] Error: {}", e);
                break;
            }
        }
    }
    println!();
}

fn demo_error_handling() {
    println!("[WARNING]  Demo 3: Error Handling");
    println!("--------------------------");

    let invalid_mdsl = r#"
        OUTLET "Test" {
            value = 123.45.67;  // Invalid number
            text = "unterminated string
        };
    "#;

    println!("Input with errors:");
    println!("{}", invalid_mdsl);
    println!("\nLexer error handling:");

    let mut lexer = Lexer::new(invalid_mdsl);

    loop {
        match lexer.next_token() {
            Ok(token) => {
                if token.kind == TokenKind::Eof {
                    break;
                }
                println!("  [OK] Token: {:?} at {}", token.kind, token.position);
            }
            Err(e) => {
                println!("  [ERROR] Lexer error: {}", e);
                break;
            }
        }
    }
    println!();
}

fn demo_file_parsing() {
    println!("[FILE] Demo 4: Complete MDSL File Parsing");
    println!("-------------------------------------");

    // Try to read the test file
    match std::fs::read_to_string("mdsl-rs/tests/fixtures/test_input.mdsl") {
        Ok(content) => {
            println!(
                "Successfully loaded test_input.mdsl ({} bytes)",
                content.len()
            );
            println!("First few lines:");
            for (i, line) in content.lines().take(5).enumerate() {
                println!("  {}: {}", i + 1, line);
            }
            println!("  ...\n");

            analyze_mdsl_file(&content);
        }
        Err(e) => {
            println!("Could not read test_input.mdsl: {}", e);
            println!("Creating a virtual example instead...\n");

            let sample_content = r#"
                IMPORT "test.mdsl";
                LET sample_var = "test value";
                FAMILY "Test Family" {
                    @comment "Sample family";
                    OUTLET "Test Outlet" {
                        id = 12345;
                    };
                };
            "#;

            analyze_mdsl_file(sample_content);
        }
    }
}

fn analyze_mdsl_file(content: &str) {
    let lexer = Lexer::new(content);
    let mut stats = TokenStats::new();

    println!("[VAR] Analyzing MDSL tokens...");

    for token_result in lexer {
        match token_result {
            Ok(token) => {
                stats.record_token(&token.kind);

                // Show interesting tokens
                match &token.kind {
                    TokenKind::Import => {
                        println!("  [IMPORT] Import statement at {}", token.position)
                    }
                    TokenKind::Let => {
                        println!("  [VAR] Variable declaration at {}", token.position)
                    }
                    TokenKind::Template => {
                        println!("  [TEMPLATE] Template definition at {}", token.position)
                    }
                    TokenKind::Family => println!("  [FAMILY] Family block at {}", token.position),
                    TokenKind::Outlet => {
                        println!("  [OUTLET] Outlet definition at {}", token.position)
                    }
                    TokenKind::Comment(content) => {
                        println!(
                            "  [COMMENT] Comment: '{}' at {}",
                            if content.len() > 40 {
                                format!("{}...", &content[..40])
                            } else {
                                content.clone()
                            },
                            token.position
                        );
                    }
                    TokenKind::Variable(name) => {
                        println!(
                            "  [VAR] Variable reference: ${} at {}",
                            name, token.position
                        )
                    }
                    TokenKind::DiachronicLink => {
                        println!("  [EXTENDS] Diachronic link at {}", token.position)
                    }
                    TokenKind::Data => println!("  [DEMO] Data block at {}", token.position),
                    _ => {} // Skip routine tokens
                }
            }
            Err(e) => {
                println!("  [ERROR] Lexer error: {}", e);
                break;
            }
        }
    }

    println!("\n[STATS] Token Statistics:");
    stats.print_summary();
}

#[derive(Default)]
struct TokenStats {
    total_tokens: usize,
    keywords: usize,
    identifiers: usize,
    literals: usize,
    punctuation: usize,
    annotations: usize,
    variables: usize,
}

impl TokenStats {
    fn new() -> Self {
        Self::default()
    }

    fn record_token(&mut self, token_kind: &TokenKind) {
        self.total_tokens += 1;

        match token_kind {
            // Keywords
            TokenKind::Import
            | TokenKind::Let
            | TokenKind::Template
            | TokenKind::Family
            | TokenKind::Outlet
            | TokenKind::Extends
            | TokenKind::BasedOn
            | TokenKind::From
            | TokenKind::To
            | TokenKind::Current
            | TokenKind::Year
            | TokenKind::Data
            | TokenKind::DiachronicLink
            | TokenKind::SynchronousLink
            | TokenKind::Override => {
                self.keywords += 1;
            }

            // Identifiers
            TokenKind::Identifier(_) => {
                self.identifiers += 1;
            }

            // Literals
            TokenKind::StringLiteral(_)
            | TokenKind::NumberLiteral(_)
            | TokenKind::BooleanLiteral(_) => {
                self.literals += 1;
            }

            // Variables
            TokenKind::Variable(_) => {
                self.variables += 1;
            }

            // Annotations
            TokenKind::Comment(_) | TokenKind::MapsTo(_) => {
                self.annotations += 1;
            }

            // Punctuation
            TokenKind::LeftBrace
            | TokenKind::RightBrace
            | TokenKind::LeftBracket
            | TokenKind::RightBracket
            | TokenKind::LeftParen
            | TokenKind::RightParen
            | TokenKind::Semicolon
            | TokenKind::Comma
            | TokenKind::Assign
            | TokenKind::Dot
            | TokenKind::Colon => {
                self.punctuation += 1;
            }

            _ => {} // EOF and others
        }
    }

    fn print_summary(&self) {
        println!("  [DEMO] Total tokens: {}", self.total_tokens);
        println!("  🔤 Keywords: {}", self.keywords);
        println!("  [IDENTIFIERS]  Identifiers: {}", self.identifiers);
        println!("  [DEMO] Literals: {}", self.literals);
        println!("  [VAR] Variables: {}", self.variables);
        println!("  [COMMENT] Annotations: {}", self.annotations);
        println!("  [PUNCTUATION]  Punctuation: {}", self.punctuation);

        if self.total_tokens > 0 {
            println!(
                "  [STATS] Keyword density: {:.1}%",
                (self.keywords as f64 / self.total_tokens as f64) * 100.0
            );
        }
    }
}
