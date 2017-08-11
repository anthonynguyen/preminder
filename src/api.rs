use std::io::Read;

use reqwest;
use reqwest::header;
use serde;
use serde_json;

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

    fn get_raw(&self, path: &str) -> Result<String> {
        let mut res = self.client.get(&format!("{}{}", self.base_url, path))?
            .header(header::Authorization(format!("token {}", self.token)))
            .send()?;

        match res.status() {
            reqwest::StatusCode::Ok => {}
            _ => return Err(Error::from(format!("Request failed: {:?}", res))),
        }

        let mut body = String::new();
        res.read_to_string(&mut body)?;

        Ok(body)
    }

    fn get<T>(&self, path: &str) -> Result<T>
        where T: serde::de::DeserializeOwned {
        let raw = self.get_raw(path)?;
        let parsed: T = serde_json::from_str(&raw)?;
        Ok(parsed)
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

