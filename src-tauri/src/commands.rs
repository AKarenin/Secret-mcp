use crate::db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSecretInput {
    pub name: String,
    pub description: Option<String>,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSecretInput {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteEnvResult {
    pub success: bool,
    pub written: usize,
    pub missing: Vec<String>,
}

/// List all secrets (values masked)
#[tauri::command]
pub fn list_secrets() -> Result<Vec<db::SecretInfo>, String> {
    db::list_secrets()
}

/// Get a single secret by ID (includes value for editing)
#[tauri::command]
pub fn get_secret(id: String) -> Result<Option<db::Secret>, String> {
    db::get_secret(&id)
}

/// Create a new secret
#[tauri::command]
pub fn create_secret(input: CreateSecretInput) -> Result<db::Secret, String> {
    db::create_secret(&input.name, input.description.as_deref(), &input.value)
}

/// Update an existing secret
#[tauri::command]
pub fn update_secret(input: UpdateSecretInput) -> Result<db::Secret, String> {
    db::update_secret(
        &input.id,
        &input.name,
        input.description.as_deref(),
        &input.value,
    )
}

/// Delete a secret
#[tauri::command]
pub fn delete_secret(id: String) -> Result<bool, String> {
    db::delete_secret(&id)
}

/// Search secrets by name or description
#[tauri::command]
pub fn search_secrets(query: String) -> Result<Vec<db::SecretSearchResult>, String> {
    db::search_secrets(&query)
}

/// Write secrets to a .env file
#[tauri::command]
pub fn write_env(keys: Vec<String>, path: String) -> Result<WriteEnvResult, String> {
    let (written, missing) = db::write_env_file(&keys, &path)?;
    Ok(WriteEnvResult {
        success: true,
        written,
        missing,
    })
}

/// Get the database path (for MCP server configuration)
#[tauri::command]
pub fn get_db_path() -> String {
    db::get_db_path_string()
}
