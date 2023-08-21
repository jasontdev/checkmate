// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use model::traits::CRUD;
use rusqlite::{Connection, Error};
use tauri::{Manager, State};

use model::Day;
use model::Task;

mod model;

pub struct AppState {
    pub(crate) db: std::sync::Mutex<Option<Connection>>,
}

#[tauri::command]
fn get_day(app_state: State<AppState>, date: String) -> Result<Day, String> {
    // should insert a new row where the requested day does not exist
    match Day::find_or_create(
        app_state.db.lock().unwrap().as_ref().unwrap(),
        date.to_string(),
    ) {
        Ok(day) => Ok(Day {
            id: day.id,
            date: day.date,
            activities: day.activities,
        }),
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

fn save_task(app_state: State<AppState>, task: Task) -> Result<Task, String> {
    let connection = app_state.db.lock().unwrap();
    match task.create(&connection.as_ref().unwrap()) {
        Ok(task) => Ok(task),
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
