// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use specta::collect_types;
use specta::Type;
use tauri_specta::ts;

#[derive(Deserialize, Type)]
pub struct GreetArg {
    pub name: String,
}

#[derive(Serialize, Type)]
pub struct GreetReturn {
    pub greet: String,
}

#[tauri::command]
#[specta::specta] // <-- This bit here
fn greet(arg: GreetArg) -> GreetReturn {
    GreetReturn {
        greet: format!("Hello, {}!", arg.name),
    }
}
fn main() {
    ts::export(collect_types![greet], "../src/bindings.ts").unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[test]
fn export_bindings() {
    ts::export(collect_types![greet], "../src/bindings.ts").unwrap();
}
