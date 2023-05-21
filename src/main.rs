mod github;
use github::fetch_repo_list;

fn main() -> Result<(), ureq::Error> {
    let username = "PhilboBaggins";
    let response = fetch_repo_list(username)?;

    println!("## Received list of {} repos", response.len());
    for (idx, repo) in response.iter().enumerate() {
        println!("{}. {}", idx + 1, repo.clone_url);
    }

    Ok(())
}
