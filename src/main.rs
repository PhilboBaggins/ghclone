use dialoguer::{theme::ColorfulTheme, FuzzySelect};

mod github;
use github::fetch_repo_list;

fn main() -> Result<(), ureq::Error> {
    let username = "PhilboBaggins";

    println!("Fetching list of repositories for {}", username);
    let (names, clone_urls) = fetch_repo_list(username)?;

    let selection_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select repository")
        .default(0)
        .items(&names[..])
        .interact()
        .unwrap();

    println!("TODO: git clone {}", clone_urls[selection_index]);

    Ok(())
}
