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

        println!("Here are your notes:\n");

        for note in notes.iter() {
            println!("{}: {}", note["id"], note["title"]);
        }
    }

    if let Some(note_title) = args.create {
        println!("Creating note with title: {}", note_title);
    }

    if let Some(note_title) = args.update {
        println!("Updating note with title: {}", note_title);
    }

    if let Some(note_title) = args.delete {
        println!("Deleting note with title: {}", note_title);
    }

    if let Some(note_title) = args.find {
        println!("Finding note with title: {}", note_title);
    }

    Ok(())
}

