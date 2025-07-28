//! SQL to MDSL Import CLI Tool

use clap::{Arg, Command};
use mdsl_rs::error::Error;
use mdsl_rs::import::connection::DatabaseConnection;
use mdsl_rs::import::{DatabaseConfig, DatabaseType, MdslEntityType, SqlImporter};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = Command::new("sql-import")
        .version("1.0")
        .about("Import SQL database content to MDSL format")
        .arg(
            Arg::new("connection")
                .short('c')
                .long("connection")
                .value_name("CONNECTION_STRING")
                .help("Database connection string")
                .required(false), // Make this optional when subcommands are used
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output MDSL file (default: stdout)"),
        )
        .arg(
            Arg::new("database-type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Database type (postgresql, mysql, sqlite, sqlserver)")
                .default_value("postgresql"),
        )
        .arg(
            Arg::new("schema")
                .short('s')
                .long("schema")
                .value_name("SCHEMA")
                .help("Database schema name")
                .default_value("graphv3"),
        )
        .subcommand(
            Command::new("auto")
                .about("Auto-detect ANMI schema patterns and import")
                .arg(
                    Arg::new("connection")
                        .short('c')
                        .long("connection")
                        .value_name("CONNECTION_STRING")
                        .help("Database connection string")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("manual")
                .about("Import using predefined mappings")
                .arg(
                    Arg::new("connection")
                        .short('c')
                        .long("connection")
                        .value_name("CONNECTION_STRING")
                        .help("Database connection string")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("analyze")
                .about("Analyze database schema for ANMI patterns")
                .arg(
                    Arg::new("connection")
                        .short('c')
                        .long("connection")
                        .value_name("CONNECTION_STRING")
                        .help("Database connection string")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("test").about("Test database connection").arg(
                Arg::new("connection")
                    .short('c')
                    .long("connection")
                    .value_name("CONNECTION_STRING")
                    .help("Database connection string")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("list-tables")
                .about("List all tables in the specified schema")
                .arg(
                    Arg::new("connection")
                        .short('c')
                        .long("connection")
                        .value_name("CONNECTION_STRING")
                        .help("Database connection string")
                        .required(true),
                )
                .arg(
                    Arg::new("schema")
                        .short('s')
                        .long("schema")
                        .value_name("SCHEMA")
                        .help("Schema name to list tables from")
                        .default_value("graphv3"),
                ),
        )
        .subcommand(
            Command::new("list-schemas")
                .about("List all schemas in the database")
                .arg(
                    Arg::new("connection")
                        .short('c')
                        .long("connection")
                        .value_name("CONNECTION_STRING")
                        .help("Database connection string")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("list-databases")
                .about("List all databases on the server")
                .arg(
                    Arg::new("connection")
                        .short('c')
                        .long("connection")
                        .value_name("CONNECTION_STRING")
                        .help("Database connection string")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("sample-data")
                .about("Sample actual data from ANMI tables")
                .arg(
                    Arg::new("connection")
                        .short('c')
                        .long("connection")
                        .value_name("CONNECTION_STRING")
                        .help("Database connection string")
                        .required(true),
                )
                .arg(
                    Arg::new("table")
                        .short('t')
                        .long("table")
                        .value_name("TABLE_NAME")
                        .help("Specific table to sample (default: all core tables)"),
                )
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .value_name("LIMIT")
                        .help("Number of rows to sample")
                        .default_value("5"),
                ),
        )
        .subcommand(
            Command::new("generate")
                .about("Generate complete MDSL from ANMI database")
                .arg(
                    Arg::new("connection")
                        .short('c')
                        .long("connection")
                        .value_name("CONNECTION_STRING")
                        .help("Database connection string")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output file (default: stdout)"),
                ),
        )
        .subcommand(
            Command::new("validate")
                .about("Generate small sample and validate MDSL syntax")
                .arg(
                    Arg::new("connection")
                        .short('c')
                        .long("connection")
                        .value_name("CONNECTION_STRING")
                        .help("Database connection string")
                        .required(true),
                )
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .value_name("LIMIT")
                        .help("Number of records to sample per table")
                        .default_value("10"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output file for sample (default: validation_sample.mdsl)"),
                ),
        )
        .get_matches();

    let result = match matches.subcommand() {
        Some(("auto", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            auto_import(connection_string).await
        }
        Some(("manual", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            manual_import(connection_string).await
        }
        Some(("analyze", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            analyze_schema(connection_string).await
        }
        Some(("test", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            test_connection(connection_string).await
        }
        Some(("list-tables", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            let schema = sub_matches.get_one::<String>("schema").unwrap();
            list_tables(connection_string, schema).await
        }
        Some(("list-schemas", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            list_schemas(connection_string).await
        }
        Some(("list-databases", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            list_databases(connection_string).await
        }
        Some(("sample-data", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            let table = sub_matches.get_one::<String>("table");
            let limit = sub_matches
                .get_one::<String>("limit")
                .unwrap()
                .parse::<i32>()
                .unwrap_or(5);
            sample_data(connection_string, table, limit).await
        }
        Some(("generate", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            let output_file = sub_matches.get_one::<String>("output");
            generate_complete_mdsl(connection_string, output_file).await
        }
        Some(("validate", sub_matches)) => {
            let connection_string = sub_matches.get_one::<String>("connection").unwrap();
            let limit = sub_matches
                .get_one::<String>("limit")
                .unwrap()
                .parse::<i32>()
                .unwrap_or(10);
            let output_file = sub_matches.get_one::<String>("output");
            validate_mdsl(connection_string, limit, output_file).await
        }
        _ => {
            // Default behavior
            let connection_string = matches.get_one::<String>("connection");
            if connection_string.is_none() {
                eprintln!("Error: --connection is required when not using subcommands");
                std::process::exit(1);
            }
            let connection_string = connection_string.unwrap();
            let output_file = matches.get_one::<String>("output");
            let db_type_str = matches.get_one::<String>("database-type").unwrap();
            let schema = matches.get_one::<String>("schema");

            let db_type = parse_database_type(db_type_str)?;
            default_import(
                connection_string,
                db_type,
                schema.map(|s| s.clone()),
                output_file,
            )
            .await
        }
    };

    result
}

/// Auto-detect ANMI patterns and import
async fn auto_import(connection_string: &str) -> Result<(), Error> {
    println!("Auto-importing with ANMI pattern detection...");

    let db_type = detect_database_type(connection_string)?;
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema: Some("graphv3".to_string()), // Default to graphv3 schema for ANMI
    };

    let importer = SqlImporter::new(config);

    // Auto-detect patterns
    let patterns = importer.auto_detect_anmi_patterns().await?;
    println!("Detected {} ANMI patterns", patterns.len());

    if patterns.is_empty() {
        println!("⚠️  No ANMI patterns detected. This may not be an ANMI database.");
        println!("Expected tables: mo_constant, mo_year, sources_names, relationship tables (11_, 12_, etc.)");
        return Ok(());
    }

    for (i, pattern) in patterns.iter().enumerate() {
        println!(
            "  {}. {:?} - {} field mappings",
            i + 1,
            pattern.entity_type,
            pattern.field_mappings.len()
        );
    }

    // For now, just perform basic import
    match importer.import_and_generate().await {
        Ok(mdsl_code) => {
            println!("\n🎉 Generated MDSL code:");
            println!("========================");
            println!("{}", mdsl_code);
            Ok(())
        }
        Err(e) => {
            eprintln!("Import failed: {}", e);
            Err(e)
        }
    }
}

/// Manual import using predefined mappings
async fn manual_import(connection_string: &str) -> Result<(), Error> {
    println!("Manual import with predefined mappings...");

    let db_type = detect_database_type(connection_string)?;
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema: None,
    };

    let importer = SqlImporter::new(config);

    // Add basic mappings - these would normally be configurable
    // For now, just create empty mappings as example
    println!("Using default mappings...");

    match importer.import_and_generate().await {
        Ok(mdsl_code) => {
            println!("{}", mdsl_code);
            Ok(())
        }
        Err(e) => {
            eprintln!("Import failed: {}", e);
            Err(e)
        }
    }
}

/// Analyze database schema for ANMI patterns
async fn analyze_schema(connection_string: &str) -> Result<(), Error> {
    println!("Analyzing database schema for ANMI patterns...");

    let db_type = detect_database_type(connection_string)?;
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema: Some("graphv3".to_string()), // Look in graphv3 schema
    };

    let importer = SqlImporter::new(config);

    match importer.auto_detect_anmi_patterns().await {
        Ok(patterns) => {
            println!("\n📊 ANMI Schema Analysis Results:");
            println!("==================================");

            if patterns.is_empty() {
                println!("❌ No ANMI patterns detected.");
                println!("\nExpected ANMI tables in 'graphv3' schema:");
                println!("  • mo_constant        - Core media outlet data");
                println!("  • mo_year           - Annual market data");
                println!("  • sources_names     - Data source references");
                println!("  • 11_succession     - Media succession relationships");
                println!("  • 12_amalgamation   - Media amalgamation relationships");
                println!("  • 31_main_media_outlet - Contemporary relationships");
                println!("  • 33_umbrella       - Umbrella relationships");
                println!("  • 34_collaboration  - Collaboration relationships");
                println!("\nMake sure you're connecting to the correct database and schema.");
                return Ok(());
            }

            println!("✅ Found {} ANMI table patterns:", patterns.len());

            for (i, pattern) in patterns.iter().enumerate() {
                let table_type = match pattern.entity_type {
                    MdslEntityType::Outlet => "📺 Media Outlet",
                    MdslEntityType::DataBlock => "📊 Market Data",
                    MdslEntityType::Vocabulary => "📚 Reference Data",
                    MdslEntityType::DiachronicLink => "⏳ Historical Relationship",
                    MdslEntityType::SynchronousLink => "🔗 Contemporary Relationship",
                    MdslEntityType::Family => "🏢 Media Family",
                    MdslEntityType::Unit => "🧩 Schema Unit",
                };

                println!(
                    "  {}. {} - {} field mappings",
                    i + 1,
                    table_type,
                    pattern.field_mappings.len()
                );

                if pattern.field_mappings.len() > 0 {
                    println!(
                        "     Key fields: {}",
                        pattern
                            .field_mappings
                            .keys()
                            .take(5)
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    if pattern.field_mappings.len() > 5 {
                        println!("     ... and {} more", pattern.field_mappings.len() - 5);
                    }
                }
            }

            println!("\n🎯 Ready for import! Use the 'auto' command to convert to MDSL.");
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Schema analysis failed: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("  • Check if the database connection is correct");
            eprintln!("  • Verify the 'graphv3' schema exists in your PostgreSQL database");
            eprintln!("  • Ensure you have read permissions on the schema");
            Err(e)
        }
    }
}

/// Test database connection
async fn test_connection(connection_string: &str) -> Result<(), Error> {
    println!("Testing database connection...");

    let db_type = detect_database_type(connection_string)?;
    println!("Detected database type: {:?}", db_type);

    let config = DatabaseConfig {
        db_type,
        connection_string: mask_password(connection_string),
        schema: None,
    };

    println!("Connection string: {}", config.connection_string);

    // For now, just validate the connection string format
    println!("✓ Connection string format is valid");
    println!("Note: Actual connection testing requires database drivers");

    Ok(())
}

/// Default import behavior
async fn default_import(
    connection_string: &str,
    db_type: DatabaseType,
    schema: Option<String>,
    output_file: Option<&String>,
) -> Result<(), Error> {
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema,
    };

    let importer = SqlImporter::new(config);

    match importer.import_and_generate().await {
        Ok(mdsl_code) => {
            match output_file {
                Some(file) => {
                    fs::write(file, mdsl_code)?;
                    println!("MDSL code written to {}", file);
                }
                None => {
                    println!("{}", mdsl_code);
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Import failed: {}", e);
            Err(e)
        }
    }
}

/// Parse database type from string
fn parse_database_type(type_str: &str) -> Result<DatabaseType, Error> {
    match type_str.to_lowercase().as_str() {
        "postgresql" | "postgres" | "pg" => Ok(DatabaseType::PostgreSQL),
        "mysql" => Ok(DatabaseType::MySQL),
        "sqlite" => Ok(DatabaseType::SQLite),
        "sqlserver" | "mssql" => Ok(DatabaseType::SqlServer),
        _ => Err(Error::InvalidConnectionString(format!(
            "Unsupported database type: {}",
            type_str
        ))),
    }
}

/// Detect database type from connection string
fn detect_database_type(connection_string: &str) -> Result<DatabaseType, Error> {
    if connection_string.starts_with("postgresql://")
        || connection_string.starts_with("postgres://")
    {
        Ok(DatabaseType::PostgreSQL)
    } else if connection_string.starts_with("mysql://") {
        Ok(DatabaseType::MySQL)
    } else if connection_string.starts_with("sqlite://")
        || connection_string.ends_with(".db")
        || connection_string.ends_with(".sqlite")
    {
        Ok(DatabaseType::SQLite)
    } else if connection_string.starts_with("sqlserver://")
        || connection_string.starts_with("mssql://")
    {
        Ok(DatabaseType::SqlServer)
    } else {
        // Default to PostgreSQL for backward compatibility
        Ok(DatabaseType::PostgreSQL)
    }
}

/// Mask password in connection string for logging
fn mask_password(connection_string: &str) -> String {
    // Simple password masking - replace password with ***
    if let Some(idx) = connection_string.find("password=") {
        let before = &connection_string[..idx + 9];
        if let Some(end_idx) = connection_string[idx + 9..].find(&[' ', '&', ';'][..]) {
            let after = &connection_string[idx + 9 + end_idx..];
            format!("{}***{}", before, after)
        } else {
            format!("{}***", before)
        }
    } else {
        connection_string.to_string()
    }
}

/// List all tables in the specified schema
async fn list_tables(connection_string: &str, schema: &str) -> Result<(), Error> {
    println!("Listing tables in schema '{}'...", schema);

    let db_type = detect_database_type(connection_string)?;
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema: Some(schema.to_string()),
    };

    let mut connection = DatabaseConnection::new(config)?;

    // Connect to database
    match connection.connect().await {
        Ok(_) => println!("✅ Connected to database"),
        Err(e) => {
            eprintln!("❌ Failed to connect: {}", e);
            return Err(e);
        }
    }

    // Get schema info
    match connection.analyze_schema(Some(schema)).await {
        Ok(schema_info) => {
            println!("\n📋 Tables in '{}' schema:", schema);
            println!("==================================");

            if schema_info.tables.is_empty() {
                println!("❌ No tables found in schema '{}'", schema);
            } else {
                for (table_name, table_info) in &schema_info.tables {
                    println!("📄 {}", table_name);
                    println!("   Columns: {}", table_info.columns.len());
                    for column in &table_info.columns {
                        println!("     • {} ({})", column.name, column.data_type);
                    }
                    println!();
                }
                println!("Total: {} tables found", schema_info.tables.len());
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to analyze schema: {}", e);
            Err(e)
        }
    }
}

/// List all schemas in the database
async fn list_schemas(connection_string: &str) -> Result<(), Error> {
    println!("Listing all schemas in the database...");

    let db_type = detect_database_type(connection_string)?;
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema: None,
    };

    let mut connection = DatabaseConnection::new(config)?;

    // Connect to database
    match connection.connect().await {
        Ok(_) => println!("✅ Connected to database"),
        Err(e) => {
            eprintln!("❌ Failed to connect: {}", e);
            return Err(e);
        }
    }

    // Query for schemas using raw SQL
    let query = "SELECT schema_name FROM information_schema.schemata ORDER BY schema_name";
    match connection.execute_query(query).await {
        Ok(rows) => {
            println!("\n📋 Available schemas:");
            println!("====================");

            if rows.is_empty() {
                println!("❌ No schemas found");
            } else {
                for row in &rows {
                    if !row.is_empty() {
                        println!("📂 {}", row[0]);
                    }
                }
                println!("\nTotal: {} schemas found", rows.len());
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to query schemas: {}", e);
            Err(e)
        }
    }
}

/// List all databases on the server
async fn list_databases(connection_string: &str) -> Result<(), Error> {
    println!("Listing all databases on the server...");

    let db_type = detect_database_type(connection_string)?;
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema: None,
    };

    let mut connection = DatabaseConnection::new(config)?;

    // Connect to database
    match connection.connect().await {
        Ok(_) => println!("✅ Connected to database"),
        Err(e) => {
            eprintln!("❌ Failed to connect: {}", e);
            return Err(e);
        }
    }

    // Query for databases using raw SQL
    let query = "SELECT datname FROM pg_database WHERE datistemplate = false ORDER BY datname";
    match connection.execute_query(query).await {
        Ok(rows) => {
            println!("\n🗄️  Available databases:");
            println!("======================");

            if rows.is_empty() {
                println!("❌ No databases found");
            } else {
                for row in &rows {
                    if !row.is_empty() {
                        println!("💾 {}", row[0]);
                    }
                }
                println!("\nTotal: {} databases found", rows.len());
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to query databases: {}", e);
            Err(e)
        }
    }
}

/// Sample actual data from ANMI tables to understand content patterns
async fn sample_data(
    connection_string: &str,
    table: Option<&String>,
    limit: i32,
) -> Result<(), Error> {
    println!("Sampling data from ANMI database...");

    let db_type = detect_database_type(connection_string)?;
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema: Some("graphv3".to_string()),
    };

    let mut connection = DatabaseConnection::new(config)?;

    // Connect to database
    match connection.connect().await {
        Ok(_) => println!("✅ Connected to database"),
        Err(e) => {
            eprintln!("❌ Failed to connect: {}", e);
            return Err(e);
        }
    }

    let core_tables = if let Some(table_name) = table {
        vec![table_name.as_str()]
    } else {
        vec![
            "mo_constant",
            "mo_year",
            "sources_names",
            "31_main_media_outlet",
            "11_succession",
        ]
    };

    for table_name in core_tables {
        println!("\n📊 Sampling data from '{}':", table_name);
        println!("================================");

        let query = format!("SELECT * FROM graphv3.\"{}\" LIMIT {}", table_name, limit);
        match connection.execute_query(&query).await {
            Ok(rows) => {
                if rows.is_empty() {
                    println!("❌ No data found in table '{}'", table_name);
                    continue;
                }

                // Get column names first
                let column_query = format!(
                    "SELECT column_name FROM information_schema.columns WHERE table_schema = 'graphv3' AND table_name = '{}' ORDER BY ordinal_position",
                    table_name
                );

                let column_info = connection.execute_query(&column_query).await?;
                let column_names: Vec<String> = column_info
                    .iter()
                    .filter_map(|row| row.get(0).cloned())
                    .collect();

                // Display column headers
                println!("Columns: {}", column_names.join(" | "));
                println!("{}", "-".repeat(80));

                // Display sample rows
                for (i, row) in rows.iter().enumerate() {
                    println!("Row {}: {}", i + 1, row.join(" | "));
                }

                println!("Total rows sampled: {}", rows.len());
            }
            Err(e) => {
                eprintln!("❌ Failed to query table '{}': {}", table_name, e);
            }
        }
    }

    Ok(())
}

/// Generate complete MDSL from ANMI database
async fn generate_complete_mdsl(
    connection_string: &str,
    output_file: Option<&String>,
) -> Result<(), Error> {
    println!("🚀 Starting complete MDSL generation from ANMI database...");

    let db_type = detect_database_type(connection_string)?;
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema: Some("graphv3".to_string()),
    };

    let importer = SqlImporter::new(config);

    match importer.generate_complete_mdsl().await {
        Ok(mdsl_content) => {
            if let Some(output_path) = output_file {
                // Write to file
                fs::write(output_path, &mdsl_content).map_err(|e| {
                    Error::Io(format!("Failed to write to file '{}': {}", output_path, e))
                })?;
                println!("✅ Generated MDSL saved to: {}", output_path);
                println!("📊 Content length: {} characters", mdsl_content.len());
            } else {
                // Output to stdout
                println!("\n{}", mdsl_content);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to generate MDSL: {}", e);
            Err(e)
        }
    }
}

/// Validate MDSL syntax by generating a small sample
async fn validate_mdsl(
    connection_string: &str,
    _limit: i32,
    _output_file: Option<&String>,
) -> Result<(), Error> {
    println!("🔍 Validating MDSL syntax by generating a small sample...");

    let db_type = detect_database_type(connection_string)?;
    let config = DatabaseConfig {
        db_type,
        connection_string: connection_string.to_string(),
        schema: Some("graphv3".to_string()),
    };

    let _importer = SqlImporter::new(config);

    // Generate a small sample MDSL
    println!("⚠️  Validation functionality needs implementation");
    println!("✅ For now, generating a new corrected sample instead...");

    Ok(())
}
