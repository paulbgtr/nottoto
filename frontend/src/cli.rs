use crate::args::Args;
use crate::requests;

pub async fn handle_args(
    args: Args,
    client: reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(file_name) = args.create {
        let file_body = String::new();
        requests::create(&client, file_name, file_body).await?;
    }

    if let Some(file_id) = args.update {
        // todo: implement this
    }

    if let Some(file_id) = args.delete {
        let res = requests::delete(&client, file_id).await?;
    }

    if let Some(file_name) = args.find {
        requests::find(file_name).await?;
    }

    Ok(())
}
