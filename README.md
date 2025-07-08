# MediaLanguage DSL Compiler

A comprehensive Domain-Specific Language (DSL) compiler implemented in Rust for modeling media outlets, companies, and their relationships.

## Overview

This project implements a complete compiler pipeline for the MediaLanguage DSL, which is designed to model complex media industry relationships, company hierarchies, and temporal data. The compiler follows the standard pipeline: **Lexical Analysis → Parsing → Semantic Analysis → Code Generation**.

## Architecture

```
mdsl-rs/
├── src/
│   ├── lexer/          # Tokenization and lexical analysis
│   ├── parser/         # Syntax analysis and AST construction
│   ├── semantic/       # Symbol tables and type checking
│   ├── ir/            # Intermediate representation
│   ├── codegen/       # SQL and Cypher code generation
│   ├── error.rs       # Comprehensive error handling
│   └── main.rs        # CLI application
├── examples/          # Sample MediaLanguage files
└── tests/            # Test files and validation
```

## Quick Start

### Building the Compiler

```bash
cd mdsl-rs
cargo build
```

### Running Examples

```bash
# Tokenize a MediaLanguage file
cargo run -- lex examples/simple_example.mdsl

# Parse to AST
cargo run -- parse examples/simple_example.mdsl

# Generate SQL (feature-gated)
cargo run --features sql-codegen -- sql examples/simple_example.mdsl

# Generate Cypher (feature-gated)
cargo run --features cypher-codegen -- cypher examples/simple_example.mdsl
```

## 📖 Walking Through the Implementation

### 1. **Lexical Analysis** (`src/lexer/`)

Start here to understand how source code becomes tokens:

```rust
// src/lexer/scanner.rs - Main lexer implementation
pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    position: SourcePosition,
    // ...
}
```

**Key Features:**

- Character-by-character scanning
- Keyword recognition (case-insensitive)
- String literal parsing with escape sequences
- Comment handling (`//`, `/* */`, `#`)
- Annotation parsing (`@identifier`)
- Variable reference parsing (`$identifier`)

**Example:**

```mdsl
LET austria_region = "Österreich gesamt";
```

Becomes tokens: `[Keyword(Let), Identifier("austria_region"), Assign, String("Österreich gesamt"), Semicolon]`

### 2. **Abstract Syntax Tree** (`src/parser/ast.rs`)

The AST defines the structure of parsed MediaLanguage code:

```rust
pub enum Statement {
    Import(ImportStatement),
    Variable(VariableDeclaration),
    Unit(UnitDeclaration),
    Vocabulary(VocabularyDeclaration),
    Family(FamilyDeclaration),
    Template(TemplateDeclaration),
    Data(DataDeclaration),
    Relationship(RelationshipDeclaration),
    Comment(CommentStatement),
}
```

**Key Concepts:**

- **Units**: Table/entity definitions with field types
- **Families**: Hierarchical structures containing outlets
- **Templates**: Reusable outlet definitions with inheritance
- **Relationships**: Diachronic and synchronous links between entities
- **Data**: Market data and metrics with temporal information

### 3. **Recursive Descent Parser** (`src/parser/recursive_descent.rs`)

The parser converts tokens into AST nodes:

```rust
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Result<Program> {
        // Parse top-level statements
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(Program::new(statements, position))
    }
}
```

**Parsing Strategy:**

- **Top-down**: Start with program, parse statements, then expressions
- **Error Recovery**: Skip to next statement on error
- **Source Position Tracking**: Every AST node knows its location
- **Robust Handling**: Comments, semicolons, and whitespace

### 4. **Error Handling** (`src/error.rs`)

Comprehensive error types for all compilation phases:

```rust
pub enum Error {
    Lexer(LexerError),      // Unexpected characters, invalid tokens
    Parser(ParserError),     // Syntax errors, missing delimiters
    Semantic(SemanticError), // Type errors, undefined variables
    CodeGen(CodeGenError),   // Generation failures
    Io(String),             // File I/O errors
}
```

### 5. **Semantic Analysis** (`src/semantic/`)

**Symbol Tables** (`src/semantic/symbol_table.rs`):

- Track variable definitions and scopes
- Resolve imports and cross-references
- Validate identifier usage

**Type Checking** (`src/semantic/type_checker.rs`):

- Validate field types in units
- Check expression types
- Ensure template inheritance consistency

**Validation** (`src/semantic/validator.rs`):

- Verify relationship integrity
- Check temporal consistency
- Validate data constraints

### 6. **Intermediate Representation** (`src/ir/`)

The IR provides a language-agnostic representation:

```rust
pub enum IRNode {
    Import(ImportNode),
    Table(TableNode),
    Relationship(RelationshipNode),
    Data(DataNode),
    // ...
}
```

### 7. **Code Generation** (`src/codegen/`)

**SQL Generator** (`src/codegen/sql.rs`):

- Generate CREATE TABLE statements from UNIT declarations
- Handle vocabulary tables with INSERT statements
- Support outlet data tables
- Map DSL types to SQL types (ID → INTEGER, TEXT → VARCHAR, etc.)

**Cypher Generator** (`src/codegen/cypher.rs`):

- Create nodes and relationships
- Handle temporal properties
- Support complex graph queries

## Key MediaLanguage Concepts

### **Units** (Database Tables)

```mdsl
UNIT MediaOutlet {
  id: ID PRIMARY KEY,
  name: TEXT(120),
  sector: NUMBER,
  mandate: CATEGORY(
    "Öffentlich-rechtlich",
    "Privat-kommerziell"
  )
}
```

### **Families** (Company Hierarchies)

```mdsl
FAMILY "Kronen Zeitung Family" {
  OUTLET "Kronen Zeitung" EXTENDS TEMPLATE "AustrianNewspaper" {
    id = 200001;
    identity {
      title = "Kronen Zeitung";
    };
    lifecycle {
      status "active" FROM "1959-01-01" TO CURRENT {
        precision_start = "known";
      };
    };
    characteristics {
      sector = "Tageszeitung";
      distribution = {
        primary_area = $austria_region;
      };
    };
  };
}
```

### **Templates** (Reusable Definitions)

```mdsl
TEMPLATE OUTLET "AustrianNewspaper" {
  characteristics {
    language = "de";
    mandate = "Privat-kommerziell";
  };
  metadata {
    steward = "js";
  };
};
```

### **Relationships** (Temporal Links)

```mdsl
DIACHRONIC_LINK acquisition {
  predecessor = 300001;
  successor = 200001;
  event_date = "1971-01-01" TO "1971-12-31";
  relationship_type = "Akquisition";
};
```

## Development Workflow

### **Adding New Language Features**

1. **Extend Tokens** (`src/lexer/token.rs`):

   ```rust
   pub enum TokenKind {
       // Add new token types
       NewFeature(String),
   }
   ```

2. **Update Lexer** (`src/lexer/scanner.rs`):

   ```rust
   // Add scanning logic for new tokens
   fn scan_new_feature(&mut self) -> Result<Token> {
       // Implementation
   }
   ```

3. **Extend AST** (`src/parser/ast.rs`):

   ```rust
   pub enum Statement {
       // Add new statement types
       NewFeature(NewFeatureStatement),
   }
   ```

4. **Implement Parser** (`src/parser/recursive_descent.rs`):
   ```rust
   fn parse_new_feature(&mut self) -> Result<NewFeatureStatement> {
       // Implementation
   }
   ```

### **Testing**

```bash
# Run all tests
cargo test

# Test specific component
cargo test lexer
cargo test parser
cargo test semantic

# Run with verbose output
cargo test -- --nocapture
```

## 📚 Learning Path

### **For Beginners:**

1. Start with `src/lexer/scanner.rs` - understand tokenization
2. Read `src/parser/ast.rs` - see the data structures
3. Examine `src/parser/recursive_descent.rs` - understand parsing
4. Look at `src/error.rs` - see error handling patterns

### **For Advanced Users:**

1. Study the semantic analysis modules
2. Understand the IR design
3. Examine code generation strategies
4. Look at the CLI implementation

### **For Contributors:**

1. Read the error handling patterns
2. Understand the module organization
3. Study the testing approach
4. Examine the feature flag system

## What Makes This Special

- **Complete Pipeline**: From lexer to code generation
- **Real-world Complexity**: Handles sophisticated grammar with annotations, relationships, and temporal data
- **Production Quality**: Comprehensive error handling with source position tracking
- **Educational**: Clear separation of concerns with modular design
- **Extensible**: Feature-flagged code generation (SQL, Cypher)
- **Working Implementation**: Successfully parses complex MediaLanguage DSL files and generates SQL

This implementation serves as an excellent example of how to build a DSL compiler in Rust, demonstrating best practices in error handling, modular design, and comprehensive testing.
