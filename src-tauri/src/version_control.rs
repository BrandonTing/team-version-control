use serde::{Deserialize, Serialize};
use serde_json::json;
use specta::Type;
use tauri_plugin_store::StoreBuilder;

#[derive(Serialize, Type, Deserialize, Debug)]
pub struct Team {
    pub title: String,
    pub description: String,
    // pub created_at: time::SystemTime,
    pub branches: Vec<Branch>,
    pub current_branch: String,
}

#[derive(Serialize, Type, Deserialize, Debug)]
pub struct Branch {
    pub id: String,
    pub title: String,
    pub history: Vec<Change>,
    pub current_change: String,
}

#[derive(Serialize, Type, Deserialize, Debug)]
pub struct Change {
    pub id: String,
    pub message: String,
    pub context: String,
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn get_team(app_handle: tauri::AppHandle) -> Vec<Team> {
    let mut store = StoreBuilder::new(app_handle, ".team.dat".parse().unwrap()).build();

    let _ = store.load();
    // let _ = store.insert(
    //     "team1".to_string(),
    //     json!(Team {
    //         title: "test".to_string(),
    //         description: "test".to_string(),
    //         // created_at: time::SystemTime::now(),
    //         branches: vec![],
    //         current_branch: "".to_string(),
    //     }),
    // );
    // match store.save() {
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    //     _ => println!("saved"),
    // };
    match serde_json::from_value::<Vec<Team>>(store.values().map(|x| x.clone()).collect()) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            vec![]
        }
    }
}
