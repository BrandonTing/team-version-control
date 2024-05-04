#[tauri::command]
#[specta::specta] // <-- This bit here
pub async fn upload(title: String, paste: String) -> Result<String, String> {
    let params = [
        ("title", title),
        ("paste", paste.clone()),
        ("author", "Version Control User".to_string()),
    ];
    let client = reqwest::Client::new();
    let response = client
        .post("https://pokepast.es/create")
        .form(&params)
        .send()
        .await;
    let url = match response {
        Ok(res) => {
            let url = res.url();
            url.clone()
        }
        Err(_e) => {
            println!("Failed to create paste");
            return Err("Failed to create paste".to_string());
        }
    };
    return Ok(url.to_string());
}
