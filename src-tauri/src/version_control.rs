use serde::Serialize;
use specta::Type;

#[derive(Serialize, Type)]
pub struct Team {
    pub title: String,
    pub description: String,
    // pub created_at: time::SystemTime,
    pub branches: Vec<Branch>,
    pub current_branch: String,
}

#[derive(Serialize, Type)]
pub struct Branch {
    pub id: String,
    pub title: String,
    pub history: Vec<Change>,
    pub current_change: String,
}

#[derive(Serialize, Type)]
pub struct Change {
    pub id: String,
    pub message: String,
    pub context: String,
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn get_team() -> Vec<Team> {
    vec![Team {
        title: "test".to_string(),
        description: "test".to_string(),
        // created_at: time::SystemTime::now(),
        branches: vec![],
        current_branch: "".to_string(),
    }]
}
