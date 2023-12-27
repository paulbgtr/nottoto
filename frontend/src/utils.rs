use crate::requests;
use std::collections::HashMap;

pub async fn get_note(
    note_id: &String,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let note = requests::find_note(note_id.parse::<u16>()?).await?;

    if note.len() == 0 {
        return Err("Note with an id of {} is not found".into());
    }

    Ok(note)
}

