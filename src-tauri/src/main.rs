// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod store;
mod version_control;
use version_control::{
    create_branch, create_branch_from_change, create_change, create_team, delete_team,
    get_change_history, get_team, get_teams, reset_store,
};
mod upload;
use upload::upload;

fn main() {
    let _ = fix_path_env::fix(); // <---- Add this

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_teams,
            get_team,
            create_team,
            delete_team,
            create_branch,
            create_change,
            reset_store,
            get_change_history,
            upload,
            create_branch_from_change
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
            create_change,
            reset_store,
            get_change_history,
            upload,
            delete_team,
            create_branch_from_change
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
