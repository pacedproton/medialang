# MediaLanguage DSL Implementation in Rust

## Project Structure

```
mdsl-rs/
├── Cargo.toml                    # Main project configuration
├── README.md                     # Project documentation
├── grammar/
│   ├── mdsl.ebnf                 # EBNF grammar specification
│   └── syntax_examples.mdsl      # Example syntax for testing
├── src/
│   ├── lib.rs                    # Library entry point
│   ├── main.rs                   # CLI entry point
│   ├── lexer/
│   │   ├── mod.rs               # Lexer module
│   │   ├── token.rs             # Token definitions
│   │   └── scanner.rs           # Character scanning
│   ├── parser/
│   │   ├── mod.rs               # Parser module
│   │   ├── ast.rs               # AST node definitions
│   │   ├── recursive_descent.rs # Recursive descent parser
│   │   └── error.rs             # Parser error handling
│   ├── semantic/
│   │   ├── mod.rs               # Semantic analysis
│   │   ├── symbol_table.rs      # Symbol table management
│   │   ├── type_checker.rs      # Type checking
│   │   └── validator.rs         # Semantic validation
│   ├── ir/
│   │   ├── mod.rs               # Intermediate representation
│   │   ├── nodes.rs             # IR node definitions
│   │   └── transformer.rs       # AST to IR transformation
│   ├── codegen/
│   │   ├── mod.rs               # Code generation
│   │   ├── sql.rs               # SQL generator
│   │   ├── cypher.rs            # Cypher generator
│   │   └── common.rs            # Common generation utilities
│   ├── repl/
│   │   ├── mod.rs               # REPL implementation
│   │   └── commands.rs          # REPL commands
│   └── utils/
│       ├── mod.rs               # Utility functions
│       ├── error.rs             # Error handling
│       └── source_map.rs        # Source position tracking
├── tests/
│   ├── integration/
│   │   ├── lexer_tests.rs       # Lexer integration tests
│   │   ├── parser_tests.rs      # Parser integration tests
│   │   └── codegen_tests.rs     # Code generation tests
│   └── fixtures/
│       ├── valid/               # Valid test files
│       └── invalid/             # Invalid test files for error testing
└── examples/
    ├── basic_unit.mdsl          # Basic unit definition
    ├── vocabulary.mdsl          # Vocabulary example
    ├── family_outlet.mdsl       # Family/outlet structure
    └── complete_schema.mdsl     # Complete schema example
```

## Architecture Overview

### 1. Multi-Pass Architecture

```
Source Code → Lexer → Parser → AST → Semantic Analysis → IR → Code Generation
```

### 2. Core Components

#### Lexer (Pass 1)

- **Token Recognition**: Identify keywords, identifiers, literals, operators
- **Position Tracking**: Maintain source location for error reporting
- **Comment Handling**: Preserve comments for documentation generation

#### Parser (Pass 2)

- **Recursive Descent**: Handle nested structures and scopes
- **AST Construction**: Build typed AST nodes
- **Error Recovery**: Continue parsing after errors when possible

#### Semantic Analysis (Pass 3)

- **Symbol Resolution**: Resolve variables, imports, and references
- **Type Checking**: Validate data types and assignments
- **Scope Management**: Handle nested scopes and inheritance

#### Intermediate Representation (Pass 4)

- **Normalization**: Convert AST to normalized IR
- **Dependency Resolution**: Resolve import dependencies
- **Template Expansion**: Expand templates and inheritance

#### Code Generation (Pass 5)

- **SQL Generation**: Generate CREATE TABLE statements
- **Cypher Generation**: Generate graph database schema
- **Documentation**: Generate schema documentation

### 3. Key Features

#### Language Support

- **Imports**: File-based module system
- **Variables**: String and numeric constants
- **Units**: Table/entity definitions
- **Vocabularies**: Enumeration definitions
- **Templates**: Inheritance and extension
- **Relationships**: Diachronic and synchronous links
- **Data Definitions**: Market data and metrics

#### Type System

- **Primitive Types**: `ID`, `TEXT(n)`, `NUMBER`, `BOOLEAN`
- **Complex Types**: `CATEGORY(...)`, arrays, objects
- **Constraints**: Primary keys, foreign keys, validation rules

#### Error Handling

- **Rich Diagnostics**: Source location, suggestions, context
- **Error Recovery**: Continue parsing for multiple error reporting
- **Validation**: Semantic validation with detailed messages

## Technology Stack

- **Language**: Rust (latest stable)
- **Dependencies**: Minimal (only std, alloc)
- **Testing**: Built-in Rust testing framework
- **Documentation**: rustdoc
- **Build System**: Cargo
