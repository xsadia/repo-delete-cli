use core::fmt;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.github.com";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubRepository {
    id: i32,
    name: String,
}

impl fmt::Display for GithubRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.name, self.id)
    }
}

#[derive(Clone)]
pub struct App {
    username: String,
    token: String,
    repos: Vec<GithubRepository>,
    client: reqwest::Client,
}

impl App {
    pub fn new(username: String, token: String) -> Self {
        return Self {
            username,
            token,
            repos: vec![],
            client: reqwest::Client::new(),
        };
    }

    pub async fn init_repositories(&mut self) -> Result<(), reqwest::Error> {
        let repos = self
            .client
            .get(URL.to_string() + "/users/" + &self.username.as_str() + "/repos")
            .header("User-Agent", "Delete-cli")
            .query(&[("per_page", 100)])
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<Vec<GithubRepository>>()
            .await?;

        self.repos = repos;
        Ok(())
    }

    pub async fn delete_repository(&self, index: usize) -> Result<(), reqwest::Error> {
        let repo = self.repos.get(index).unwrap();
        self.client
            .delete(URL.to_string() + "/repos/" + self.username.as_str() + "/" + repo.name.as_str())
            .header("User-Agent", "Delete-cli")
            .bearer_auth(&self.token)
            .send()
            .await?;

        Ok(())
    }

    pub fn get_repositories(&self) -> &Vec<GithubRepository> {
        &self.repos
    }

    pub fn get_repository(&self, index: usize) -> std::option::Option<&GithubRepository> {
        self.repos.get(index)
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }
}
