use crate::requests;

pub async fn handle_args(
    args: crate::args::Args,
    client: reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.all {
        let notes = requests::get_all_notes().await?;

        if notes.len() == 0 {
            println!("No notes are found");
            return Ok(());
        }

        for note in notes.iter() {
            println!("{}: {}", note["id"], note["title"]);
        }
    }

    if let Some(note_id) = args.find {
        let note = requests::find_note(note_id.parse::<u16>()?).await?;

        if note.len() == 0 {
            println!("Note with an id of {} is not found", note_id);
        }

        let (id, title) = (note["id"].to_string(), note["title"].to_string());

        println!("{}: {}", id, title);
    }

    if let Some(note_title) = args.create {
        // todo: impl creating a temp file and opening it in vim

        let created_note = requests::create(&client, note_title, None).await?;

        let (id, title) = (
            created_note["id"].to_string(),
            created_note["title"].to_string(),
        );

        println!("{}: {}", id, title);
    }

    if let Some(note_title) = args.update {
        println!("Updating note with title: {}", note_title);
    }

    if let Some(note_title) = args.delete {
        println!("Deleting note with title: {}", note_title);
    }

    Ok(())
}
