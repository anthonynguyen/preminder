use regex;
use reqwest;

use std::collections::HashMap;
use std::io::Read;

use errors::*;
use output::{OutputData, OutputMeta, OutputPlugin};

pub struct HipchatPlugin {
    url: String,
    notify: bool,
    message_colour: String,
    from: String,
    max_results: usize,
    template_name: String
}

impl OutputPlugin for HipchatPlugin {
    fn new(config: &Option<HashMap<String, String>>,
        templates: &[String]) -> Result<Box<OutputPlugin>> {
        let mut config = config.to_owned()
            .ok_or("No config specified for Hipchat Plugin")?;

        let base = config.remove("url")
            .ok_or("No Hipchat URL found")?.to_owned();
        let room = config.remove("room")
            .ok_or("No Hipchat room found")?.to_owned();
        let token = config.remove("token")
            .ok_or("No Hipchat token found")?.to_owned();

        let max_results = config.remove("max_results")
            .unwrap_or_else(|| "0".to_owned())
            .parse::<usize>()
            .chain_err(|| "smtp_port must be an integer!")?;

        let from = config.remove("from")
            .unwrap_or_else(|| "Github PR reminder".to_owned());
        let message_colour = config.remove("colour")
            .unwrap_or_else(|| "yellow".to_owned());
        let notify = config.remove("notify")
            .unwrap_or_else(|| "false".to_owned())
            .parse::<bool>()
            .chain_err(|| "Valid values for 'notify' are `true` and `false`")?;

        let template_name = config.remove("template")
            .ok_or("No `template` found")?.to_owned();
        if !templates.contains(&template_name) {
            return Err(format!("No `{}` template found!", template_name).into())
        }

        let url = format!("{}/v2/room/{}/notification?auth_token={}",
            base, room, token);

        Ok(Box::new(HipchatPlugin{
            url: url,
            notify: notify,
            message_colour: message_colour.to_owned(),
            from: from,
            max_results: max_results,
            template_name: template_name
        }))
    }

    // https://www.hipchat.com/docs/apiv2/method/send_room_notification
    fn remind(&self,
        _meta: &OutputMeta,
        _data: &OutputData,
        templated: &HashMap<String, String>
    ) -> Result<()> {
        let message = templated.get(&self.template_name).unwrap();

        let re = regex::Regex::new(r"\s+")?;
        let message = re.replace_all(message, " ").to_string();

        let client = reqwest::Client::new()?;

        for chunk in message.into_bytes().chunks(10_000) {
            let payload = json!({
                "from": self.from,
                "color": self.message_colour,
                "notify": self.notify,
                "message_format": "html",
                "message": String::from_utf8(chunk.to_vec())?
            });

            let mut res = client.post(&self.url)?
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
