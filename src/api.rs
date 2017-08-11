use reqwest;
use reqwest::header;
use serde;

use errors::*;
use types;

const GITHUB_API_VERSION: &'static str = "api/v3";

#[derive(Debug)]
pub struct Api {
    pub base_url: String,
    pub token: String,

    client: reqwest::Client,
}

impl Api {
    pub fn new(token: String, host: Option<String>) -> Result<Self> {
        let cl = reqwest::Client::new()?;
        let url = match host {
            Some(en) => format!("https://{}/{}", en, GITHUB_API_VERSION),
            _ => "https://api.github.com".to_owned()
        };

        Ok(Api {
            base_url: url,
            token: token.to_owned(),
            client: cl
        })
    }

    fn get_raw(&self, path: &str) -> Result<reqwest::Response> {
        let res = self.client.get(&format!("{}{}", self.base_url, path))?
            .header(header::Authorization(format!("token {}", self.token)))
            .send()?;

        match res.status() {
            reqwest::StatusCode::Ok => {}
            _ => return Err(Error::from(format!("Request failed: {:?}", res))),
        }

        Ok(res)
    }

    fn get<T>(&self, path: &str) -> Result<T>
        where T: serde::de::DeserializeOwned {
        let mut res = self.get_raw(path)?;
        res.json::<T>().chain_err(|| "Invalid response from API")
    }

    pub fn list_repos(&self, subject: &str) -> Result<Vec<types::Repository>> {
        self.get::<Vec<types::Repository>>(&format!("/users/{}/repos", subject))
            .chain_err(|| format!("Could not retrieve repositories for {}", subject))
    }

    pub fn list_pull_requests(&self, subject: &str) -> Result<Vec<types::PullRequest>> {
        self.get::<Vec<types::PullRequest>>(&format!("/repos/{}/pulls", subject))
            .chain_err(|| format!("Could not retrieve pull requests for {}", subject))
    }
}

