use regex;
use reqwest;

use std::collections::HashMap;
use std::io::Read;

use errors::*;
use output::{OutputData, OutputMeta, OutputPlugin};

#[derive(Clone,Debug,Deserialize)]
pub struct Config {
    url: String,
    room: u16,
    token: String,
    from: String,

    #[serde(default)] notify: bool,
    #[serde(default = "default_colour")] colour: String,

    template: String
}

fn default_colour() -> String {
    "yellow".to_string()
}

pub struct Plugin {
    full_url: String,
    config: Config
}

impl OutputPlugin for Plugin {
    fn check_templates(&self, templates: &[String]) -> Result<()> {
        if !templates.contains(&self.config.template) {
            return Err(format!("Hipchat template missing: {}", self.config.template).into());
        }

        Ok(())
    }

    // https://www.hipchat.com/docs/apiv2/method/send_room_notification
    fn remind(&self,
        _meta: &OutputMeta,
        _data: &OutputData,
        templated: &HashMap<String, String>
    ) -> Result<()> {
        let message = templated.get(&self.config.template).unwrap();

        let re = regex::Regex::new(r"\s+")?;
        let message = re.replace_all(message, " ").to_string();

        let client = reqwest::Client::new()?;

        for chunk in message.into_bytes().chunks(10_000) {
            let payload = json!({
                "from": self.config.from,
                "color": self.config.colour,
                "notify": self.config.notify,
                "message_format": "html",
                "message": String::from_utf8(chunk.to_vec())?
            });

            let mut res = client.post(&self.full_url)?
                .header(reqwest::header::ContentType::json())
                .body(payload.to_string())
                .send()?;

            let mut content = String::new();
            res.read_to_string(&mut content)?;

            info!("{:?}", res);
            info!("{}", content);
        }

        Ok(())
    }
}

impl Plugin {
    pub fn new(config: &Config) -> Result<Box<OutputPlugin>> {
        let full_url = format!("{}/v2/room/{}/notification?auth_token={}",
            config.url, config.room, config.token);

        Ok(Box::new(Plugin {
            config: config.clone(),
            full_url
        }))
    }
}

