// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod store;
mod version_control;

use version_control::{create_branch, create_change, create_team, get_teams};

fn main() {
    let _ = fix_path_env::fix(); // <---- Add this

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_teams,
            create_team,
            create_branch,
            create_change
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
use specta::collect_types;
#[cfg(test)]
use tauri_specta::ts;

#[test]
fn export_bindings() {
    ts::export(
        collect_types![get_teams, create_team, create_branch, create_change],
        "../src/bindings.ts",
    )
    .unwrap();
}
