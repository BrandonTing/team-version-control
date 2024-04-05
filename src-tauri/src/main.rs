// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod store;
mod version_control;
use version_control::{create_branch, create_change, create_team, get_team, get_teams};

fn main() {
    let _ = fix_path_env::fix(); // <---- Add this

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_teams,
            get_team,
            create_team,
            create_branch,
            create_change
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
use specta::collect_types;
#[cfg(test)]
use tauri_specta::ts;

#[test]
fn export_bindings() {
    match ts::export(
        collect_types![
            get_teams,
            get_team,
            create_team,
            create_branch,
            create_change
        ],
        "../src/lib/bindings.ts",
    ) {
        Err(e) => {
            println!("Error: {:?}", e)
        }
        _ => {
            println!("Finished")
        }
    }
}
