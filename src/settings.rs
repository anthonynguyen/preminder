use std::collections::HashMap;

use config;
use output;

use errors::*;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub github: GithubSettings,

    pub recent: String,
    pub stale: String,

    pub template: Option<HashMap<String, String>>,
    pub outputs: Vec<output::OutputBlock>,
}

#[derive(Debug, Deserialize)]
pub struct GithubSettings {
    pub token: String,
    pub host: Option<String>,
    pub subjects: Vec<String>,
}

impl Settings {
    pub fn new(config_path: Option<&str>) -> Result<Self> {
        let mut settings = config::Config::default();
        settings.merge(config::Environment::new())?;

        if let Some(path) = config_path {
            settings.merge(config::File::with_name(path))?;
        }

        settings.try_into().chain_err(
            || "Could not load configuration!",
        )
    }
}
