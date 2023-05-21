use serde::Deserialize;

const MAX_REPOS_PER_PAGE: u32 = 100;

#[derive(Deserialize, Debug)]
pub struct RepoApiResponse {
    pub name: String,
    pub clone_url: String,
}

pub fn fetch_repo_list(username: &str) -> Result<(Vec<String>, Vec<String>), ureq::Error> {
    let mut names = Vec::new();
    let mut clone_urls = Vec::new();
    let mut page_num = 1;

    loop {
        let url = format!(
            "https://api.github.com/users/{}/repos?per_page={}&page={}",
            username, MAX_REPOS_PER_PAGE, page_num
        );
        //println!("GET {}", url);

        let response: Vec<RepoApiResponse> = ureq::get(&url)
            .set("Accept", "application/vnd.github.v3+json")
            .call()?
            .into_json()?;

        if response.is_empty() {
            break;
        } else {
            for repo in response {
                names.push(repo.name);
                clone_urls.push(repo.clone_url);
            }
        }

        page_num += 1;
    }

    Ok((names, clone_urls))
}
