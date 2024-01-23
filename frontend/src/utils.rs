use crate::requests;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde_json;
use std::fs;

pub async fn get_note(
    note_id: &String,
    client: &reqwest::Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let note = requests::find_note(note_id.parse::<u16>()?, &client).await?;
    Ok(note)
}

pub async fn handle_auth_header() -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let token_path = "token.txt";
    let token = fs::read_to_string(token_path)?;

    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

    Ok(headers)
}
