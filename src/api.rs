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
    pub fn new(token: &str, host: &Option<String>) -> Result<Self> {
        let cl = reqwest::Client::new()?;

        let url = match host.clone() {
            Some(en) => format!("https://{}/{}", en, GITHUB_API_VERSION),
            _ => "https://api.github.com".to_owned()
        };

        Ok(Api {
            base_url: url,
            token: token.to_owned(),
            client: cl
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn get_raw(&self, url: &str) -> Result<reqwest::Response> {
        let res = self.client.get(url)?
            .header(header::Authorization(format!("token {}", self.token)))
            .send()?;

        match res.status() {
            reqwest::StatusCode::Ok => {}
            _ => return Err(Error::from(format!("Request failed: {:?}", res))),
        }

        Ok(res)
    }

    fn get_pages<T: serde::de::DeserializeOwned>(&self, initial_path: &str) -> Result<Vec<T>> {
        let mut res = self.get_raw(&self.url(initial_path))?;
        let mut list: Vec<T> = Vec::new();

        loop {
            list.append(&mut res.json::<Vec<T>>()
                .chain_err(|| format!("Invalid response from API({}): {}",
                    res.status(), res.url()))?);

            res = {
                // Look for a Link header with rel=next, and keep following it
                // There must be a cleaner way to structure this code...
                let link = res.headers()
                    .get::<reqwest::header::Link>()
                    .and_then(|lv| lv.values().iter().find(|lv| {
                        lv.rel().and_then(|rels| rels.iter().find(|rel| {
                            **rel == reqwest::header::RelationType::Next
                        })).is_some()
                    }));

                match link {
                    None => break,
                    Some(lv) => self.get_raw(lv.link())?
                }
            }
        }

        Ok(list)
    }

    pub fn list_repos(&self, subject: &str) -> Result<Vec<types::Repository>> {
        self.get_pages::<types::Repository>(&format!("/users/{}/repos", subject))
            .chain_err(|| format!("Could not retrieve repositories for {}", subject))
    }

    pub fn list_pull_requests(&self, subject: &str) -> Result<Vec<types::PullRequest>> {
        self.get_pages::<types::PullRequest>(&format!("/repos/{}/pulls", subject))
            .chain_err(|| format!("Could not retrieve pull requests for {}", subject))
    }
}

