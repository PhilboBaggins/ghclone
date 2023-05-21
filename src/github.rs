use serde::Deserialize;

const MAX_REPOS_PER_PAGE: u32 = 100;

#[derive(Deserialize, Debug)]
pub struct RepoApiResponse {
    pub name: String,
    pub clone_url: String,
}

pub fn fetch_repo_list(username: &str) -> Result<Vec<RepoApiResponse>, ureq::Error> {
    let mut all_repos = Vec::new();
    let mut page_num = 1;

    loop {
        let url = format!("https://api.github.com/users/{}/repos?per_page={}&page={}", username, MAX_REPOS_PER_PAGE, page_num);
        //println!("GET {}", url);

        // TODO: Set accept header - "Accept: application/vnd.github.v3+json"
        let response: Vec<RepoApiResponse> = ureq::get(&url)
            .call()?
            .into_json()?;
        
        if response.len() == 0 {
            break;
        } else {
            for repo in response {
                all_repos.push(repo);
            }
        }

        page_num += 1;
    }

    Ok(all_repos)
}
