use std::process::Command;
use dialoguer::{theme::ColorfulTheme, Input, FuzzySelect};

mod github;
use github::fetch_repo_list;

fn main() -> Result<(), ureq::Error> {
    // Ask user for Github username 
    let username = ask_for_username()?;

    // Fetch list of user's repositories
    println!("Fetching list of repositories for {}...", username);
    let (names, clone_urls) = fetch_repo_list(&username)?;

    // Select repository
    let selection_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select repository")
        .default(0)
        .items(&names[..])
        .interact()?;
    let name = &names[selection_index];
    let clone_url = &clone_urls[selection_index];

    // Ask user for local directory name
    let local_dir: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Local path")
        .default(name.to_string())
        .interact_text()?;

    // Git clone
    Command::new("git")
        .arg("clone")
        .arg(clone_url)
        .arg(local_dir)
        .spawn()
        .expect("failed to execute git clone")
        .wait()?;

    Ok(())
}

fn ask_for_username() -> Result<String, std::io::Error> {
    // TODO: Read list of previous usernames
    let prev_usernames = vec!["PhilboBaggins"];

    let username = match prev_usernames.len() {
        0 => {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Github username")
                .interact_text()?
                //.unwrap()
        },
        1 => {
            let prev_username = *prev_usernames.first().unwrap();
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Github username")
                .default(prev_username.to_string())
                .interact_text()?
        },
        _ => {
            // TODO: Present list of previous usernames and allow user select one or enter a new one
            prev_usernames.first().unwrap().to_string()
        },
    };
    
    // TODO: If `username` not in `prev_usernames` list; add it and save to config file
    
    Ok(username)
}
