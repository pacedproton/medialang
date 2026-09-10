//! Database connection and schema analysis

use crate::error::{Error, Result};
use crate::import::{DatabaseConfig, DatabaseType};
use std::collections::HashMap;

#[cfg(feature = "tokio-postgres")]
use tokio_postgres::{Client, NoTls};

#[cfg(feature = "import")]
use rust_decimal;

/// Database connection abstraction
pub struct DatabaseConnection {
    config: DatabaseConfig,
    connection_string: String,
    #[cfg(feature = "tokio-postgres")]
    postgres_client: Option<Client>,
}

/// Database schema information
#[derive(Debug, Clone)]
pub struct SchemaInfo {
    /// Tables in the database
    pub tables: HashMap<String, TableInfo>,
    /// Database metadata
    pub metadata: DatabaseMetadata,
}

/// Table information
#[derive(Debug, Clone)]
pub struct TableInfo {
    /// Table name
    pub name: String,
    /// Table columns
    pub columns: Vec<ColumnInfo>,
    /// Primary key columns
    pub primary_keys: Vec<String>,
    /// Foreign key relationships
    pub foreign_keys: Vec<ForeignKeyInfo>,
    /// Indexes
    pub indexes: Vec<IndexInfo>,
}

/// Column information
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    /// Column name
    pub name: String,
    /// SQL data type
    pub data_type: String,
    /// Whether column is nullable
    pub nullable: bool,
    /// Default value if any
    pub default_value: Option<String>,
    /// Max length for string types
    pub max_length: Option<usize>,
}

/// Foreign key relationship information
#[derive(Debug, Clone)]
pub struct ForeignKeyInfo {
    /// Source column
    pub source_column: String,
    /// Referenced table
    pub target_table: String,
    /// Referenced column
    pub target_column: String,
    /// Constraint name
    pub constraint_name: String,
}

/// Index information
#[derive(Debug, Clone)]
pub struct IndexInfo {
    /// Index name
    pub name: String,
    /// Indexed columns
    pub columns: Vec<String>,
    /// Whether index is unique
    pub unique: bool,
}

/// Database metadata
#[derive(Debug, Clone)]
pub struct DatabaseMetadata {
    /// Database name
    pub name: String,
    /// Database version
    pub version: String,
    /// Schema name (if applicable)
    pub schema: Option<String>,
}

/// Database query result
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Column names
    pub columns: Vec<String>,
    /// Rows of data
    pub rows: Vec<Vec<Option<String>>>,
}

impl DatabaseConnection {
    /// Create a new database connection instance
    pub fn new(config: DatabaseConfig) -> Result<Self> {
        let connection_string = config.connection_string.clone();

        Ok(DatabaseConnection {
            config,
            connection_string,
            #[cfg(feature = "tokio-postgres")]
            postgres_client: None,
        })
    }

    /// Connect to the database
    #[cfg(feature = "tokio-postgres")]
    pub async fn connect(&mut self) -> Result<()> {
        match self.config.db_type {
            DatabaseType::PostgreSQL => {
                let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls)
                    .await
                    .map_err(|e| Error::DatabaseConnection(e.to_string()))?;

                // The connection object must be spawned in a background task
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("PostgreSQL connection error: {}", e);
                    }
                });

                self.postgres_client = Some(client);
                Ok(())
            }
            _ => Err(Error::NotImplemented(format!(
                "{:?} connections not yet implemented",
                self.config.db_type
            ))),
        }
    }

    #[cfg(not(feature = "tokio-postgres"))]
    pub async fn connect(&mut self) -> Result<()> {
        Err(Error::NotImplemented(
            "Database connections require tokio-postgres feature".to_string(),
        ))
    }

    /// Get list of tables in the specified schema
    #[cfg(feature = "tokio-postgres")]
    pub async fn get_tables(&self, schema: Option<&str>) -> Result<Vec<String>> {
        if let Some(client) = &self.postgres_client {
            match self.config.db_type {
                DatabaseType::PostgreSQL => {
                    let schema_name = schema.unwrap_or("public");
                    let query = "SELECT table_name FROM information_schema.tables WHERE table_schema = $1 ORDER BY table_name";

                    let rows = client
                        .query(query, &[&schema_name])
                        .await
                        .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

                    let tables: Vec<String> =
                        rows.iter().map(|row| row.get::<_, String>(0)).collect();

                    Ok(tables)
                }
                _ => Err(Error::NotImplemented(format!(
                    "{:?} table queries not yet implemented",
                    self.config.db_type
                ))),
            }
        } else {
            Err(Error::DatabaseConnection(
                "Not connected to database".to_string(),
            ))
        }
    }

    #[cfg(not(feature = "tokio-postgres"))]
    pub async fn get_tables(&self, _schema: Option<&str>) -> Result<Vec<String>> {
        Err(Error::NotImplemented(
            "Database queries require tokio-postgres feature".to_string(),
        ))
    }

    /// Get column information for a table
    #[cfg(feature = "tokio-postgres")]
    pub async fn get_columns(
        &self,
        table_name: &str,
        schema: Option<&str>,
    ) -> Result<Vec<(String, String)>> {
        if let Some(client) = &self.postgres_client {
            match self.config.db_type {
                DatabaseType::PostgreSQL => {
                    let schema_name = schema.unwrap_or("public");
                    let query = "SELECT column_name, data_type FROM information_schema.columns WHERE table_schema = $1 AND table_name = $2 ORDER BY ordinal_position";

                    let rows = client
                        .query(query, &[&schema_name, &table_name])
                        .await
                        .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

                    let columns: Vec<(String, String)> = rows
                        .iter()
                        .map(|row| (row.get::<_, String>(0), row.get::<_, String>(1)))
                        .collect();

                    Ok(columns)
                }
                _ => Err(Error::NotImplemented(format!(
                    "{:?} column queries not yet implemented",
                    self.config.db_type
                ))),
            }
        } else {
            Err(Error::DatabaseConnection(
                "Not connected to database".to_string(),
            ))
        }
    }

    #[cfg(not(feature = "tokio-postgres"))]
    pub async fn get_columns(
        &self,
        _table_name: &str,
        _schema: Option<&str>,
    ) -> Result<Vec<(String, String)>> {
        Err(Error::NotImplemented(
            "Database queries require tokio-postgres feature".to_string(),
        ))
    }

    /// Query data from a table
    #[cfg(feature = "tokio-postgres")]
    pub async fn query_table_data(
        &self,
        table_name: &str,
        schema: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<std::collections::HashMap<String, String>>> {
        if let Some(client) = &self.postgres_client {
            match self.config.db_type {
                DatabaseType::PostgreSQL => {
                    let schema_name = schema.unwrap_or("public");
                    let full_table_name = format!("{}.{}", schema_name, table_name);

                    let query = if let Some(limit) = limit {
                        format!("SELECT * FROM {} LIMIT {}", full_table_name, limit)
                    } else {
                        format!("SELECT * FROM {}", full_table_name)
                    };

                    let rows = client
                        .query(&query, &[])
                        .await
                        .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

                    if rows.is_empty() {
                        return Ok(vec![]);
                    }

                    let column_names: Vec<String> = rows[0]
                        .columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect();
                    let mut result = Vec::new();

                    for row in rows {
                        let mut record = std::collections::HashMap::new();
                        for (i, column_name) in column_names.iter().enumerate() {
                            let value: Option<String> = match row.try_get(i) {
                                Ok(Some(v)) => Some(v),
                                Ok(None) => None,
                                Err(_) => {
                                    // Try different types
                                    match row.try_get::<_, Option<i32>>(i) {
                                        Ok(Some(v)) => Some(v.to_string()),
                                        Ok(None) => None,
                                        Err(_) => match row.try_get::<_, Option<i64>>(i) {
                                            Ok(Some(v)) => Some(v.to_string()),
                                            Ok(None) => None,
                                            Err(_) => Some("<unparseable>".to_string()),
                                        },
                                    }
                                }
                            };
                            record.insert(
                                column_name.clone(),
                                value.unwrap_or_else(|| "NULL".to_string()),
                            );
                        }
                        result.push(record);
                    }

                    Ok(result)
                }
                _ => Err(Error::NotImplemented(format!(
                    "{:?} data queries not yet implemented",
                    self.config.db_type
                ))),
            }
        } else {
            Err(Error::DatabaseConnection(
                "Not connected to database".to_string(),
            ))
        }
    }

    #[cfg(not(feature = "tokio-postgres"))]
    pub async fn query_table_data(
        &self,
        _table_name: &str,
        _schema: Option<&str>,
        _limit: Option<usize>,
    ) -> Result<Vec<std::collections::HashMap<String, String>>> {
        Err(Error::NotImplemented(
            "Database queries require tokio-postgres feature".to_string(),
        ))
    }

    /// Analyze database schema
    pub async fn analyze_schema(&self, schema: Option<&str>) -> Result<SchemaInfo> {
        let tables = self.get_tables(schema).await?;
        let metadata = self.get_database_metadata().await?;

        let mut table_info_map = HashMap::new();

        for table_name in tables {
            let columns = self.get_columns(&table_name, schema).await?;
            let table_info = TableInfo {
                name: table_name.clone(),
                columns: columns
                    .into_iter()
                    .map(|(name, data_type)| ColumnInfo {
                        name,
                        data_type,
                        nullable: true, // We'd need a more detailed query to get this
                        default_value: None, // We'd need a more detailed query to get this
                        max_length: None, // We'd need a more detailed query to get this
                    })
                    .collect(),
                primary_keys: vec![], // We'd need a more detailed query to get this
                foreign_keys: vec![], // We'd need additional queries for this
                indexes: vec![],      // We'd need additional queries for this
            };
            table_info_map.insert(table_name, table_info);
        }

        Ok(SchemaInfo {
            tables: table_info_map,
            metadata,
        })
    }

    async fn get_database_metadata(&self) -> Result<DatabaseMetadata> {
        let name = "ANMI Database".to_string();
        let version = "Unknown".to_string();

        Ok(DatabaseMetadata {
            name,
            version,
            schema: self.config.schema.clone(),
        })
    }

    /// Execute a raw SQL query and return string results
    #[cfg(feature = "tokio-postgres")]
    pub async fn execute_query(&self, query: &str) -> Result<Vec<Vec<String>>> {
        if let Some(client) = &self.postgres_client {
            let rows = client
                .query(query, &[])
                .await
                .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

            // Convert rows to strings by handling PostgreSQL types properly
            let mut results = Vec::new();
            for row in &rows {
                let mut row_data = Vec::new();
                for i in 0..row.len() {
                    let column_type = row.columns()[i].type_();
                    let _column_name = row.columns()[i].name();

                    // Debug logging removed for production

                    let value_str = match column_type.name() {
                        "text" | "varchar" | "char" => row
                            .try_get::<_, Option<String>>(i)
                            .unwrap_or(None)
                            .unwrap_or_else(|| "NULL".to_string()),
                        "int4" => row
                            .try_get::<_, Option<i32>>(i)
                            .unwrap_or(None)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "NULL".to_string()),
                        "int8" => row
                            .try_get::<_, Option<i64>>(i)
                            .unwrap_or(None)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "NULL".to_string()),
                        "numeric" => {
                            // Handle PostgreSQL numeric/decimal types using rust_decimal
                            match row.try_get::<_, Option<rust_decimal::Decimal>>(i) {
                                Ok(Some(decimal)) => decimal.to_string(),
                                Ok(None) => "NULL".to_string(),
                                Err(_) => "NULL".to_string(),
                            }
                        }
                        "date" => row
                            .try_get::<_, Option<chrono::NaiveDate>>(i)
                            .unwrap_or(None)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "NULL".to_string()),
                        "float8" => row
                            .try_get::<_, Option<f64>>(i)
                            .unwrap_or(None)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "NULL".to_string()),
                        _ => {
                            // For unknown types, try string conversion
                            row.try_get::<_, Option<String>>(i)
                                .unwrap_or(None)
                                .unwrap_or_else(|| format!("({})", column_type.name()))
                        }
                    };
                    row_data.push(value_str);
                }
                results.push(row_data);
            }

            Ok(results)
        } else {
            Err(Error::DatabaseConnection(
                "Not connected to database".to_string(),
            ))
        }
    }

    #[cfg(not(feature = "tokio-postgres"))]
    pub async fn execute_query(&self, _query: &str) -> Result<Vec<Vec<String>>> {
        Err(Error::NotImplemented(
            "Database queries require tokio-postgres feature".to_string(),
        ))
    }
}
