# COMPREHENSIVE MDSL PROJECT LLM HANDOVER DOCUMENTATION

## PROJECT CONTEXT AND USER REQUIREMENTS

### Critical User Rules

- **ABSOLUTELY NO EMOJIS** in code or documentation (strict user requirement)
- User prefers clean, technical documentation
- Focus on practical implementation over decorative formatting

### Project Goals

This is the **ANMI-ML** project: Media Data Specification Language (MDSL) system for importing Austrian National Media Archive (ANMI) SQL database content and converting to MDSL format.

**Core Objectives:**

1. Parse and validate MDSL syntax for media outlet specifications
2. Import data from PostgreSQL ANMI database (657 media outlets, 1,613 market data records)
3. Generate syntactically correct MDSL files with time series data
4. Ensure referential integrity and handle Unicode Austrian/German characters

## DETAILED CODEBASE ARCHITECTURE

### File Structure with Implementation Details

```
mdsl-rs/
├── src/
│   ├── lexer/
│   │   ├── mod.rs           # Token definitions and lexer interface
│   │   ├── scanner.rs       # Character-by-character scanning logic
│   │   └── token.rs         # Token types (UNIT, VOCABULARY, etc.)
│   ├── parser/
│   │   ├── mod.rs           # Parser interface and public API
│   │   ├── recursive_descent.rs  # Main parsing logic
│   │   ├── ast.rs           # Abstract Syntax Tree definitions
│   │   └── error.rs         # Parser-specific error types
│   ├── semantic/
│   │   ├── mod.rs           # Semantic analysis interface
│   │   ├── symbol_table.rs  # Symbol resolution and scoping
│   │   ├── type_checker.rs  # Type validation for MDSL constructs
│   │   └── validator.rs     # MDSL-specific validation rules
│   ├── ir/
│   │   ├── mod.rs           # Intermediate representation interface
│   │   ├── nodes.rs         # IR node types (IRUnit, IRVocabulary, etc.)
│   │   └── transformer.rs   # AST -> IR transformation
│   ├── codegen/
│   │   ├── mod.rs           # Code generation interface
│   │   ├── sql.rs           # Generate SQL from MDSL
│   │   └── cypher.rs        # Generate Cypher from MDSL
│   ├── import/              # DATABASE IMPORT FUNCTIONALITY (MAIN FOCUS)
│   │   ├── mod.rs           # Core import logic and ANMI-specific functions
│   │   ├── connection.rs    # Database connection handling
│   │   ├── generator.rs     # MDSL text generation from data structures
│   │   └── mapper.rs        # SQL data -> MDSL structure mapping
│   ├── bin/
│   │   ├── test_runner.rs   # Test execution binary
│   │   └── sql_import.rs    # CLI for database import
│   ├── main.rs              # Main MDSL CLI binary
│   ├── lib.rs               # Library interface
│   └── error.rs             # Global error types
```

## CRITICAL IMPLEMENTATION DETAILS

### 1. UNIT Generation Problem (HIGHEST PRIORITY)

**Current Broken Implementation** in `src/import/mod.rs` around line 864:

```rust
fn generate_media_outlets_mdsl(&self, output: &mut String, outlets: &[MediaOutletData]) -> Result<()> {
    // PROBLEM: Generates WRONG format
    writeln!(output, "UNIT {} {{", clean_name)?;
    writeln!(output, "    IDENTITY {{")?;
    writeln!(output, "        name: \"{}\"", outlet.mo_title)?;
    writeln!(output, "        location: \"{}\"", outlet.location.unwrap_or_default())?;
    writeln!(output, "    }}")?;
    writeln!(output, "    LIFECYCLE {{")?;
    // ... more wrong format
}
```

**Required Correct Format** (based on `../MediaLanguage/anmi_core_entity_units.mdsl`):

```rust
fn generate_media_outlets_mdsl(&self, output: &mut String, outlets: &[MediaOutletData]) -> Result<()> {
    for outlet in outlets {
        let clean_name = self.clean_identifier(&outlet.mo_title);
        writeln!(output, "UNIT {} {{", clean_name)?;
        writeln!(output, "    id_mo: ID PRIMARY KEY,")?;
        writeln!(output, "    mo_title: TEXT(120),")?;
        writeln!(output, "    location: TEXT(25),")?;
        if let Some(start_date) = &outlet.start_date {
            writeln!(output, "    start_year: NUMBER,")?;
        }
        writeln!(output, "}}")?;
    }
}
```

### 2. Vocabulary Generation (FIXED BUT DETAILS IMPORTANT)

**Location**: `src/import/mod.rs` line 834-860
**Fix Applied**: Changed from `string_key = "value"` to `numeric_key: "value"`
**Current Working Code**:

```rust
fn generate_vocabulary_from_sources(&self, output: &mut String, sources: &[SourceData]) -> Result<()> {
    writeln!(output, "VOCABULARY DataSources {{")?;
    writeln!(output, "    TYPES {{")?;
    for (i, source) in sources.iter().enumerate() {
        let comma = if i < sources.len() - 1 { "," } else { "" };
        let cleaned_name = self.clean_identifier(&source.source_name);  // KEY FIX
        writeln!(output, "        {}: \"{}\"{}",
            i + 1,  // Numeric key
            cleaned_name.replace('"', "\\\""),
            comma
        )?;
    }
    writeln!(output, "    }}")?;
    writeln!(output, "}}")?;
    Ok(())
}
```

### 3. Unicode Character Handling (CRITICAL FUNCTION)

**Location**: `src/import/mod.rs` line 1179-1207
**Implementation**:

```rust
fn clean_identifier(&self, input: &str) -> String {
    input
        .replace(' ', "_")
        .replace(',', "")
        .replace('[', "_")
        .replace(']', "_")
        .replace('(', "_")
        .replace(')', "_")
        .replace('-', "_")
        .replace('.', "_")
        .replace('"', "_")
        .replace(':', "_")
        .replace('/', "_")
        .replace('\\', "_")
        // Handle German/Austrian Unicode characters
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('Ä', "Ae")
        .replace('Ö', "Oe")
        .replace('Ü', "Ue")
        .replace('ß', "ss")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}
```

**Critical Usage**: Must be applied to ALL user-provided strings, especially:

- Unit names (cannot start with numbers)
- Vocabulary keys and values
- Field names and comments

## DATABASE SCHEMA AND CONNECTION DETAILS

### Connection Information

**Database**: PostgreSQL at `100.77.115.86:5432/cmc`
**Connection String**: `postgresql://postgres:,3cF;0^L$=]vc,*bQAf#@100.77.115.86:5432/cmc`
**Status**: Authentication issues observed - credentials may need refresh

### ANMI Table Structures (Extracted from Previous Analysis)

```sql
-- mo_constant: Core media outlet data (657 records)
CREATE TABLE mo_constant (
    id_mo INTEGER PRIMARY KEY,
    mo_title VARCHAR(120),        -- Media outlet name
    location VARCHAR(25),         -- Geographic location
    start_date DATE,              -- Launch date
    end_date DATE,                -- Termination date (nullable)
    current_status VARCHAR(50),   -- Active/Inactive status
    -- Additional fields...
);

-- mo_year: Time series market data (1,613 records, years 1991-2023)
CREATE TABLE mo_year (
    id_mo INTEGER,                -- FK to mo_constant (many NULLs!)
    year INTEGER,                 -- Data year
    circulation DECIMAL,          -- Circulation numbers
    reach_nat DECIMAL,           -- National reach percentage
    market_share DECIMAL,        -- Market share
    calc VARCHAR(50),            -- Calculation method
    comments TEXT,               -- Frequency patterns and notes
    -- Note: Many records have NULL id_mo but contain valuable data
);

-- sources_names: Reference data sources (54 records)
CREATE TABLE sources_names (
    id_source INTEGER PRIMARY KEY,
    source_name VARCHAR(255)      -- Source description (contains Unicode!)
);

-- Relationship tables (627 total records):
-- 11_succession, 12_amalgamation, 31_main_media_outlet, etc.
-- Pattern: [type_code]_[relationship_name]
```

### Data Volume Analysis (From Previous Runs)

- **Media Outlets**: 657 total from mo_constant
- **Market Data**: 1,613 records spanning 1991-2023 (33 years)
- **Source References**: 54 unique data sources
- **Relationships**: 627 records across 11 relationship types
- **Time Series**: 66 DATA blocks generated (33 years × 2 calculations)

## ANMI SCHEMA PATTERN DETECTION

**Location**: `src/import/mod.rs` function `detect_anmi_table_mapping`
**Implementation Logic**:

```rust
fn detect_anmi_table_mapping(&self, table_name: &str) -> Result<TableMapping> {
    let (entity_type, field_mappings) = match table_name {
        "mo_constant" => {
            // Maps to UNIT declarations
            let mut mappings = HashMap::new();
            mappings.insert("id_mo".to_string(), FieldMapping {
                sql_column: "id_mo".to_string(),
                mdsl_field: "id".to_string(),
                transform: Some(FieldTransform::Direct),
                required: true,
            });
            // ... more mappings
            (MdslEntityType::Unit, mappings)
        }
        "mo_year" => {
            // Maps to DATA blocks with time series
            (MdslEntityType::DataBlock, /* mappings */)
        }
        "sources_names" => {
            // Maps to VOCABULARY entries
            (MdslEntityType::Vocabulary, /* mappings */)
        }
        // Relationship tables pattern matching
        table_name if table_name.starts_with("11_") => /* Succession */,
        table_name if table_name.starts_with("12_") => /* Amalgamation */,
        // ... more patterns
    };
}
```

## PARSER AND LEXER IMPLEMENTATION DETAILS

### Token Recognition (src/lexer/token.rs)

**Key Tokens for UNIT parsing**:

```rust
pub enum TokenType {
    // Keywords
    Unit,           // "UNIT"
    Vocabulary,     // "VOCABULARY"
    Types,          // "TYPES"

    // Identifiers and literals
    Identifier(String),    // Unit names, field names
    Number(i64),          // Numeric values
    String(String),       // String literals

    // Punctuation
    LeftBrace,      // "{"
    RightBrace,     // "}"
    Colon,          // ":"
    Comma,          // ","

    // Types
    Id,             // "ID"
    Text,           // "TEXT"
    Number,         // "NUMBER"
    PrimaryKey,     // "PRIMARY KEY"
}
```

### Parser State Machine (src/parser/recursive_descent.rs)

**UNIT Parsing Logic**:

```rust
fn parse_unit(&mut self) -> Result<UnitDeclaration> {
    self.expect(TokenType::Unit)?;
    let name = self.expect_identifier()?;
    self.expect(TokenType::LeftBrace)?;

    let mut fields = Vec::new();
    while !self.check(TokenType::RightBrace) {
        let field = self.parse_unit_field()?;
        fields.push(field);

        if self.check(TokenType::Comma) {
            self.advance();
        }
    }

    self.expect(TokenType::RightBrace)?;
    Ok(UnitDeclaration { name, fields })
}

fn parse_unit_field(&mut self) -> Result<UnitField> {
    let name = self.expect_identifier()?;
    self.expect(TokenType::Colon)?;
    let field_type = self.parse_field_type()?;
    Ok(UnitField { name, field_type })
}
```

## CARGO CONFIGURATION AND FEATURE FLAGS

### Cargo.toml Features

```toml
[features]
default = ["sql-codegen", "cypher-codegen"]
sql-codegen = []
cypher-codegen = []
repl = []
visualization = []
import = ["tokio", "serde", "toml", "clap", "chrono", "tokio-postgres", "rust_decimal"]
```

**Critical**: The `import` feature MUST be enabled for SQL import functionality:

```bash
cargo run --features="import" --bin sql_import -- generate --connection "..." --output file.mdsl
```

### Binary Targets

```toml
[[bin]]
name = "mdsl"
path = "src/main.rs"              # Main validator/parser

[[bin]]
name = "sql_import"
path = "src/bin/sql_import.rs"    # Database import tool
required-features = ["import"]

[[bin]]
name = "test_runner"
path = "src/bin/test_runner.rs"   # Test execution
```

## VALIDATION SYSTEM DETAILS

### Working Validation (Proven)

```bash
# Test with reference file - THIS WORKS
cargo run --bin mdsl -- validate ../MediaLanguage/anmi_core_entity_units.mdsl
# Output: "Status: PASSED, Total Constructs: 12, Errors: 0"
```

### Current Validation Errors Pattern

1. **Unicode Characters**: "Unexpected character 'Ö' at line:col" (FIXED)
2. **Unit Names Starting with Numbers**: "Unexpected token '12' at line:col, expected unit name" (FIXED)
3. **Wrong UNIT Format**: "Unexpected token 'IDENTITY' at line:col, expected field name" (CURRENT BLOCKER)

## COMPLETE COMMAND REFERENCE

### Development Commands

```bash
# Compile with import features
cargo build --features="import"

# Run unit tests (user memory: use specific test command)
cargo test --test unit_tests

# Validate MDSL files
cargo run --bin mdsl -- validate <file.mdsl>

# Generate from database (when fixed)
cargo run --features="import" --bin sql_import -- generate \
  --connection "postgresql://postgres:,3cF;0^L$=]vc,*bQAf#@100.77.115.86:5432/cmc" \
  --output complete_anmi.mdsl

# Check for syntax issues
cargo run --bin mdsl -- validate file.mdsl 2>&1 | head -20
```

### Debugging Commands

```bash
# Find Unicode characters
grep -n "[^[:ascii:]]" file.mdsl

# Find unit names starting with numbers
grep "^UNIT [0-9]" file.mdsl

# Check vocabulary format
grep -A 5 -B 5 "VOCABULARY" file.mdsl
```

## EXACT NEXT STEPS WITH CODE LOCATIONS

### Step 1: Fix UNIT Generation (CRITICAL)

**File**: `src/import/mod.rs`
**Function**: `generate_media_outlets_mdsl` (around line 864)
**Required Change**: Replace entire function body to generate field definitions instead of IDENTITY/LIFECYCLE blocks

**Before (WRONG)**:

```rust
writeln!(output, "UNIT {} {{", clean_name)?;
writeln!(output, "    IDENTITY {{")?;
writeln!(output, "        name: \"{}\"", outlet.mo_title)?;
```

**After (CORRECT)**:

```rust
writeln!(output, "UNIT {} {{", clean_name)?;
writeln!(output, "    id_mo: ID PRIMARY KEY,")?;
writeln!(output, "    mo_title: TEXT(120),")?;
writeln!(output, "    location: TEXT(25),")?;
writeln!(output, "}}")?;
```

### Step 2: Update MediaOutletData Mapping

**File**: `src/import/mod.rs`
**Struct**: `MediaOutletData` (around line 1210)
**Ensure fields match**: `id_mo`, `mo_title`, `location`, `start_date`, `end_date`

### Step 3: Test Pipeline

1. Fix generation function
2. Recompile: `cargo build --features="import"`
3. Test: `cargo run --features="import" --bin sql_import -- generate --connection "..." --output test.mdsl`
4. Validate: `cargo run --bin mdsl -- validate test.mdsl`

## FILES CONTAINING CORRECT REFERENCE EXAMPLES

### Working MDSL Examples

- `../MediaLanguage/anmi_core_entity_units.mdsl` - Perfect UNIT syntax reference
- `../MediaLanguage/anmi_main.mdsl` - Complete structure example
- `examples/simple_example.mdsl` - Basic syntax

### Generated Files Status

- `complete_anmi_full_final.mdsl` - Vocabulary fixed, UNITs broken, 17,553 lines
- `complete_anmi_full_final.mdsl.bak` - Backup before Unicode fixes
- `complete_anmi_full_final.mdsl.bak2` - Backup before unit name fixes

## ERROR PATTERNS AND SOLUTIONS

### Pattern 1: Unicode Characters

**Error**: `Lexer error: Unexpected character 'Ö' at 35:9`
**Solution**: Apply `clean_identifier()` to ALL user strings
**Status**: FIXED in vocabulary generation

### Pattern 2: Unit Names with Numbers

**Error**: `Parser error: Unexpected token '12' at 70:6, expected unit name`
**Solution**: Add prefix to unit names starting with digits
**Status**: FIXED with sed command

### Pattern 3: Wrong UNIT Structure

**Error**: `Parser error: Unexpected token 'IDENTITY' at 71:5, expected field name`
**Solution**: Rewrite UNIT generation to use field definitions
**Status**: NEEDS FIXING (CURRENT BLOCKER)

## PERFORMANCE AND SCALABILITY NOTES

- Database contains 657 media outlets - manageable size
- Generated file is 313KB (17,553 lines) - reasonable for parser
- Time series data spans 33 years (1991-2023)
- Many mo_year records have NULL id_mo but contain valuable aggregate data
- Connection timeout issues observed - may need retry logic

## SUCCESS CRITERIA CHECKLIST

### Immediate Success (Next Session Goals)

- [ ] UNIT generation produces field definitions instead of IDENTITY blocks
- [ ] Generated MDSL file passes validation without errors
- [ ] Database connection works reliably
- [ ] All 657 media outlets properly converted

### Complete Success

- [ ] Full ANMI dataset imported and validated
- [ ] Time series data integrated correctly
- [ ] Referential integrity maintained
- [ ] Performance acceptable for production use

---

**CRITICAL NOTE FOR NEXT CONTEXT**: The main blocker is the UNIT generation format in `src/import/mod.rs` line ~864. Fix this FIRST before attempting any other work. The parser expects field definitions, not nested blocks.
