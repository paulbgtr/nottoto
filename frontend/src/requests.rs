use std::collections::HashMap;

async fn create(
    client: reqwest::Client,
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

async fn update(
    client: reqwest::Client,
    file_name: Option<String>,
    file_body: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = HashMap::new();

    if let Some(file_name) = file_name {
        data.insert("file_name", file_name);
    }

    if let Some(file_body) = file_body {
        data.insert("file_body", file_body);
    }

    let res = client
        .post("http://localhost:3001/notes")
        .json(&data)
        .send()
        .await;

    Ok(())
}
