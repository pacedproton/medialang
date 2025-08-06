# GUI Team: Programmatic MDSL Generation Guide

This guide shows how to programmatically generate a new MDSL file from the ANMI SQL database with all recent fixes applied.

## Prerequisites

1. **Rust toolchain** installed
2. **Database access** to the ANMI PostgreSQL database
3. **Build the MDSL compiler** with import feature enabled

## Step 1: Build the Compiler

```bash
cd /Users/mike/localsrc/ANMI-ML/mdsl-rs

# Build with import feature
cargo build --features import --release
```

## Step 2: Generate MDSL from Database

Use the `sql_import` binary to extract the complete database:

```bash
# Generate complete MDSL file
./target/release/sql_import generate \
  --connection "postgresql://postgres:PASSWORD@100.77.115.86:5432/cmc" \
  --output complete_anmi_generated.mdsl
```

### Connection String Format

The connection string follows PostgreSQL format:
```
postgresql://[username]:[password]@[host]:[port]/[database]
```

### Command Line Options

- `--connection`: PostgreSQL connection string
- `--output`: Output MDSL file name
- `--help`: Show all available options

## Step 3: What the Generator Includes

The generated MDSL file will contain:

### 1. Sector Vocabulary (Fixed)
```mdsl
VOCABULARY SECTOR {
    TYPES {
        11: "Print - Newspapers", // 386 outlets
        12: "Print - Magazines", // 74 outlets  
        14: "Print - Other", // 1 outlet
        20: "Radio", // 23 outlets
        30: "Television", // 107 outlets
        40: "Online/Digital", // 65 outlets
        90: "Multimedia/Conglomerate" // 1 outlet
    }
}
```

### 2. Media Outlets with Proper Sector Values
```mdsl
OUTLET "ORF Radio Test" {
    identity {
        id = 200018;
        title = "ORF Radio Test";
    };
    characteristics {
        sector = 20;  // Numeric value, not string
        mandate = 1;
        distribution = {
            primary_area = 20;
            local = false;
        };
        language = "deutsch";
    };
};
```

### 3. Market Data (DATA blocks)
```mdsl
DATA FOR 200018 {
    total_records: 3
    years {
        2020 {
            reach_national = 45.5;
        };
        2021 {
            reach_national = 47.2;
            market_share = 12.3;
        };
    };
}
```

### 4. Complete Coverage
- **657 media outlets** from `mo_constant` table
- **613 relationships** from numbered relationship tables
- **1055 market data records** from `mo_year` table
- **All source references** from `sources_names` table

## Step 4: Validate the Generated File

```bash
# Validate MDSL syntax
cargo run --features sql-codegen -- validate complete_anmi_generated.mdsl

# Generate SQL to verify database equivalence
cargo run --features sql-codegen -- sql complete_anmi_generated.mdsl > regenerated.sql

# Generate Cypher for Neo4j
cargo run --features cypher-codegen -- cypher complete_anmi_generated.mdsl > regenerated.cypher
```

## Step 5: Quality Checks

### Sector Data Quality (All Fixed)
- ✅ Sector vocabulary properly generated from database
- ✅ Sector values are numeric (not strings)
- ✅ All outlets have valid sector assignments
- ✅ Sector names mapped correctly:
  - 11 = "Print - Newspapers"
  - 12 = "Print - Magazines"  
  - 14 = "Print - Other"
  - 20 = "Radio"
  - 30 = "Television"
  - 40 = "Online/Digital"
  - 90 = "Multimedia/Conglomerate"

### Database Coverage Validation
```bash
# Test complete toolchain
cargo run --bin test_runner -- --file complete_anmi_generated.mdsl

# Run comprehensive tests
cargo test --features sql-codegen,cypher-codegen --all
```

## Step 6: Database Generation from MDSL

Once you have generated MDSL files, you can programmatically create fresh databases:

### SQL Database Generation
```bash
# Generate ANMI-compatible SQL from MDSL
./target/release/mdsl sql current_anmi.mdsl > regenerated_anmi.sql

# Execute SQL to create fresh database
psql -h 100.77.115.86 -U postgres -d cmc_fresh < regenerated_anmi.sql
```

### Neo4j Graph Database Generation  
```bash
# Generate Cypher from MDSL with custom prefix
./target/release/mdsl cypher current_anmi.mdsl > regenerated_graph.cypher

# Load into Neo4j (replace with your Neo4j credentials)
cypher-shell -u neo4j -p password -f regenerated_graph.cypher
```

## Step 7: Complete Database Refresh Workflow

### Full Database Refresh Pipeline
```python
import subprocess
import os
import psycopg2
from neo4j import GraphDatabase

class DatabaseRefreshPipeline:
    def __init__(self, mdsl_project_path="/path/to/mdsl-rs"):
        self.project_path = mdsl_project_path
        self.pg_conn = "postgresql://postgres:PASSWORD@100.77.115.86:5432/cmc"
        self.neo4j_uri = "bolt://localhost:7687"
        self.neo4j_auth = ("neo4j", "password")
    
    def refresh_from_scratch(self):
        """Complete database refresh from PostgreSQL source"""
        try:
            # Step 1: Generate fresh MDSL from source database
            print("🔄 Generating MDSL from source database...")
            mdsl_file = self.generate_mdsl_from_source()
            
            # Step 2: Generate SQL for fresh PostgreSQL database
            print("🔄 Generating SQL from MDSL...")
            sql_file = self.generate_sql_from_mdsl(mdsl_file)
            
            # Step 3: Generate Cypher for fresh Neo4j database
            print("🔄 Generating Cypher from MDSL...")
            cypher_file = self.generate_cypher_from_mdsl(mdsl_file)
            
            # Step 4: Create fresh PostgreSQL database
            print("🔄 Creating fresh PostgreSQL database...")
            self.create_fresh_postgres_db(sql_file)
            
            # Step 5: Create fresh Neo4j database
            print("🔄 Creating fresh Neo4j database...")
            self.create_fresh_neo4j_db(cypher_file)
            
            print("✅ Database refresh complete!")
            return {
                "mdsl_file": mdsl_file,
                "sql_file": sql_file, 
                "cypher_file": cypher_file,
                "status": "success"
            }
            
        except Exception as e:
            print(f"❌ Database refresh failed: {e}")
            return {"status": "error", "error": str(e)}
    
    def generate_mdsl_from_source(self):
        """Generate MDSL from source PostgreSQL database"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_file = f"anmi_generated_{timestamp}.mdsl"
        
        cmd = [
            "./target/release/sql_import", "generate",
            "--connection", self.pg_conn,
            "--output", output_file
        ]
        
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=self.project_path)
        
        if result.returncode != 0:
            raise Exception(f"MDSL generation failed: {result.stderr}")
        
        return output_file
    
    def generate_sql_from_mdsl(self, mdsl_file):
        """Generate ANMI-compatible SQL from MDSL"""
        sql_file = mdsl_file.replace('.mdsl', '_anmi.sql')
        
        cmd = ["./target/release/mdsl", "sql", mdsl_file]
        
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=self.project_path)
        
        if result.returncode != 0:
            raise Exception(f"SQL generation failed: {result.stderr}")
        
        # Write SQL output to file
        with open(os.path.join(self.project_path, sql_file), 'w') as f:
            f.write(result.stdout)
        
        return sql_file
    
    def generate_cypher_from_mdsl(self, mdsl_file):
        """Generate Cypher from MDSL"""
        cypher_file = mdsl_file.replace('.mdsl', '_graph.cypher')
        
        cmd = ["./target/release/mdsl", "cypher", mdsl_file]
        
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=self.project_path)
        
        if result.returncode != 0:
            raise Exception(f"Cypher generation failed: {result.stderr}")
        
        # Write Cypher output to file
        with open(os.path.join(self.project_path, cypher_file), 'w') as f:
            f.write(result.stdout)
        
        return cypher_file
    
    def create_fresh_postgres_db(self, sql_file):
        """Create fresh PostgreSQL database"""
        # Drop and recreate database
        conn = psycopg2.connect(self.pg_conn.replace('/cmc', '/postgres'))
        conn.autocommit = True
        cursor = conn.cursor()
        
        cursor.execute("DROP DATABASE IF EXISTS cmc_fresh;")
        cursor.execute("CREATE DATABASE cmc_fresh;")
        
        cursor.close()
        conn.close()
        
        # Execute SQL file
        fresh_conn = self.pg_conn.replace('/cmc', '/cmc_fresh')
        cmd = ["psql", fresh_conn, "-f", sql_file]
        
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=self.project_path)
        
        if result.returncode != 0:
            raise Exception(f"PostgreSQL creation failed: {result.stderr}")
    
    def create_fresh_neo4j_db(self, cypher_file):
        """Create fresh Neo4j database"""
        driver = GraphDatabase.driver(self.neo4j_uri, auth=self.neo4j_auth)
        
        with driver.session() as session:
            # Clear existing data
            session.run("MATCH (n) DETACH DELETE n")
            
            # Execute Cypher file
            with open(os.path.join(self.project_path, cypher_file), 'r') as f:
                cypher_content = f.read()
            
            # Split and execute Cypher statements
            statements = [stmt.strip() for stmt in cypher_content.split(';') if stmt.strip()]
            
            for statement in statements:
                if statement:
                    session.run(statement)
        
        driver.close()

# Usage example
if __name__ == "__main__":
    pipeline = DatabaseRefreshPipeline("/Users/mike/localsrc/ANMI-ML/mdsl-rs")
    result = pipeline.refresh_from_scratch()
    print(f"Refresh result: {result}")
```

## Step 8: Integration in GUI Applications

### Option A: Simple Command Line Integration
```python
import subprocess
import os
from datetime import datetime

def generate_mdsl_from_database():
    """Generate fresh MDSL from database"""
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_file = f"anmi_generated_{timestamp}.mdsl"
    
    cmd = [
        "./target/release/sql_import", "generate",
        "--connection", "postgresql://postgres:PASSWORD@100.77.115.86:5432/cmc",
        "--output", output_file
    ]
    
    result = subprocess.run(cmd, capture_output=True, text=True, cwd="/path/to/mdsl-rs")
    
    if result.returncode == 0:
        return output_file
    else:
        raise Exception(f"MDSL generation failed: {result.stderr}")

def generate_databases_from_mdsl(mdsl_file):
    """Generate both SQL and Cypher from MDSL"""
    base_name = mdsl_file.replace('.mdsl', '')
    
    # Generate SQL
    sql_cmd = ["./target/release/mdsl", "sql", mdsl_file]
    sql_result = subprocess.run(sql_cmd, capture_output=True, text=True, cwd="/path/to/mdsl-rs")
    
    if sql_result.returncode == 0:
        with open(f"{base_name}_anmi.sql", 'w') as f:
            f.write(sql_result.stdout)
    
    # Generate Cypher
    cypher_cmd = ["./target/release/mdsl", "cypher", mdsl_file]
    cypher_result = subprocess.run(cypher_cmd, capture_output=True, text=True, cwd="/path/to/mdsl-rs")
    
    if cypher_result.returncode == 0:
        with open(f"{base_name}_graph.cypher", 'w') as f:
            f.write(cypher_result.stdout)
    
    return {
        "sql_file": f"{base_name}_anmi.sql" if sql_result.returncode == 0 else None,
        "cypher_file": f"{base_name}_graph.cypher" if cypher_result.returncode == 0 else None
    }
```

### Option B: Direct Rust Library Integration
```rust
use mdsl_rs::import::SqlImporter;
use mdsl_rs::lexer::Scanner;
use mdsl_rs::parser::Parser;
use mdsl_rs::ir::transformer;
use mdsl_rs::codegen::sql_anmi::AnmiSqlGenerator;
use mdsl_rs::codegen::cypher::CypherGenerator;

fn complete_database_refresh() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Generate MDSL from database
    let connection_string = "postgresql://postgres:PASSWORD@100.77.115.86:5432/cmc";
    let importer = SqlImporter::new(connection_string)?;
    let mdsl_content = importer.generate_mdsl()?;
    
    std::fs::write("generated.mdsl", &mdsl_content)?;
    
    // Step 2: Parse MDSL to IR
    let mut scanner = Scanner::new(&mdsl_content);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let ir = transformer::transform(&ast)?;
    
    // Step 3: Generate SQL
    let sql_generator = AnmiSqlGenerator::new();
    let sql_content = sql_generator.generate(&ir)?;
    std::fs::write("generated_anmi.sql", &sql_content)?;
    
    // Step 4: Generate Cypher
    let cypher_generator = CypherGenerator::with_prefix("fresh");
    let cypher_content = cypher_generator.generate(&ir)?;
    std::fs::write("generated_graph.cypher", &cypher_content)?;
    
    Ok(())
}
```

## Step 9: Available Command-Line Tools

The MDSL toolchain provides three main binaries:

### 1. `sql_import` - Database to MDSL Converter
```bash
# Generate MDSL from PostgreSQL database
./target/release/sql_import generate \
  --connection "postgresql://user:pass@host:5432/db" \
  --output generated.mdsl

# Available options
./target/release/sql_import --help
```

### 2. `mdsl` - MDSL Compiler (Main Tool)
```bash
# Validate MDSL syntax
./target/release/mdsl validate input.mdsl

# Generate standard SQL
./target/release/mdsl sql input.mdsl > output.sql

# Generate Cypher for Neo4j
./target/release/mdsl cypher input.mdsl > output.cypher

# Parse to tokens (debug)
./target/release/mdsl lex input.mdsl

# Parse to AST (debug) 
./target/release/mdsl parse input.mdsl

# Available commands
./target/release/mdsl --help
```

### 3. `test_runner` - Test Suite Runner
```bash
# Run all MDSL test files
./target/release/test_runner --all

# Test specific file
./target/release/test_runner --file path/to/test.mdsl

# Test with verbose output
./target/release/test_runner --verbose
```

## Step 10: Programmatic Database Refresh Examples

### Quick Refresh Script (Bash)
```bash
#!/bin/bash
# refresh_databases.sh - Complete database refresh from scratch

set -e  # Exit on any error

MDSL_DIR="/Users/mike/localsrc/ANMI-ML/mdsl-rs"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
PG_CONN="postgresql://postgres:PASSWORD@100.77.115.86:5432/cmc"

cd "$MDSL_DIR"

echo "🔄 Step 1: Building MDSL compiler..."
cargo build --features import,sql-codegen,cypher-codegen --release

echo "🔄 Step 2: Generating MDSL from source database..."
MDSL_FILE="anmi_generated_${TIMESTAMP}.mdsl"
./target/release/sql_import generate --connection "$PG_CONN" --output "$MDSL_FILE"

echo "🔄 Step 3: Generating SQL from MDSL..."
SQL_FILE="anmi_generated_${TIMESTAMP}.sql"
./target/release/mdsl sql "$MDSL_FILE" > "$SQL_FILE"

echo "🔄 Step 4: Generating Cypher from MDSL..."  
CYPHER_FILE="anmi_generated_${TIMESTAMP}.cypher"
./target/release/mdsl cypher "$MDSL_FILE" > "$CYPHER_FILE"

echo "🔄 Step 5: Validating generated MDSL..."
./target/release/mdsl validate "$MDSL_FILE"

echo "✅ Database refresh complete!"
echo "📁 Generated files:"
echo "   MDSL: $MDSL_FILE"
echo "   SQL:  $SQL_FILE" 
echo "   Cypher: $CYPHER_FILE"
echo ""
echo "📊 Statistics:"
echo "   MDSL lines: $(wc -l < "$MDSL_FILE")"
echo "   SQL lines:  $(wc -l < "$SQL_FILE")"
echo "   Cypher lines: $(wc -l < "$CYPHER_FILE")"
```

### Python Integration Class
```python
from datetime import datetime
import subprocess
import json
import os

class MDSLToolchain:
    def __init__(self, project_path="/Users/mike/localsrc/ANMI-ML/mdsl-rs"):
        self.project_path = project_path
        self.sql_import_bin = os.path.join(project_path, "target/release/sql_import")
        self.mdsl_bin = os.path.join(project_path, "target/release/mdsl")
        self.test_runner_bin = os.path.join(project_path, "target/release/test_runner")
    
    def build_toolchain(self, features=["import", "sql-codegen", "cypher-codegen"]):
        """Build the MDSL toolchain with specified features"""
        feature_str = ",".join(features)
        cmd = ["cargo", "build", "--features", feature_str, "--release"]
        
        result = subprocess.run(cmd, cwd=self.project_path, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise Exception(f"Build failed: {result.stderr}")
        
        return {"status": "success", "features": features}
    
    def import_from_database(self, connection_string, output_file=None):
        """Import MDSL from PostgreSQL database"""
        if output_file is None:
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            output_file = f"anmi_imported_{timestamp}.mdsl"
        
        cmd = [self.sql_import_bin, "generate", "--connection", connection_string, "--output", output_file]
        
        result = subprocess.run(cmd, cwd=self.project_path, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise Exception(f"Import failed: {result.stderr}")
        
        return {"mdsl_file": output_file, "status": "success"}
    
    def validate_mdsl(self, mdsl_file):
        """Validate MDSL file syntax"""
        cmd = [self.mdsl_bin, "validate", mdsl_file]
        
        result = subprocess.run(cmd, cwd=self.project_path, capture_output=True, text=True)
        
        return {
            "valid": result.returncode == 0,
            "output": result.stdout,
            "errors": result.stderr if result.returncode != 0 else None
        }
    
    def generate_sql(self, mdsl_file, output_file=None):
        """Generate SQL from MDSL file"""
        cmd = [self.mdsl_bin, "sql", mdsl_file]
        
        result = subprocess.run(cmd, cwd=self.project_path, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise Exception(f"SQL generation failed: {result.stderr}")
        
        if output_file:
            with open(os.path.join(self.project_path, output_file), 'w') as f:
                f.write(result.stdout)
            return {"sql_file": output_file, "status": "success"}
        
        return {"sql_content": result.stdout, "status": "success"}
    
    def generate_cypher(self, mdsl_file, output_file=None):
        """Generate Cypher from MDSL file"""
        cmd = [self.mdsl_bin, "cypher", mdsl_file]
        
        result = subprocess.run(cmd, cwd=self.project_path, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise Exception(f"Cypher generation failed: {result.stderr}")
        
        if output_file:
            with open(os.path.join(self.project_path, output_file), 'w') as f:
                f.write(result.stdout)
            return {"cypher_file": output_file, "status": "success"}
        
        return {"cypher_content": result.stdout, "status": "success"}
    
    def run_tests(self, mdsl_file=None, verbose=False):
        """Run MDSL test suite"""
        cmd = [self.test_runner_bin]
        
        if mdsl_file:
            cmd.extend(["--file", mdsl_file])
        else:
            cmd.append("--all")
        
        if verbose:
            cmd.append("--verbose")
        
        result = subprocess.run(cmd, cwd=self.project_path, capture_output=True, text=True)
        
        return {
            "passed": result.returncode == 0,
            "output": result.stdout,
            "errors": result.stderr if result.returncode != 0 else None
        }
    
    def full_refresh_workflow(self, connection_string):
        """Complete database refresh workflow"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        try:
            # Step 1: Build toolchain
            build_result = self.build_toolchain()
            
            # Step 2: Import from database
            import_result = self.import_from_database(connection_string)
            mdsl_file = import_result["mdsl_file"]
            
            # Step 3: Validate MDSL
            validation_result = self.validate_mdsl(mdsl_file)
            if not validation_result["valid"]:
                raise Exception(f"MDSL validation failed: {validation_result['errors']}")
            
            # Step 4: Generate SQL
            sql_file = f"anmi_generated_{timestamp}.sql"
            sql_result = self.generate_sql(mdsl_file, sql_file)
            
            # Step 5: Generate Cypher
            cypher_file = f"anmi_generated_{timestamp}.cypher"
            cypher_result = self.generate_cypher(mdsl_file, cypher_file)
            
            # Step 6: Run tests
            test_result = self.run_tests(mdsl_file)
            
            return {
                "status": "success",
                "timestamp": timestamp,
                "files": {
                    "mdsl": mdsl_file,
                    "sql": sql_file,
                    "cypher": cypher_file
                },
                "validation": validation_result,
                "tests": test_result
            }
            
        except Exception as e:
            return {
                "status": "error", 
                "error": str(e),
                "timestamp": timestamp
            }

# Usage example
if __name__ == "__main__":
    toolchain = MDSLToolchain()
    
    connection = "postgresql://postgres:PASSWORD@100.77.115.86:5432/cmc"
    result = toolchain.full_refresh_workflow(connection)
    
    print(json.dumps(result, indent=2))
```

## Troubleshooting

### Build Issues
```bash
# Clean rebuild
cargo clean
cargo build --features import,sql-codegen,cypher-codegen --release
```

### Database Connection Issues
- Verify PostgreSQL credentials
- Check network access to 100.77.115.86:5432
- Ensure database `cmc` exists

### Missing Features
- Ensure you build with `--features import` for database import
- Use `--features sql-codegen` for SQL generation
- Use `--features cypher-codegen` for Cypher generation

## Recent Fixes Applied

1. **Sector Vocabulary Generation**: Automatically extracts and generates sector vocabulary
2. **Numeric Sector Values**: Fixed sector values to be numbers (20) not strings ("20")
3. **DATA Block Parsing**: Complete parser implementation for market data
4. **100% Database Coverage**: All 657 outlets, 613 relationships, 1055 data records
5. **SQL/Cypher Generation**: Full toolchain working with proper schema mapping

## Expected Output

The generated MDSL file will be approximately 15,000+ lines containing:
- Complete Austrian media landscape
- All historical relationships and ownership data
- Market data and reach statistics
- Proper vocabularies and typing
- Ready for Neo4j graph database or SQL regeneration

This replaces the need for manual SQL database maintenance - the MDSL file becomes the single source of truth.

## Quick Reference: GUI Integration Commands

### One-Line Database Refresh
```bash
# Complete workflow in one command chain
cd /Users/mike/localsrc/ANMI-ML/mdsl-rs && \
./target/release/sql_import generate --connection "postgresql://postgres:PASSWORD@100.77.115.86:5432/cmc" --output fresh.mdsl && \
./target/release/mdsl validate fresh.mdsl && \
./target/release/mdsl sql fresh.mdsl > fresh_anmi.sql && \
./target/release/mdsl cypher fresh.mdsl > fresh_graph.cypher && \
echo "✅ Fresh databases generated: fresh_anmi.sql, fresh_graph.cypher"
```

### Key File Outputs
- **MDSL Source**: `~15,000+ lines` - Complete Austrian media landscape
- **ANMI SQL**: `~200+ lines` - PostgreSQL schema + data with graphv3 tables
- **Neo4j Cypher**: `~1000+ lines` - Graph database nodes and relationships

### Production Deployment Checklist
1. ✅ Build toolchain with all features
2. ✅ Generate MDSL from source database  
3. ✅ Validate MDSL syntax
4. ✅ Generate SQL/Cypher outputs
5. ✅ Run test suite validation
6. ✅ Backup existing databases
7. ✅ Deploy fresh databases
8. ✅ Verify data integrity

### Support
For issues with the GUI integration, check:
- `/Users/mike/localsrc/ANMI-ML/mdsl-rs/tests/bootstrap_tests.rs` - Reference implementations
- `/Users/mike/localsrc/ANMI-ML/mdsl-rs/README.md` - Project documentation
- `cargo test --all-features` - Comprehensive test validation