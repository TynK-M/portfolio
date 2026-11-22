use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

use crate::config;

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub open_issues_count: u64,
    pub updated_at: String,
}

pub fn fetch_repositories(names: &[&str]) -> Result<Vec<Repository>, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent("portfolio-generator")
        .build()?;

    let mut repositories = Vec::with_capacity(names.len());

    for name in names {
        let url = format!(
            "https://api.github.com/repos/{username}/{name}",
            username = config::GITHUB_USERNAME,
            name = name
        );

        let repository = client
            .get(url)
            .send()?
            .error_for_status()?
            .json::<Repository>()?;

        repositories.push(repository);
    }

    Ok(repositories)
}
