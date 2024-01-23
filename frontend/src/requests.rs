use crate::utils;
use serde_json;
use std::collections::HashMap;

pub async fn get_all_notes(
    client: &reqwest::Client,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let url = "http://localhost:3000/notes";

    let headers = utils::handle_auth_header().await?;

    let res = client.get(url).headers(headers).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let notes = serde_json::from_str(&res.text().await?)?;
            return Ok(notes);
        }
        _ => {
            return Err("Something went wrong".into());
        }
    }
}

pub async fn find_note(
    note_id: u16,
    client: &reqwest::Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3000/notes/{}", note_id);

    let headers = utils::handle_auth_header().await?;

    let res = client.get(url).headers(headers).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let note = serde_json::from_str(&res.text().await?)?;
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

pub async fn create_note(
    client: &reqwest::Client,
    note_title: String,
    note_body: Option<String>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut data = HashMap::new();
    data.insert("title", note_title);

    let headers = utils::handle_auth_header().await?;

    if let Some(note_body) = note_body {
        data.insert("body", note_body);
    } else {
        data.insert("body", "".to_string());
    }

    let res = client
        .post("http://localhost:3000/notes")
        .headers(headers)
        .json(&data)
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::CREATED => {
            let note = serde_json::from_str(&res.text().await?)?;
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

pub async fn update_note(
    client: &reqwest::Client,
    note_id: u16,
    note_title: Option<String>,
    note_body: Option<String>,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3000/notes/{}", note_id);
    let mut data = HashMap::new();

    let headers = utils::handle_auth_header().await?;

    if let Some(note_title) = note_title {
        data.insert("title", note_title);
    }

    if let Some(note_body) = note_body {
        data.insert("body", note_body);
    }

    let res = client
        .patch(url)
        .headers(headers)
        .json(&data)
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::OK => {
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

pub async fn delete_note(
    client: &reqwest::Client,
    note_id: u16,
) -> Result<&str, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3000/notes/{}", note_id);

    let headers = utils::handle_auth_header().await?;

    let res = client.delete(url).headers(headers).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            return Ok("Note deleted");
        }
        reqwest::StatusCode::BAD_REQUEST => {
            return Err("Bad request".into());
        }
        _ => {
            return Err("Something went wrong".into());
        }
    }
}

pub async fn user_register(
    client: &reqwest::Client,
    email: String,
    password: String,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let url = "http://localhost:3000/users/signup";

    let headers = utils::handle_auth_header().await?;

    let mut data = HashMap::new();

    data.insert("email", email.trim());
    data.insert("password", password.trim());

    let res = client.post(url).headers(headers).json(&data).send().await?;

    match res.status() {
        reqwest::StatusCode::CREATED => {
            let user: HashMap<String, String> = res.json().await?;
            return Ok(user);
        }
        reqwest::StatusCode::BAD_REQUEST => {
            return Err("Bad request".into());
        }
        _ => {
            return Err("Something went wrong".into());
        }
    }
}

pub async fn user_login(
    client: &reqwest::Client,
    email: String,
    password: String,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let url = "http://localhost:3000/users/signin";

    let headers = utils::handle_auth_header().await?;

    let mut data = HashMap::new();

    data.insert("email", email.trim());
    data.insert("password", password.trim());

    let res = client.post(url).headers(headers).json(&data).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let user: HashMap<String, String> = res.json().await?;
            return Ok(user);
        }
        reqwest::StatusCode::BAD_REQUEST => {
            return Err("Bad request".into());
        }
        _ => {
            return Err("Something went wrong".into());
        }
    }
}

pub async fn user_verify(
    client: &reqwest::Client,
    token: String,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    let mut data = HashMap::new();

    data.insert("jwt", token);

    let url = "http://localhost:3000/users/verify";

    let res = client.post(url).json(&data).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            return Ok("User verified");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err("Unauthorized".into());
        }
        _ => {
            return Err("Something went wrong".into());
        }
    }
}
