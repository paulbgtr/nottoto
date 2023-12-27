use std::collections::HashMap;

pub async fn get_all_notes() -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let url = "http://localhost:3000/notes";

    let res = reqwest::get(url).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let notes: Vec<HashMap<String, String>> = res.json().await?;
            return Ok(notes);
        }
        _ => {
            return Err("Something went wrong".into());
        }
    }
}

pub async fn find_note(
    note_id: u16,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3000/notes/{}", note_id);

    let res = reqwest::get(url).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let note: HashMap<String, String> = res.json().await?;
            return Ok(note);
        }
        reqwest::StatusCode::NOT_FOUND => {
            return Err("Note not found".into());
        }
        _ => {
            return Err("Something went wrong".into());
        }
    }
}

pub async fn create(
    client: &reqwest::Client,
    note_title: String,
    note_body: Option<String>,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut data = HashMap::new();
    data.insert("title", note_title);

    if let Some(note_body) = note_body {
        data.insert("body", note_body);
    }

    let res = client
        .post("http://localhost:3000/notes")
        .json(&data)
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::CREATED => {
            let note: HashMap<String, String> = res.json().await?;
            return Ok(note);
        }
        reqwest::StatusCode::BAD_REQUEST => {
            return Err("Bad request".into());
        }
        _ => {
            return Err("Something went wrong".into());
        }
    }
}

pub async fn update(
    client: &reqwest::Client,
    note_id: u16,
    note_title: Option<String>,
    note_body: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3000/notes/{}", note_id);
    let mut data = HashMap::new();

    if let Some(note_title) = note_title {
        data.insert("title", note_title);
    }

    if let Some(note_body) = note_body {
        data.insert("body", note_body);
    }

    let res = client.patch(url).json(&data).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let note: HashMap<String, String> = res.json().await?;
            println!("{:?}", note);
        }
        reqwest::StatusCode::BAD_REQUEST => {
            return Err("Bad request".into());
        }
        _ => {
            return Err("Something went wrong".into());
        }
    }

    Ok(())
}

pub async fn delete(
    client: &reqwest::Client,
    note_id: u16,
) -> Result<&str, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3000/notes/{}", note_id);

    client.delete(url).send().await?;

    let message = "file with id: {file_id} has been deleted";

    Ok(message)
}
