use mini_redis::{client, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;
use tokio;

#[derive(Deserialize, Debug)]
struct SearchResult {
    items: Vec<Repository>,
}

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    stargazers_count: i32,
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = Client::new();

        let response = client
            .get("https://api.github.com/search/repositories?q=cumulus+in:dependencies")
            .send()
            .await
            .unwrap();

        let text = response.text().await.unwrap();

        let result: SearchResult = from_str(&text).unwrap();

        let repositories = result
            .items
            .iter()
            .map(|repo| (repo.stargazers_count, repo.name.clone()))
            .collect::<Vec<(i32, String)>>();

        let sorted_repositories = repositories
            .into_iter()
            .filter(|(stars, _)| *stars > 0)
            .collect::<Vec<(i32, String)>>();

        for (stars, repo_name) in sorted_repositories {
            println!("{} stars: {}", stars, repo_name);
        }
    });
}
