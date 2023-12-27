use crate::requests;
use crate::utils;

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
        let note = utils::get_note(&note_id).await?;

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

    if let Some(note_id) = args.rename {
        let note = utils::get_note(&note_id).await?;

        let note_title = note["title"].to_string();
        let mut new_note_title = String::new();

        println!("Enter new title for the note: {}", &note_title);

        std::io::stdin().read_line(&mut new_note_title)?;

        let renamed_note =
            requests::update(&client, note_id.parse::<u16>()?, Some(new_note_title), None).await?;

        let (id, title) = (
            renamed_note["id"].to_string(),
            renamed_note["title"].to_string(),
        );

        println!("Updated note: {} {}", id, title);
    }

    if let Some(note_id) = args.delete {
        let note = utils::get_note(&note_id).await?;

        let (id, title) = (note["id"].to_string(), note["title"].to_string());

        println!("Are you sure you want to delete {} {}? (y/n)", id, title);

        let mut confirmation = String::new();

        std::io::stdin().read_line(&mut confirmation)?;

        if confirmation.trim() == "y" {
            requests::delete(&client, note_id.parse::<u16>()?).await?;
            println!("Note {} {} is deleted", id, title);
        } else {
            println!("Note {} {} is not deleted", id, title);
        }
    }

    Ok(())
}
