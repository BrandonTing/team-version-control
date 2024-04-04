use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};
use specta::Type;
use std::vec;
use uuid::Uuid;

use crate::store::get_store;

#[derive(Serialize, Type, Deserialize, Debug)]
pub struct Team {
    pub title: String,
    pub description: String,
    #[specta(type = u32)]
    pub created_at: i64,
    pub branches: Vec<Branch>,
    pub current_branch_title: String,
}

impl Team {
    fn new(title: String, description: String, main_branch_title: String) -> Self {
        Team {
            title: title.to_string(),
            description: description.to_string(),
            created_at: Utc::now().timestamp(),
            branches: vec![Branch {
                title: main_branch_title.clone(),
                description: "".to_string(),
                history: vec![],
                current_change_id: "".to_string(),
            }],
            current_branch_title: main_branch_title.to_string(),
        }
    }
}

#[derive(Serialize, Type, Deserialize, Clone, Debug)]
pub struct Branch {
    pub title: String,
    pub description: String,
    pub history: Vec<Change>,
    pub current_change_id: String,
}

#[derive(Serialize, Type, Deserialize, Clone, Debug)]
pub struct Change {
    pub id: String,
    pub message: String,
    pub context: String,
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn get_teams(app_handle: tauri::AppHandle) -> Result<Vec<Team>, String> {
    let store = get_store(app_handle);
    match serde_json::from_value::<Vec<Team>>(store.values().map(|x| x.clone()).collect()) {
        Ok(v) => Ok(v),
        Err(_) => Err("Failed to fetch teams".to_string()),
    }
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn create_team(
    app_handle: tauri::AppHandle,
    title: String,
    description: String,
    main_branch_title: String,
) -> Result<Team, String> {
    let mut store = get_store(app_handle);
    match store.get(&title) {
        Some(_) => {
            return Err("The title has been used.".to_string());
        }
        _ => {
            println!("No conflict.")
        }
    };

    let team = Team::new(title.clone(), description, main_branch_title);
    match store.insert(title, json!(team)) {
        Ok(_) => {
            match store.save() {
                Err(_) => return Err("Failed to save info to file".to_string()),
                _ => println!("saved"),
            };
            return Ok(team);
        }
        Err(_) => return Err("Failed to insert team to store".to_string()),
    }
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn create_branch(
    app_handle: tauri::AppHandle,
    team_title: String,
    title: String,
    description: String,
) -> Result<Branch, String> {
    let store = get_store(app_handle);
    let mut team = match store.get(&team_title) {
        None => {
            return Err("Target team doesn't exist.".to_string());
        }
        Some(v) => match from_value::<Team>(v.clone()) {
            Err(_) => {
                return Err("Failed to parse team.".to_string());
            }
            Ok(t) => t,
        },
    };
    let branch = Branch {
        title: title.to_string(),
        description: description.to_string(),
        history: vec![],
        current_change_id: "".to_string(),
    };
    match team.branches.iter().find(|x| x.title == title) {
        Some(_) => {
            return Err("Please provide unique title for the branch.".to_string());
        }
        None => {
            team.branches.push(branch.clone());
        }
    };
    return Ok(branch);
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn create_change(
    app_handle: tauri::AppHandle,
    team_title: String,
    branch_title: String,
    message: String,
    context: String,
) -> Result<Change, String> {
    let store = get_store(app_handle);
    let mut team = match store.get(&team_title) {
        None => {
            return Err("Target team doesn't exist.".to_string());
        }
        Some(v) => match from_value::<Team>(v.clone()) {
            Err(_) => {
                return Err("Failed to parse team.".to_string());
            }
            Ok(t) => t,
        },
    };
    let branch = match team.branches.iter_mut().find(|x| x.title == branch_title) {
        None => {
            return Err("Failed to find target branch.".to_string());
        }
        Some(branch) => branch,
    };
    let change = Change {
        id: Uuid::new_v4().to_string(),
        message,
        context,
    };
    branch.history.push(change.clone());
    return Ok(change);
}
