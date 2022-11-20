#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use diesel_migrations::{embed_migrations};

use std::{sync::Mutex};

#[macro_use]
extern crate diesel;
#[macro_use] 
extern crate diesel_migrations;
embed_migrations!("./migrations");

use diesel::prelude::*;
pub mod schema;
pub mod db;


#[tauri::command]
fn todos_list(state: tauri::State<AppState>) -> String{
    let con = state.conn.lock().unwrap();
    db::todos_list(&con)
}
#[tauri::command]
fn todos_create(body: String, state: tauri::State<AppState>) -> String{
    let conn = state.conn.lock().unwrap();
    db::todos_create(&conn, &body).to_string()
}

#[tauri::command]
fn todos_toggle(id: i32, state: tauri::State<AppState>) -> String{
    let conn = state.conn.lock().unwrap();
    db::todos_toggle(&conn, id)
}
#[tauri::command]
fn todos_delete(id: i32, state: tauri::State<AppState>) -> String {
    let conn = state.conn.lock().unwrap();
    db::todos_delete(&conn, id);
    String::from("")
}
// End of DB example

struct AppState {
    count: Mutex<i64>,
    conn: Mutex<SqliteConnection>,
}

// Count Commands
#[tauri::command]
fn get_count(state: tauri::State<AppState>) -> i64 {
    state.count.lock().unwrap().clone()
}

#[tauri::command]
fn update_count(update: i64, state: tauri::State<AppState>) -> i64 {
    let mut cnt = state.count.lock().unwrap();
    *cnt += update;
    cnt.clone()
}

fn main() {
    let conn = db::establish_connection();

    // embedded_migrations::run(&conn);
    embedded_migrations::run(&conn).expect("Error migrating");

    let state = AppState {
        count: Default::default(),
        conn: Mutex::new(db::establish_connection()),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_count,
            update_count,
            todos_list,
            todos_create,
            todos_toggle,
            todos_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}