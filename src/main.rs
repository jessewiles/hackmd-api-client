use clap::{Arg, ArgAction, Command};
use reqwest::blocking::Client;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Serialize)]
struct HmdNote {
    id: String,
    title: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct HmdNoteContent {
    id: String,
    content: String,
    title: String,
}

fn main() {
    let matches = Command::new("hackmdio")
        .about("HACKMD.io client")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Jesse Wiles")
        // Push subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("push")
                .short_flag('U')
                .long_flag("push")
                .about("Push a note to hackmd.io.")
                .arg(
                    Arg::new("id")
                        .short('i')
                        .long("id")
                        .help("ID of the note to push")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("path")
                        .long("path")
                        .short('p')
                        .help("view package information")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Synchronize notes."),
        )
        .get_matches();

    let client = Client::new();

    match matches.subcommand() {
        Some(("sync", _sync_matches)) => match env::var("HACKMD_API_KEY") {
            Ok(api_key) => {
                let hmd_dir = check_hmd_dir().unwrap();
                let notes_seg = PathBuf::from("notes.json");
                let notes_path = hmd_dir.join(notes_seg);

                let request = client
                    .get("https://api.hackmd.io/v1/notes")
                    .header(header::AUTHORIZATION, format!("Bearer {}", api_key));

                let response = request.send();
                let notes: Vec<HmdNote> = response.unwrap().json().unwrap();
                let mut file = File::create(notes_path).unwrap();
                let out = serde_json::to_string_pretty(&notes).unwrap();
                file.write_all(out.as_bytes()).unwrap();

                for note in notes {
                    let note_path = hmd_dir.join(format!("{0}.md", note.id));
                    let note_request = client
                        .get(format!("https://api.hackmd.io/v1/notes/{0}", note.id))
                        .header(header::AUTHORIZATION, format!("Bearer {}", api_key));
                    let note_response = note_request.send();
                    let note: HmdNoteContent = note_response.unwrap().json().unwrap();
                    let mut note_file = File::create(note_path).unwrap();
                    note_file.write_all(note.content.as_bytes()).unwrap();
                }
            }
            Err(e) => println!("couldn't find HACKMD_API_KEY in the environment: {}", e),
        },
        Some(("push", _push_matches)) => {
            println!("Push response.");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}

fn check_hmd_dir() -> Result<PathBuf, String> {
    match env::var("HOME") {
        Ok(home_dir) => {
            let home_path = PathBuf::from(home_dir);
            let hmd_seg = PathBuf::from(".hackmdio");
            let hmd_dir = home_path.join(hmd_seg);
            match fs::metadata(hmd_dir.clone()) {
                Ok(_) => {}
                Err(_) => match fs::create_dir(hmd_dir.clone()) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                },
            }
            return Ok(hmd_dir);
        }
        Err(_) => todo!(),
    }
}
