use std::collections::HashMap;

pub async fn create(
    client: &reqwest::Client,
    file_name: String,
    file_body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = HashMap::new();
    data.insert("file_name", file_name);
    data.insert("file_name", file_body);

    let res = client
        .post("http://localhost:3001/notes")
        .json(&data)
        .send()
        .await?;

    Ok(())
}

pub async fn update(
    client: &reqwest::Client,
    file_id: u16,
    file_name: Option<String>,
    file_body: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3001/notes/{}", file_id);
    let mut data = HashMap::new();

    if let Some(file_name) = file_name {
        data.insert("file_name", file_name);
    }

    if let Some(file_body) = file_body {
        data.insert("file_body", file_body);
    }

    let res = client.patch(url).json(&data).send().await;

    Ok(())
}

pub async fn delete(
    client: &reqwest::Client,
    file_id: String,
) -> Result<&str, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3001/notes/{}", file_id);

    client.delete(url).send().await?;

    let message = "file with id: {file_id} has been deleted";

    Ok(message)
}

pub async fn find(file_id: String) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3001/notes/{}", file_id);

    let res_body = reqwest::get(url).await?.text().await?;

    Ok(res_body)
}
