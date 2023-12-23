use std::collections::HashMap;

pub async fn create(
    client: &reqwest::Client,
    note_title: String,
    note_body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = HashMap::new();
    data.insert("file_name", note_title);
    data.insert("file_name", note_body);

    let res = client
        .post("http://localhost:3000/notes")
        .json(&data)
        .send()
        .await?;

    Ok(())
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

    let res = client.patch(url).json(&data).send().await;

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

pub async fn find(note_id: u16) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3000/notes/{}", note_id);

    let res_body = reqwest::get(url).await?.text().await?;

    Ok(res_body)
}
