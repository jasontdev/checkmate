// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Error};
use tauri::{Manager, State};

use model::Day;

mod model;

pub struct AppState {
    pub(crate) db: std::sync::Mutex<Option<Connection>>,
}

#[tauri::command]
fn get_day(app_state: State<AppState>, date: String) -> Result<Day, String> {
    // should insert a new row where the requested day does not exist
    match Day::find_by_date(
        app_state.db.lock().unwrap().as_ref().unwrap(),
        date.to_string(),
    ) {
        Ok(day) => match day {
            // TODO: replace with call to create()
            None => Ok(Day {
                id: 0,
                date: date.to_string(),
                activities: vec![],
            }),
            Some(day) => Ok(day),
        },
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
fn create_tables(app_state: State<AppState>) -> Result<(), String> {
    match Day::create_table(app_state.db.lock().unwrap().as_ref().unwrap()) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

fn main() -> Result<(), Error> {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![create_tables, get_day])
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
