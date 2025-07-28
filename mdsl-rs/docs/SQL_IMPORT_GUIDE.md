# SQL to MDSL Import Guide

This guide explains how to use the SQL to MDSL import functionality to convert existing SQL database data into MediaLanguage DSL format.

## Overview

The SQL import tool can automatically detect ANMI-like database schemas and convert them into MDSL format, supporting both diachronic (temporal) and synchronic (contemporary) relationships.

## Installation

Build the import tool with the import feature enabled:

```bash
cd mdsl-rs
cargo build --features import --bin sql_import
```

## Quick Start

### 1. Test Database Connection

First, test your database connection:

```bash
./target/debug/sql_import test "postgresql://user:password@localhost/media_db"
```

### 2. Analyze Database Schema

Analyze the database structure to see what tables and patterns are detected:

```bash
./target/debug/sql_import analyze "postgresql://user:password@localhost/media_db"
```

### 3. Auto-Import

Let the tool automatically detect ANMI patterns and import:

```bash
./target/debug/sql_import auto "postgresql://user:password@localhost/media_db"
```

## Connection String Formats

### PostgreSQL

```
postgresql://username:password@hostname:port/database
postgres://username:password@hostname:port/database
```

### MySQL

```
mysql://username:password@hostname:port/database
```

### SQLite

```
sqlite:///path/to/database.db
/path/to/database.db
/path/to/database.sqlite
```

### SQL Server

```
sqlserver://username:password@hostname:port/database
mssql://username:password@hostname:port/database
```

## Auto-Detection Patterns

The import tool automatically detects these ANMI-like patterns:

### Media Outlet Tables

- Tables named `medienangebot`, `media_outlet`, or similar
- Tables with columns `id_mo`, `mo_title`
- Convert to MDSL `OUTLET` declarations

### Media Company Tables

- Tables named `medienunternehmen`, `media_company`, or similar
- Tables with columns `id_mu`, `mu_title`
- Convert to MDSL company structures

### Diachronic Relationship Tables

- Tables with names containing `diachrone`, `diachronic`, or `temporal`
- Tables with columns like `id_mo_predecessor`, `id_mo_successor`, `event_date`
- Convert to MDSL `DIACHRONIC_LINK` declarations

### Synchronous Relationship Tables

- Tables with names containing `synchrone`, `synchronous`, or `contemporary`
- Tables with columns like `id_mo_1`, `id_mo_2`, `start_year`, `end_year`
- Convert to MDSL `SYNCHRONOUS_LINK` declarations

## Generated MDSL Structure

The import tool generates complete MDSL files with:

### Standard Imports

```mdsl
IMPORT "anmi_common_codes.mdsl";
IMPORT "anmi_media_sectors.mdsl";
IMPORT "anmi_mandate_types.mdsl";
// ... other standard imports
```

### Families and Outlets

```mdsl
FAMILY "Auto-Generated Media Group" {
    @comment "Imported from SQL database on 2024-01-01"

    OUTLET "Example Newspaper" {
        id = 200001;
        identity {
            title = "Example Newspaper";
            url = "https://example.com";
        };
        lifecycle {
            status "active" FROM "1995-01-01" TO CURRENT {
                precision_start = "known";
                precision_end = "known";
            };
        };
        characteristics {
            sector = "Tageszeitung";
            mandate = "Privat-kommerziell";
            distribution = {
                primary_area = "Österreich gesamt";
                local = false;
            };
            language = "de";
        };
        metadata {
            steward = "imported";
            verified = "2024-01-01";
        };
    };
}
```

### Relationships

```mdsl
DIACHRONIC_LINK acquisition_example {
    predecessor = 300001;
    successor = 200001;
    event_date = "1971-01-01";
    relationship_type = "Akquisition";
    @comment "Imported relationship data";
    @maps_to "MedienangeboteDiachroneBeziehungen";
};

SYNCHRONOUS_LINK partnership_example {
    outlet_1 = {
        id = 200001;
        role = "primary";
    };
    outlet_2 = {
        id = 200002;
        role = "secondary";
    };
    relationship_type = "Kooperation";
    period = "2020-01-01" TO "2023-12-31";
    @maps_to "MedienangeboteSynchroneBeziehungen";
};
```

## Field Mapping

The import tool maps common SQL field patterns to MDSL structures:

### Media Outlet Mapping

| SQL Column                               | MDSL Location                               | Description         |
| ---------------------------------------- | ------------------------------------------- | ------------------- |
| `id_mo`                                  | `id`                                        | Primary identifier  |
| `mo_title`                               | `identity.title`                            | Outlet title        |
| `start_year`, `start_month`, `start_day` | `lifecycle.status.FROM`                     | Launch date         |
| `end_year`, `end_month`, `end_day`       | `lifecycle.status.TO`                       | End date            |
| `mandate`                                | `characteristics.mandate`                   | Business type       |
| `location`                               | `characteristics.editorial_office`          | Editorial location  |
| `language`                               | `characteristics.language`                  | Primary language    |
| `editorial_line_self_descr`              | `characteristics.editorial_stance.self`     | Self-description    |
| `editorial_line_external_attr`           | `characteristics.editorial_stance.external` | External assessment |
| `comments`                               | `metadata.notes`                            | Additional notes    |

### Relationship Mapping

| SQL Column                 | MDSL Field          | Description                |
| -------------------------- | ------------------- | -------------------------- |
| `id_mo_predecessor`        | `predecessor`       | Source outlet ID           |
| `id_mo_successor`          | `successor`         | Target outlet ID           |
| `relationship_type`        | `relationship_type` | Type of relationship       |
| `event_date`, `event_year` | `event_date`        | When relationship occurred |
| `start_year`               | `period.FROM`       | Relationship start         |
| `end_year`                 | `period.TO`         | Relationship end           |

## Manual Configuration

For databases that don't follow ANMI patterns, you can create custom mapping configurations:

```toml
# mappings.toml
[[table]]
name = "custom_outlets"
mdsl_type = "Outlet"

[table.fields]
outlet_id = { mdsl_field = "id", type = "Id" }
outlet_name = { mdsl_field = "identity.title", type = "Text" }
launch_date = { mdsl_field = "lifecycle.start", type = "Date" }
sector_type = { mdsl_field = "characteristics.sector", type = "Text" }

[[table]]
name = "custom_relationships"
mdsl_type = "DiachronicLink"

[table.fields]
source_id = { mdsl_field = "predecessor", type = "Number" }
target_id = { mdsl_field = "successor", type = "Number" }
relation_type = { mdsl_field = "relationship_type", type = "Text" }
```

Then use it with:

```bash
./target/debug/sql_import manual "postgresql://user:pass@localhost/db" mappings.toml
```

## Database-Specific Notes

### PostgreSQL

- Supports schema names in connection string
- Excellent foreign key detection
- Handles complex data types well

### MySQL

- Case-sensitive table/column names on Linux
- Good performance for large datasets
- Supports multiple character sets

### SQLite

- File-based, no network connection needed
- Limited concurrent access
- Good for testing and small datasets

### SQL Server

- Windows Authentication supported
- Schema detection may require additional permissions
- Good enterprise feature support

## Troubleshooting

### Connection Issues

```bash
# Test with minimal connection
./target/debug/sql_import test "postgresql://user@localhost/db"

# Check if port is accessible
telnet hostname 5432
```

### Schema Detection Issues

```bash
# Analyze schema first
./target/debug/sql_import analyze "your_connection_string"

# Look for ANMI pattern detection results
```

### Import Errors

- Check database permissions (SELECT required on all tables)
- Verify foreign key constraints are properly defined
- Ensure date fields use standard SQL date formats

### Performance

- For large databases, consider filtering tables first
- Use database views to simplify complex schemas
- Index foreign key columns for better performance

## Examples

### Complete Austrian Media Database Import

```bash
# 1. Test connection
./target/debug/sql_import test "postgresql://anmi_user:password@db.example.com/anmi_production"

# 2. Analyze schema
./target/debug/sql_import analyze "postgresql://anmi_user:password@db.example.com/anmi_production"

# 3. Import with auto-detection
./target/debug/sql_import auto "postgresql://anmi_user:password@db.example.com/anmi_production" > imported_media_data.mdsl
```

### SQLite Development Database

```bash
# Local testing with SQLite
./target/debug/sql_import auto "sqlite:///tmp/media_test.db" > test_import.mdsl
```

### Custom Schema Import

```bash
# Using custom mappings for non-ANMI database
./target/debug/sql_import manual "mysql://user:pass@localhost/custom_media" custom_mappings.toml > custom_import.mdsl
```

## Integration with MDSL Workflow

After importing, you can use the generated MDSL with existing tools:

```bash
# Parse and validate imported MDSL
cargo run --features sql-codegen -- parse imported_media_data.mdsl

# Generate SQL schema from imported data
cargo run --features sql-codegen -- sql imported_media_data.mdsl

# Generate Neo4j Cypher from imported data
cargo run --features cypher-codegen -- cypher imported_media_data.mdsl
```

## Advanced Features

### Incremental Imports

- TODO: Support for incremental updates
- TODO: Change detection and delta imports

### Data Validation

- TODO: Validate imported data against MDSL schema
- TODO: Generate validation reports

### Custom Transformations

- TODO: Support for custom field transformation functions
- TODO: Lookup table resolution during import

## Contributing

To extend the import functionality:

1. Add new database drivers in `src/import/connection.rs`
2. Extend pattern detection in `src/import/mapper.rs`
3. Add custom field transformations in `src/import/mapper.rs`
4. Update CLI commands in `src/bin/sql_import.rs`

See the development documentation for more details on the import architecture.
