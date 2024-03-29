use crate::requests;
use std::fs;

pub fn user_logout() -> Result<(), Box<dyn std::error::Error>> {
    let token_path = "token.txt";
    fs::remove_file(token_path)?;

    println!("You have been logged out");

    Ok(())
}

pub async fn is_logged_in(client: &reqwest::Client) -> Result<bool, Box<dyn std::error::Error>> {
    let token_path = "token.txt";
    let token = fs::read_to_string(token_path);

    match token {
        Ok(token) => {
            let user = requests::user_verify(&client, token).await;

            match user {
                Ok(_) => {
                    return Ok(true);
                }
                Err(_) => {
                    return Ok(false);
                }
            }
        }
        Err(_) => {
            return Ok(false);
        }
    }
}
