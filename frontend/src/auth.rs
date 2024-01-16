use crate::requests;
use std::fs;

pub async fn is_logged_in(client: &reqwest::Client) -> Result<bool, Box<dyn std::error::Error>> {
    let token_path = "token.txt";
    let token = fs::read_to_string(token_path);

    if let Ok(token) = token {
        let user = requests::user_verify(&client, token).await;

        if let Ok(_) = user {
            return Ok(true);
        } else {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }
}
