// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod task;
mod time_block;

use app;
use rusqlite::{Connection, Error};
use tauri::{Manager, State};

pub struct AppState {
    pub(crate) db: std::sync::Mutex<Option<Connection>>,
}

#[tauri::command]
fn create_tables(app_state: State<AppState>) -> Result<(), String> {
    let connection = app_state.db.lock().unwrap();
    if let Err(_) = app::create_tables(connection.as_ref().unwrap()) {
        return Err("Error creating task table".to_string());
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![create_tables])
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<AppState> = handle.state();
            *app_state.db.lock().unwrap() =
                Some(Connection::open("checkmate.db").expect("Database:"));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
