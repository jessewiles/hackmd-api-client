use clap::{Arg, ArgAction, Command};
use reqwest::blocking::Client;
use reqwest::header;
//use std::collections::HashMap;
use std::env;

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
                let request = client
                    .get("https://api.hackmd.io/v1/notes")
                    .header(header::AUTHORIZATION, format!("Bearer {}", api_key));

                let response = request.send().unwrap();
                println!("Sync response: {}", response.status());
            }
            Err(e) => println!("couldn't interpret HOME: {}", e),
        },
        Some(("push", _push_matches)) => {
            println!("Push response.");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
