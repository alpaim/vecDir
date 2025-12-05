use tauri::Manager;

use crate::state::AppState;

mod state;
mod database;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle();
            let app_dir = app_handle.path().app_data_dir().expect("failed to get app data dir");

            tauri::async_runtime::block_on(async {
                let pool = database::init::initialize_database(&app_dir).await
                    .expect("failed to initialize database");
                
                let state = AppState::new(pool);
                app.manage(state);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
