mod commands;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // Initialize database
            db::init_db().expect("Failed to initialize database");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_secrets,
            commands::get_secret,
            commands::create_secret,
            commands::update_secret,
            commands::delete_secret,
            commands::search_secrets,
            commands::write_env,
            commands::get_db_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
