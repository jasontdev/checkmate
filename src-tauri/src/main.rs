// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
use crate::database::{Task, Day, Activity, Project};

#[tauri::command]
fn get_day(day: String) -> Day {
    Day {
        id: 0,
        date: day.to_string(),
        activities: vec![Activity {
            id: 0,
            project: Project {
                id: 0,
                name: "Mary Swinton".to_string(),
                tasks: vec![Task {
                    id: 0,
                    name: "Hello".to_string(),
                    project_id: 0,
                }],
            },
            task: Task {
                id: 0,
                name: "Hello".to_string(),
                project_id: 0,
            },
            day_id: 0,
            description: "Saying Hello to Mary".to_string(),
        }],
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_day])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
