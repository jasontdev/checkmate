// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use model::traits::Entity;
use rusqlite::{Connection, Error};
use tauri::Window;
use tauri::{Manager, State};

use model::Day;
use model::Task;

mod model;

pub struct AppState {
    pub(crate) db: std::sync::Mutex<Option<Connection>>,
}

#[tauri::command]
fn get_day(app_state: State<AppState>, date: String) -> Result<Day, String> {
    match Day::find_or_create(
        app_state.db.lock().unwrap().as_ref().unwrap(),
        date.to_string(),
    ) {
        Ok(day) => Ok(Day {
            id: day.id,
            date: day.date,
            tasks: day.tasks,
        }),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
fn update_day(_app_state: State<AppState>, window: Window, day: Day) -> Result<Day, String> {
    let event_name = format!("day_{}_updated", str::replace(&day.date, " ", "_"));

    window
        .emit(
            &event_name,
            Day {
                id: day.id,
                date: day.date.clone(),
                tasks: day.tasks.clone(),
            },
        )
        .unwrap();

    Ok(Day {
        id: day.id,
        date: day.date,
        tasks: day.tasks,
    })
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
        .invoke_handler(tauri::generate_handler![create_tables, get_day, update_day])
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
