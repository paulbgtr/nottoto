use crate::requests;

pub async fn is_logged_in(client: &reqwest::Client) -> Result<bool, Box<dyn std::error::Error>> {
    let user = requests::user_verify(&client).await;

    match user {
        Ok(_) => {
            return Ok(true);
        }
        Err(_) => {
            return Ok(false);
        }
    }
}
