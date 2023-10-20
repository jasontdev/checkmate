// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod task;
mod time_block;

use app;
use rusqlite::{Connection, Error};
use tauri::{Manager, State};

pub struct DBMutex(std::sync::Mutex<Option<Connection>>);

fn db_wrapper<T>(db_mutex: &DBMutex, f: fn(&Connection) -> Result<T, Error>) -> Result<T, Error> {
    let mutex_guard = db_mutex.0.lock().expect("DB connection mutex poisoned");
    let connection = mutex_guard
        .as_ref()
        .expect("Could not obtain DB connection");
    f(connection)
}

#[tauri::command]
fn create_tables(db_mutex: State<DBMutex>) -> Result<(), String> {
    match db_wrapper::<()>(&db_mutex, app::create_tables) {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not create database tables".to_string()),
    }
}

fn main() -> Result<(), Error> {
    tauri::Builder::default()
        .manage(DBMutex(Default::default()))
        .invoke_handler(tauri::generate_handler![create_tables])
        .setup(|app| {
            let handle = app.handle();
            let connection: State<DBMutex> = handle.state();
            *connection.0.lock().unwrap() =
                Some(Connection::open("checkmate.db").expect("Database:"));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
