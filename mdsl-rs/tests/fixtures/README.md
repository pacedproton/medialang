# Test Fixtures

This directory contains MDSL test files used as input data for unit and integration tests.

## Files

### `test_input.mdsl`
**Purpose:** Comprehensive test file demonstrating mixed-case syntax and complex MDSL constructs.

**Features tested:**
- Mixed-case keywords (`FAMILY` vs `family`, `OUTLET` vs `outlet`)
- Case-insensitive parser validation
- Complex nested structures (templates, families, outlets)
- Variable declarations and references (`$austria_region`, `$founding_note`)
- Template inheritance (`EXTENDS TEMPLATE`)
- Data blocks and relationships
- Comments and annotations
- Real-world complexity patterns

**Usage in tests:**
- Parser flexibility validation
- Mixed-case syntax testing
- Comprehensive construct coverage
- Regression testing for complex files
- Demo file for `src/main.rs` examples

### `test_variables.mdsl`

**Purpose:** Tests basic variable declaration functionality.

**Features tested:**

- `LET` statement syntax
- String variable values
- Basic UNIT declaration
- Field type definitions (TEXT with size parameters)

**Usage in tests:**

- Unit tests for variable parsing
- Simple parsing validation
- Minimal test case for lexer/parser functionality

### `test_variable_refs.mdsl`

**Purpose:** Tests variable reference functionality with `$variable` syntax.

**Features tested:**

- Variable declaration (`LET` statements)
- Variable references using `$variable_name` syntax
- String and numeric variable types
- Variable usage within complex structures (FAMILY/OUTLET/IDENTITY blocks)
- Integration of variables with outlet definitions

**Usage in tests:**

- Variable reference parsing tests
- Integration tests for variable substitution
- Testing variable scope and usage patterns

## Using Test Fixtures

These files are designed to be used by:

1. **Unit tests** - Testing specific parser components
2. **Integration tests** - Testing complete parsing workflows
3. **Regression tests** - Ensuring consistent parsing behavior
4. **Development testing** - Quick validation during development

## File Characteristics

- **Minimal complexity** - Focus on specific features
- **Valid syntax** - All files should parse successfully
- **Focused testing** - Each file tests a specific aspect of the language
- **Self-contained** - No external dependencies or imports

## Adding New Fixtures

When adding new test fixtures:

1. Keep files small and focused
2. Use descriptive names indicating what feature is being tested
3. Ensure the file parses correctly with the current parser
4. Document the purpose and features tested in this README
5. Consider both positive (valid) and negative (invalid) test cases

## Integration with Test Suite

Test fixtures are referenced in:

- `../unit_tests.rs` - Direct parsing tests
- `../integration_tests.rs` - End-to-end workflow tests
- `../construct_tests.rs` - Specific construct validation
