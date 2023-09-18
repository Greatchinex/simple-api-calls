use reqwest::header::USER_AGENT;
use reqwest::{get, Client};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct User {
    login: String,
    id: u32,
}

#[tokio::main]
pub async fn async_api_request() -> Result<String, anyhow::Error> {
    let res_body = get("http://httpbin.org/get").await?.text().await?;

    Ok(res_body)
}

#[tokio::main]
pub async fn github_stargazers() -> Result<Vec<User>, anyhow::Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    println!("RequestUrl: {}", request_url);
    let client = Client::new();
    let res_data = client
        .get(&request_url)
        .header(USER_AGENT, "Rust web-api-client")
        .send()
        .await?;
    let users: Vec<User> = res_data.json().await?;

    Ok(users)
}
