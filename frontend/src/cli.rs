use crate::auth;
use crate::requests;
use crate::utils;
use std::io::{Read, Seek, SeekFrom, Write};
use tempfile::NamedTempFile;

pub async fn handle_args(
    args: crate::args::Args,
    client: reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let is_logged_in = auth::is_logged_in(&client).await?;

    if !args.join && !args.login && !is_logged_in {
        println!("You are not logged in");
        println!("Please login first using -l or --login");
        println!("Or register using -j or --join");
        return Ok(());
    }

    if args.join {
        let mut email = String::new();
        let mut password = String::new();

        println!("Enter your email: ");
        std::io::stdin().read_line(&mut email)?;

        let max_attempts = 3;
        let mut attempts = 0;

        while attempts < max_attempts {
            password = rpassword::prompt_password("Your password:")?;
            let repeat_password = rpassword::prompt_password("Repeat your password:")?;

            if password == repeat_password {
                break;
            } else {
                println!("Passwords do not match. Please try again");
                attempts += 1;
            }
        }

        if attempts == max_attempts {
            println!("You have reached the maximum number of attempts. Try again later");
            return Ok(());
        }

        let register_result = requests::user_register(
            &client,
            email.trim().to_string(),
            password.trim().to_string(),
        )
        .await;

        match register_result {
            Ok(_) => {
                println!("Registration successful");
            }
            Err(_) => {
                println!("Registration failed");
            }
        }
    }

    if args.login {
        let mut email = String::new();
        let mut password = String::new();

        println!("Enter your email: ");
        std::io::stdin().read_line(&mut email)?;
        println!("Enter your password: ");
        std::io::stdin().read_line(&mut password)?;

        let login_result = requests::user_login(&client, email, password).await;

        match login_result {
            Ok(_) => {
                println!("Login successful");
            }
            Err(_) => {
                println!("Incorrect email or password");
            }
        }
    }

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

    if let Some(note_id) = args.view {
        let note = utils::get_note(&note_id).await?;

        let body = note["body"].to_string();

        print!("{}", body);
    }

    if let Some(note_title) = args.create {
        let created_note = requests::create_note(&client, note_title, None).await?;

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
            requests::update_note(&client, note_id.parse::<u16>()?, Some(new_note_title), None)
                .await?;

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
            requests::delete_note(&client, note_id.parse::<u16>()?).await?;
            println!("Note {} {} is deleted", id, title);
        } else {
            println!("Note {} {} is not deleted", id, title);
        }
    }

    if let Some(note_id) = args.edit {
        let note = utils::get_note(&note_id).await?;

        let body = note["body"].to_string();

        let mut tmpfile = NamedTempFile::new()?;
        write!(tmpfile, "{}", body)?;

        std::process::Command::new("nano")
            .arg(tmpfile.path())
            .spawn()?
            .wait()?;

        tmpfile.seek(SeekFrom::Start(0))?;

        let mut buf = String::new();
        tmpfile.read_to_string(&mut buf)?;

        let _ = requests::update_note(&client, note_id.parse::<u16>()?, None, Some(buf)).await?;
    }

    Ok(())
}
