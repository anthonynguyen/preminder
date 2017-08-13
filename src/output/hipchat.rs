use handlebars;
use reqwest;

use std::collections::HashMap;

use duration;
use errors::*;
use output::{OutputMeta, OutputPlugin};
use types;

#[derive(Debug,Deserialize)]
pub struct HipchatPlugin {
    url: String,
    notify: bool,
    message_colour: String
}

impl OutputPlugin for HipchatPlugin {
    fn new(config: &Option<HashMap<String, String>>) -> Result<Box<OutputPlugin>> {
        let mut config = config.to_owned()
            .ok_or("No config specified for Hipchat Plugin")?;

        let base = config.remove("url")
            .ok_or("No Hipchat URL found")?.to_owned();
        let room = config.remove("room")
            .ok_or("No Hipchat room found")?.to_owned();
        let token = config.remove("token")
            .ok_or("No Hipchat token found")?.to_owned();

        let message_colour = config.remove("colour")
            .unwrap_or("yellow".to_owned());
        let notify: bool = config.remove("notify")
            .unwrap_or("false".to_owned())
            .parse::<bool>()
            .chain_err(|| "Valid values for 'notify' are `true` and `false`")?;

        let url = format!("{}/v2/room/{}/notification?auth_token={}",
            base, room, token);

        Ok(Box::new(HipchatPlugin{
            url: url,
            notify: notify,
            message_colour: message_colour.to_owned()
        }))
    }

    fn remind(&self,
        meta: &OutputMeta,
        _total: &Vec<types::PullRequest>,
        created: &Vec<&types::PullRequest>,
        updated: &Vec<&types::PullRequest>
    ) -> Result<()> {
        let info = json!({
            "now": meta.now.format("%B %d, %l:%M%P").to_string(),
            "period": duration::nice(meta.period),
            "num_opened": created.len(),
            "num_updated": updated.len()
        });

        let reg = handlebars::Handlebars::new();
        let message = reg.template_render("Hello everyone! \
            As of <em>{{ now }}</em>, there have been \
            <strong>{{ num_opened }}</strong> pull requests opened, and \
            <strong>{{ num_updated }}</strong> pull requests updated \
            in the last {{ period }}.", &info)?;

        let payload = json!({
            "color": self.message_colour,
            "notify": self.notify,
            "message_format": "html",
            "message": message
        });

        let client = reqwest::Client::new()?;
        let res = client.post(&self.url)?
            .header(reqwest::header::ContentType::json())
            .body(payload.to_string())
            .send()?;

        println!("{:?}", res);

        Ok(())
    }
}
