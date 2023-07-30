use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize)]
struct NewRepo<'a> {
    name: &'a str,
    description: &'a str,
    private: bool,
}

#[derive(Deserialize, Debug)]
struct RepoResponse {
    html_url: String,
}

async fn create_github_repo(token: &str, new_repo: NewRepo<'_>) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", token))?,
    );

    let response = client
        .post("https://api.github.com/user/repos")
        .headers(headers)
        .json(&new_repo)
        .send()
        .await?;

    // Check if GitHub returned a successful status code
    if response.status().is_success() {
        let repo: RepoResponse = response.json().await?;
        println!("Created new repository at: {}", repo.html_url);
    } else {
        // If not, print the response status and text for debugging
        let status = response.status();
        let text = response.text().await?;
        println!("Failed to create repository: status {}, text: {}", status, text);
    }

    Ok(())
}

fn get_user_input(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let repo_name = get_user_input("Enter the name of the new repository: ");
    let repo_description = get_user_input("Enter a description for the new repository: ");

    let new_repo = NewRepo {
        name: &repo_name,
        description: &repo_description,
        private: false,
    };

    create_github_repo(&token, new_repo).await?;

    Ok(())
}
