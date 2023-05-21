use std::process::Command;
use dialoguer::{theme::ColorfulTheme, Input, FuzzySelect};

mod github;
use github::fetch_repo_list;

fn main() -> Result<(), ureq::Error> {

    // TODO: Ask user for Github username ... provide list of previously selected usernames and allow them to enter a new one if desired
    let username = "PhilboBaggins";

    println!("Fetching list of repositories for {}", username);
    let (names, clone_urls) = fetch_repo_list(username)?;

    let selection_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select repository")
        .default(0)
        .items(&names[..])
        .interact()
        .unwrap();

    let name = &names[selection_index];
    let clone_url = &clone_urls[selection_index];

    // Ask user for local directory name
    let local_dir: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Local path")
        .default(name.to_string())
        .interact_text()
        .unwrap();

    Command::new("git")
        .arg("clone")
        .arg(clone_url)
        .arg(local_dir)
        .spawn()
        .expect("failed to execute git clone")
        .wait()
        .expect("failed to wait on child");

    Ok(())
}
