use config;

use errors::*;

#[derive(Debug,Deserialize)]
pub struct Settings {
    pub github_api_token: String,
    pub github_host: Option<String>,
    pub github_subject: String
}

impl Settings {
    pub fn new() -> Result<Self> {
        let mut env = config::Environment::new();
        // Set this to something other than _ and we're good to go
        env.separator("/".to_owned());

        let mut settings = config::Config::default();
        settings.merge(env)?;

        settings.try_into().chain_err(|| "Could not load configuration!")
    }
}
