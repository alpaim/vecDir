use tauri::{AppHandle, Emitter, Manager, State};

use crate::state::AppState;

mod state;
mod database;
mod vector_database;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// This function/command checks if app state is ready
#[tauri::command]
fn check_is_state_ready(app: AppHandle) -> bool {
    app.try_state::<AppState>().is_some()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_dir = app_handle.path().app_data_dir().expect("failed to get app data dir");

            tauri::async_runtime::spawn(async move {
                let sqlx_pool = database::init::initialize_database(&app_dir).await
                    .expect("failed to initialize database");
                let vec_db_connection = vector_database::init::initialize_vector_database(&app_dir).await
                    .expect("failed to initialize vector database");
                
                let state = AppState::new(sqlx_pool, vec_db_connection);
                app_handle.manage(state);

                // emitting event to frontend telling backend is ready
                app_handle.emit("backend-is-ready", ())
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![check_is_state_ready])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
