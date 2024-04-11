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
}

#[derive(Serialize, Type, Deserialize, Debug)]
pub struct TeamDetail {
    pub team: Team,
    pub branches: Vec<Branch>,
    pub current_branch_title: String,
}

impl TeamDetail {
    fn new(title: String, description: String, main_branch_title: String) -> Self {
        TeamDetail {
            team: Team {
                title: title.to_string(),
                description: description.to_string(),
                created_at: Utc::now().timestamp(),
            },
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
pub fn reset_store(app_handle: tauri::AppHandle) {
    let mut store = get_store(app_handle);
    let _ = store.clear();
    let _ = store.save();
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn get_teams(app_handle: tauri::AppHandle) -> Result<Vec<Team>, String> {
    let store = get_store(app_handle);
    match serde_json::from_value::<Vec<TeamDetail>>(store.values().map(|x| x.clone()).collect()) {
        Ok(v) => Ok(v.into_iter().map(|x| x.team).collect()),
        Err(e) => {
            println!("{:?}", e);
            Err("Failed to fetch teams".to_string())
        }
    }
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn get_team(app_handle: tauri::AppHandle, key: String) -> Result<TeamDetail, String> {
    let store = get_store(app_handle);
    match serde_json::from_value::<TeamDetail>(store.get(key).unwrap().clone()) {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{:?}", e);
            Err("Failed to get target team".to_string())
        }
    }
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn create_team(
    app_handle: tauri::AppHandle,
    title: String,
    description: String,
    main_branch_title: String,
) -> Result<TeamDetail, String> {
    let mut store = get_store(app_handle);
    match store.get(&title) {
        Some(_) => {
            return Err("The title has been used.".to_string());
        }
        _ => {
            println!("No conflict.")
        }
    };

    let detail = TeamDetail::new(title.clone(), description, main_branch_title);
    match store.insert(title, json!(detail)) {
        Ok(_) => {
            match store.save() {
                Err(_) => return Err("Failed to save info to file".to_string()),
                _ => println!("saved"),
            };
            return Ok(detail);
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
) -> Result<(), String> {
    let mut store = get_store(app_handle);
    let mut detail = match store.get(&team_title) {
        None => {
            return Err("Target team doesn't exist.".to_string());
        }
        Some(v) => match from_value::<TeamDetail>(v.clone()) {
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
    match detail.branches.iter().find(|x| x.title == title) {
        Some(_) => {
            println!("failed");
            return Err("Please provide unique title for the branch.".to_string());
        }
        None => {
            println!("add a branch");
            detail.branches.push(branch);
            match store.insert(team_title, json!(detail)) {
                Err(e) => {
                    println!("{:?}", e);
                    return Err("Failed to update branch".to_string());
                }
                _ => {
                    println!("branch inserted");
                }
            };
            match store.save() {
                Err(e) => {
                    println!("{:?}", e);
                    return Err("Failed to save branch update".to_string());
                }
                _ => {
                    println!("branch saved");
                }
            };
        }
    };
    return Ok(());
}

#[tauri::command]
#[specta::specta] // <-- This bit here
pub fn create_change(
    app_handle: tauri::AppHandle,
    team_title: String,
    branch_title: String,
    message: String,
    context: String,
) -> Result<(), String> {
    let mut store = get_store(app_handle);
    let mut detail = match store.get(&team_title) {
        None => {
            return Err("Target team doesn't exist.".to_string());
        }
        Some(v) => match from_value::<TeamDetail>(v.clone()) {
            Err(_) => {
                return Err("Failed to parse team.".to_string());
            }
            Ok(t) => t,
        },
    };
    let branch = match detail.branches.iter_mut().find(|x| x.title == branch_title) {
        None => {
            return Err("Failed to find target branch.".to_string());
        }
        Some(branch) => branch,
    };
    let id = Uuid::new_v4().to_string();
    let change = Change {
        id: id.clone(),
        message,
        context,
    };
    branch.history.push(change.clone());
    branch.current_change_id = id;
    println!("{:?}", detail);
    match store.insert(team_title, json!(detail)) {
        Err(e) => {
            println!("{:?}", e);
            return Err("Failed to update change".to_string());
        }
        _ => {
            println!("change inserted");
        }
    };
    match store.save() {
        Err(e) => {
            println!("{:?}", e);
            return Err("Failed to save change update".to_string());
        }
        _ => {
            println!("change saved");
        }
    };

    return Ok(());
}
