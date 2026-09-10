# MediaLanguage DSL Parser Test Suite

This directory contains comprehensive tests for the MediaLanguage DSL parser, including unit tests, integration tests, and a test runner utility.

## Test Structure

### 1. Unit Tests (`unit_tests.rs`)

Tests individual parser components in isolation:

- **Lexer Tests**: Keyword recognition, string/number parsing, identifier tokenization
- **Parser Tests**: Individual construct parsing (imports, variables, units, vocabularies, catalogs, families)
- **Syntax Tests**: Case-insensitive keywords, trailing commas, comment handling
- **Error Recovery**: Graceful handling of invalid syntax

**Run with**: `cargo test --test unit_tests`

### 2. Integration Tests (`integration_tests.rs`)

Tests parsing of real MediaLanguage DSL files:

- **File Parsing**: Tests successful parsing of all known-good DSL files
- **AST Validation**: Verifies correct AST structure for complex constructs
- **Catalog Support**: Tests parsing of catalog sources with nested assignments
- **Vocabulary Support**: Tests nested vocabularies and different key types
- **Family/Outlet Structure**: Tests complex family declarations with outlets and data
- **Period Syntax**: Tests special PERIOD field parsing

**Run with**: `cargo test --test integration_tests`

### 3. Construct Tests (`construct_tests.rs`)

Comprehensive tests covering all major DSL constructs:

- **Basic Constructs**: IMPORT, LET, UNIT, VOCABULARY, CATALOG, TEMPLATE
- **Complex Constructs**: Complete FAMILY with all outlet blocks (IDENTITY, LIFECYCLE, CHARACTERISTICS, METADATA)
- **Advanced Features**: DATA blocks, PERIOD syntax, inheritance (EXTENDS/BASED_ON)
- **Real-world Tests**: Tests against freeze3 files for realistic complexity
- **Edge Cases**: Case-insensitive keywords, trailing commas, nested structures
- **Coverage Verification**: Comprehensive construct coverage matrix

**Run with**: `cargo test --test construct_tests`

### 4. Test Runner (`test_runner.rs`)

Utilities for running and reporting test results:

- **TestResult**: Structure for individual file test results
- **TestSummary**: Aggregated test results with success rates
- **File Testing**: Functions to test individual files or directories
- **Regression Testing**: Tests against known successful files

**Run with**: `cargo test --test test_runner`

## Test Runner Binary

The project includes a dedicated test runner binary (`src/bin/test_runner.rs`) that provides:

### Usage

```bash
# Run regression tests (default)
cargo run --bin test_runner

# Test all files in MediaLanguage directory
cargo run --bin test_runner -- --all

# Test a specific file
cargo run --bin test_runner -- --file ../MediaLanguage/sources.mdsl

# Test all files in a directory
cargo run --bin test_runner -- --directory ../MediaLanguage

# Show help
cargo run --bin test_runner -- --help
```

### Features

- **Regression Testing**: Tests only known successful files
- **Directory Testing**: Tests all .mdsl files in a directory
- **Single File Testing**: Tests a specific file
- **Detailed Reporting**: Shows success rates, statement counts, and error details
- **Exit Codes**: Returns appropriate exit codes for CI/CD integration

## Test Coverage

### Current Status

- **Total DSL Files**: 17 files in MediaLanguage directory
- **Successfully Parsing**: 14 files (82.4% success rate)
- **Unit Tests**: 14 tests covering all parser components
- **Integration Tests**: 10 tests covering real DSL file parsing
- **Construct Tests**: 13 tests providing comprehensive construct coverage

### Successfully Parsing Files

[PASS] anmi_common_codes.mdsl (3 statements)
[PASS] anmi_core_entity_units.mdsl (12 statements)
[PASS] anmi_main.mdsl (17 statements)
[PASS] anmi_mandate_types.mdsl (1 statement)
[PASS] anmi_media_sectors.mdsl (1 statement)
[PASS] express_freeze3.mdsl (14 statements)
[PASS] Medienangebot.mdsl (1 statement)
[PASS] MedienangeboteDiachroneBeziehungen.mdsl (1 statement)
[PASS] MedienangeboteSynchroneBeziehungen.mdsl (1 statement)
[PASS] Medienunternehmen.mdsl (1 statement)
[PASS] MedienunternehmenDiachroneBeziehungen.mdsl.mdsl (1 statement)
[PASS] MedienunternehmenSynchroneBeziehungenMitAnderenUnternehmen.mdsl (1 statement)
[PASS] MedienunternehmenSynchroneBeziehungenMitMedienangeboten.mdsl (1 statement)
[PASS] sources.mdsl (2 statements)

### Still Failing Files

[FAIL] express.mdsl (older version with syntax errors)
[FAIL] anmi_source_references.mdsl (invalid vocabulary key syntax)
[FAIL] anmi_market_data_schemas.mdsl (unsupported import_vocabularies construct)

## Test Categories

### Language Constructs Tested

- **Imports**: File imports with proper path resolution
- **Variables**: LET declarations with string/number values
- **Units**: Table definitions with various field types (ID, TEXT, NUMBER, BOOLEAN, CATEGORY)
- **Vocabularies**: Code/value mappings with string and numeric keys
- **Catalogs**: Source catalog definitions with nested assignments
- **Families**: Media outlet family declarations with outlets and data
- **Templates**: Outlet templates with inheritance
- **Relationships**: Diachronic and synchronic relationship definitions

### Syntax Features Tested

- **Case Insensitivity**: Keywords work in both upper and lower case
- **Trailing Commas**: Allowed in field lists, arrays, and blocks
- **Comments**: Single-line (//) and multi-line (/\* \*/) comments
- **Newlines**: Proper handling of newlines in arrays and blocks
- **Keywords as Fields**: PERIOD, RELATIONSHIP_TYPE, etc. as field names
- **Error Recovery**: Graceful handling of syntax errors

### Parser Components Tested

- **Lexer**: Tokenization of all language elements
- **Recursive Descent Parser**: All parsing methods
- **AST Generation**: Correct AST structure for all constructs
- **Error Handling**: Proper error messages and recovery

## Running All Tests

### Quick Test Suite

```bash
# Run all tests except doctests
cargo test --tests

# Run specific test categories
cargo test --test unit_tests
cargo test --test integration_tests
cargo test --test construct_tests
```

### Comprehensive Testing

```bash
# Run the test examples script
../scripts/test_examples.sh

# Or run individual commands
cargo run --bin test_runner                    # Regression tests
cargo run --bin test_runner -- --all          # All files
cargo test --test unit_tests                   # Unit tests
cargo test --test integration_tests           # Integration tests
cargo test --test construct_tests             # Construct tests
```

## Test Development Guidelines

### Adding New Tests

1. **Unit Tests**: Add to `unit_tests.rs` for individual component testing
2. **Integration Tests**: Add to `integration_tests.rs` for full file parsing
3. **Test Files**: Create minimal test files for specific constructs
4. **Error Cases**: Include tests for error conditions and recovery

### Test Naming Convention

- `test_parse_<construct>`: For parsing specific language constructs
- `test_<component>_<feature>`: For testing specific component features
- `test_<file_name>`: For testing specific DSL files

### Test Structure

- Use helper functions for common operations (parse_file, parse_input)
- Include descriptive assertions with helpful error messages
- Test both positive and negative cases
- Verify AST structure, not just parsing success

## CI/CD Integration

The test suite is designed for CI/CD integration:

### Exit Codes

- `0`: All tests passed
- `1`: Some tests failed (with detailed error reporting)

### Test Reports

- Detailed success/failure reporting
- Statement count validation
- Error message capture
- Progress indicators

### Example CI Configuration

```yaml
test:
  script:
    - cargo test --tests
    - cargo run --bin test_runner
  artifacts:
    reports:
      junit: target/test-results.xml
```

## Future Enhancements

### Planned Test Additions

- **Performance Tests**: Parser performance benchmarks
- **Memory Tests**: Memory usage validation
- **Fuzzing Tests**: Random input testing
- **Semantic Tests**: Semantic analysis validation
- **Code Generation Tests**: SQL/Cypher output validation

### Test Infrastructure

- **Test Data Generation**: Automated test file generation
- **Test Coverage Reports**: Code coverage analysis
- **Test Parallelization**: Parallel test execution
- **Test Categorization**: Test tagging and filtering
