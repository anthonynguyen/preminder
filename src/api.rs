use std::io::Read;

use reqwest;
use reqwest::header;
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


    fn get_path(&self, path: &str) -> Result<String> {
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

    pub fn list_repos(&self, subject: String) -> Result<Vec<types::Repository>> {
        let data = self.get_path(&format!("/users/{}/repos", subject))?;
        let parsed: Vec<types::Repository> = serde_json::from_str(&data)?;
        Ok(parsed)
    }
}

