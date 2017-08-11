use config;

use errors::*;

#[derive(Debug,Deserialize)]
pub struct Settings {
    pub github_api_token: String,
    pub github_host: Option<String>,
    pub github_subject: String
}

pub fn load() -> Result<Settings> {
	let mut env = config::Environment::new();
	// Set this to something other than _ and we're good to go
	env.separator("/".to_owned());

    let mut settings = config::Config::default();
    settings.merge(env)?;

    Ok(settings.try_into::<Settings>()?)
}
