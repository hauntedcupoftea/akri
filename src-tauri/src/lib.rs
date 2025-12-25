mod db;
mod models;

use models::{
    AddSubjectInput, TemplateDTO, TestInput, TestRecord, UpdateSubjectInput, UpdateTestInput,
};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    db: Mutex<Connection>,
}

#[tauri::command]
fn get_history(state: State<AppState>) -> Result<Vec<TestRecord>, String> {
    let conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::get_history(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn add_test(state: State<AppState>, data: TestInput) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::create_entry(&mut conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_test(state: State<AppState>, data: UpdateTestInput) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::update_test(&mut conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_test(state: State<AppState>, id: i64) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::delete_test(&mut conn, id).map_err(|e| e.to_string())
}

// --- SUBJECTS ---

#[tauri::command]
fn add_subject(state: State<AppState>, data: AddSubjectInput) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::add_subject(&mut conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_subject(state: State<AppState>, data: UpdateSubjectInput) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::update_subject(&mut conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_subject(state: State<AppState>, id: i64) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::delete_subject(&mut conn, id).map_err(|e| e.to_string())
}

// --- TEMPLATES ---

#[tauri::command]
fn create_template(state: State<AppState>, data: TemplateDTO) -> Result<(), String> {
    let conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::create_template(&conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_templates(state: State<AppState>) -> Result<Vec<TemplateDTO>, String> {
    let conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::get_templates(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_template(state: State<AppState>, data: TemplateDTO) -> Result<(), String> {
    let conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::update_template(&conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_template(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|_| "Failed to lock DB")?;
    db::delete_template(&conn, id).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let conn = db::init_db(app.handle()).expect("Failed to init DB");
            app.manage(AppState {
                db: Mutex::new(conn),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_history,
            add_test,
            update_test,
            delete_test,
            add_subject,
            update_subject,
            delete_subject,
            create_template,
            get_templates,
            update_template,
            delete_template
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
