use crate::query_helper;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use serde_json::{Result, Value};

use std::collections::HashMap;

pub const GITHUB_QUERY_STRING: &'static str =
    "https://api.github.com/search/repositories?q=language:ats+created:>";

pub static BOT_USER_AGENT: &'static str = "NewATSRepos";

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub id: usize,
    pub login: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub id: usize,
    pub name: String,
    pub html_url: String,
    pub owner: Owner,
    pub description: Option<String>,
    pub created_at: String,
}

/// Retrieve the latest list of ATS repos since yesterday (24-hour period)
pub fn get_new_ats_repos() -> Option<Result<Vec<Repo>>> {
    info!("Querying Github for new ATS repos...");

    if let Some(query_since_date_str) = query_helper::check_query_viability() {
        let query_url = format!("{}{}&sort=asc", GITHUB_QUERY_STRING, query_since_date_str);
        let client = reqwest::blocking::Client::new();

        let repo_data = client
            .get(&query_url)
            .header(USER_AGENT, BOT_USER_AGENT)
            .send()
            .unwrap()
            .text()
            .unwrap();

        let repos: Result<HashMap<String, Value>> = serde_json::from_str(&repo_data);
        if let Ok(parsed_repos) = repos {
            let items: Vec<Repo> =
                serde_json::from_value(parsed_repos["items"].clone()).unwrap_or(vec![]);
            info!("Found {} new ATS repos", items.len());
            return Some(Ok(items));
        }
        return None;
    }

    info!("No newly-created ATS repos found...");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retrieval_of_new_ats_repos() {
        let new_repos = get_new_ats_repos();

        for repo in new_repos {
            println!("{:#?}", repo);
        }
    }
}
