use chrono::Utc;
use once_cell::sync::Lazy;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use uuid::Uuid;

/// Secret record for API responses (value masked)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Full secret record including value (for internal use)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub value: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Search result (name and description only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretSearchResult {
    pub name: String,
    pub description: Option<String>,
}

/// Global database connection
static DB: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

/// Get the database file path
fn get_db_path() -> PathBuf {
    let app_data = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    let app_dir = app_data.join("secret-mcp");
    std::fs::create_dir_all(&app_dir).ok();
    app_dir.join("secrets.db")
}

/// Get the database path as a string (for MCP server)
pub fn get_db_path_string() -> String {
    get_db_path().to_string_lossy().to_string()
}

/// Initialize the database connection
pub fn init_db() -> Result<(), String> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    // Create secrets table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS secrets (
            id TEXT PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            value TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    // Store connection globally
    let mut db = DB.lock().map_err(|e| e.to_string())?;
    *db = Some(conn);

    Ok(())
}

/// Helper to get database connection
fn with_db<T, F: FnOnce(&Connection) -> Result<T, String>>(f: F) -> Result<T, String> {
    let db = DB.lock().map_err(|e| e.to_string())?;
    let conn = db.as_ref().ok_or("Database not initialized")?;
    f(conn)
}

/// List all secrets (values masked)
pub fn list_secrets() -> Result<Vec<SecretInfo>, String> {
    with_db(|conn| {
        let mut stmt = conn
            .prepare("SELECT id, name, description, created_at, updated_at FROM secrets ORDER BY name")
            .map_err(|e| e.to_string())?;

        let secrets = stmt
            .query_map([], |row| {
                Ok(SecretInfo {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(secrets)
    })
}

/// Get a single secret by ID (includes value)
pub fn get_secret(id: &str) -> Result<Option<Secret>, String> {
    with_db(|conn| {
        let mut stmt = conn
            .prepare("SELECT id, name, description, value, created_at, updated_at FROM secrets WHERE id = ?")
            .map_err(|e| e.to_string())?;

        let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            Ok(Some(Secret {
                id: row.get(0).map_err(|e| e.to_string())?,
                name: row.get(1).map_err(|e| e.to_string())?,
                description: row.get(2).map_err(|e| e.to_string())?,
                value: row.get(3).map_err(|e| e.to_string())?,
                created_at: row.get(4).map_err(|e| e.to_string())?,
                updated_at: row.get(5).map_err(|e| e.to_string())?,
            }))
        } else {
            Ok(None)
        }
    })
}

/// Create a new secret
pub fn create_secret(name: &str, description: Option<&str>, value: &str) -> Result<Secret, String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    with_db(|conn| {
        conn.execute(
            "INSERT INTO secrets (id, name, description, value, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            params![id, name, description, value, now, now],
        )
        .map_err(|e| e.to_string())?;

        Ok(Secret {
            id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            value: value.to_string(),
            created_at: now,
            updated_at: now,
        })
    })
}

/// Update an existing secret
pub fn update_secret(
    id: &str,
    name: &str,
    description: Option<&str>,
    value: &str,
) -> Result<Secret, String> {
    let now = Utc::now().timestamp();

    with_db(|conn| {
        let rows_affected = conn
            .execute(
                "UPDATE secrets SET name = ?, description = ?, value = ?, updated_at = ? WHERE id = ?",
                params![name, description, value, now, id],
            )
            .map_err(|e| e.to_string())?;

        if rows_affected == 0 {
            return Err("Secret not found".to_string());
        }

        // Get created_at from existing record
        let mut stmt = conn
            .prepare("SELECT created_at FROM secrets WHERE id = ?")
            .map_err(|e| e.to_string())?;
        let created_at: i64 = stmt
            .query_row(params![id], |row| row.get(0))
            .map_err(|e| e.to_string())?;

        Ok(Secret {
            id: id.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            value: value.to_string(),
            created_at,
            updated_at: now,
        })
    })
}

/// Delete a secret
pub fn delete_secret(id: &str) -> Result<bool, String> {
    with_db(|conn| {
        let rows_affected = conn
            .execute("DELETE FROM secrets WHERE id = ?", params![id])
            .map_err(|e| e.to_string())?;

        Ok(rows_affected > 0)
    })
}

/// Search secrets by name or description (fuzzy match)
pub fn search_secrets(query: &str) -> Result<Vec<SecretSearchResult>, String> {
    with_db(|conn| {
        let pattern = format!("%{}%", query.to_lowercase());

        let mut stmt = conn
            .prepare(
                "SELECT name, description FROM secrets
                 WHERE LOWER(name) LIKE ? OR LOWER(COALESCE(description, '')) LIKE ?
                 ORDER BY name",
            )
            .map_err(|e| e.to_string())?;

        let results = stmt
            .query_map(params![&pattern, &pattern], |row| {
                Ok(SecretSearchResult {
                    name: row.get(0)?,
                    description: row.get(1)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(results)
    })
}

/// Get secret values by names (for writing .env files)
pub fn get_values_by_names(names: &[String]) -> Result<Vec<(String, String)>, String> {
    with_db(|conn| {
        let mut results = Vec::new();

        for name in names {
            let mut stmt = conn
                .prepare("SELECT name, value FROM secrets WHERE name = ?")
                .map_err(|e| e.to_string())?;

            if let Ok(value) = stmt.query_row(params![name], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            }) {
                results.push(value);
            }
        }

        Ok(results)
    })
}

/// Write secrets to a .env file
pub fn write_env_file(keys: &[String], path: &str) -> Result<(usize, Vec<String>), String> {
    // Validate path - must be absolute
    let path = std::path::Path::new(path);
    if !path.is_absolute() {
        return Err("Path must be absolute".to_string());
    }

    // Get values
    let values = get_values_by_names(keys)?;
    let found_names: std::collections::HashSet<_> = values.iter().map(|(n, _)| n.clone()).collect();

    // Find missing keys
    let missing: Vec<String> = keys
        .iter()
        .filter(|k| !found_names.contains(*k))
        .cloned()
        .collect();

    // Build .env content
    let content: String = values
        .iter()
        .map(|(name, value)| {
            // Escape value if needed
            if value.contains(' ')
                || value.contains('"')
                || value.contains('\'')
                || value.contains('\n')
            {
                format!("{}=\"{}\"\n", name, value.replace('"', "\\\""))
            } else {
                format!("{}={}\n", name, value)
            }
        })
        .collect();

    // Write file
    std::fs::write(path, content).map_err(|e| e.to_string())?;

    Ok((values.len(), missing))
}
