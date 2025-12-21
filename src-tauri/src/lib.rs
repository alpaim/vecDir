use anyhow::Context;
use specta_typescript::Typescript;
use tauri::{AppHandle, Manager};

use tauri_specta::{collect_commands, collect_events, Builder, Event};

use crate::{ai::AI, state::AppState};

mod ai;
mod database;
mod indexer;
mod search;
mod state;
mod status;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// This function/command checks if app state is ready
#[tauri::command]
#[specta::specta]
fn check_is_state_ready(app: AppHandle) -> bool {
    app.try_state::<AppState>().is_some()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            check_is_state_ready,
            // DATABASE
            database::commands::get_config,
            database::commands::update_config,
            database::commands::create_space,
            database::commands::update_space,
            database::commands::get_space_by_id,
            database::commands::get_all_spaces,
            database::commands::add_root,
            database::commands::delete_root,
            database::commands::get_roots_by_space_id,
            database::commands::get_files_by_ids,
            // INDEXER
            indexer::commands::index_space,
            indexer::commands::process_space,
            // SEARCH
            search::commands::search_by_emdedding,
        ])
        .events(collect_events![
            status::events::BackendReadyEvent,
            status::events::StatusEvent,
            status::events::ErrorEvent,
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            Typescript::default().header("// @ts-nocheck"),
            "../src/lib/vecdir/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            let app_handle = app.handle().clone();
            let app_dir = app_handle
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            tauri::async_runtime::spawn(async move {
                let sqlx_pool = database::init::initialize_database(&app_dir)
                    .await
                    .expect("failed to initialize database");

                let state = AppState::new(sqlx_pool);
                app_handle.manage(state);

                // emitting event to frontend telling backend is ready
                status::events::BackendReadyEvent.emit(&app_handle)
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
