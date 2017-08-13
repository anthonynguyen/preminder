use chrono;
use handlebars;
use regex;
use reqwest;

use std;
use std::collections::HashMap;

use duration;
use errors::*;
use output::{OutputMeta, OutputPlugin};
use types;

const DEFAULT_TEMPLATE: &'static str = "Hello everyone!
    As of <em>{{ now }}</em>, there have been
    <strong>{{ num_opened }}</strong> pull requests opened, and
    <strong>{{ num_updated }}</strong> pull requests updated
    in the last {{ recent }}.

    <br /><br />

    <strong>Recently opened pull requests:</strong>
    <ul>
        {{ #each opened }}
            <li>
                [<strong><a href=\"{{ this.html_url }}\">{{ this.base.repo.full_name }}#{{ this.number }}</a></strong>]
                {{ this.title }}
                &bull;
                <a href=\"{{ this.user.html_url }}\">{{ this.user.login }}</a>
                &bull;
                <em>{{ relative this.created_at }}</em>
            </li>
        {{ /each }}
    </ul>

    <br />

    <strong>Recently updated pull requests:</strong>
    <ul>
        {{ #each updated }}
            <li>
                [<strong><a href=\"{{ this.html_url }}\">{{ this.base.repo.full_name }}#{{ this.number }}</a></strong>]
                {{ this.title }}
                &bull;
                <a href=\"{{ this.user.html_url }}\">{{ this.user.login }}</a>
                &bull;
                <em>{{ relative this.updated_at }}</em>
            </li>
        {{ /each }}
    </ul>";

pub struct HipchatPlugin {
    url: String,
    notify: bool,
    message_colour: String,
    from: String,
    handlebar: handlebars::Handlebars
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

        let from = config.remove("from")
            .unwrap_or("Github PR reminder".to_owned());
        let message_colour = config.remove("colour")
            .unwrap_or("yellow".to_owned());
        let notify = config.remove("notify")
            .unwrap_or("false".to_owned())
            .parse::<bool>()
            .chain_err(|| "Valid values for 'notify' are `true` and `false`")?;
        let template = config.remove("template")
            .unwrap_or(DEFAULT_TEMPLATE.to_owned());

        let url = format!("{}/v2/room/{}/notification?auth_token={}",
            base, room, token);

        let mut handlebar = handlebars::Handlebars::new();
        let relative_helper = |helper: &handlebars::Helper,
            _: &handlebars::Handlebars,
            rc: &mut handlebars::RenderContext
        | -> std::result::Result<(), handlebars::RenderError> {
            let param = helper.param(0)
                .ok_or(handlebars::RenderError::new("No param given?"))?
                .value()
                .as_str()
                .ok_or(handlebars::RenderError::new("Param is not a string"))?
                .parse::<chrono::DateTime<chrono::Utc>>()
                .map_err(|_| handlebars::RenderError::new("Param could not be parsed as a datetime"))?
                .with_timezone::<chrono::offset::Local>(&chrono::offset::Local);

            let now = chrono::Local::now();
            let fin = duration::relative::<chrono::offset::Local>(param, now);

            rc.writer.write(&fin.into_bytes())?;
            Ok(())
        };

        handlebar.register_template_string("hipchat", template)?;
        handlebar.register_helper("relative", Box::new(relative_helper));

        Ok(Box::new(HipchatPlugin{
            url: url,
            notify: notify,
            message_colour: message_colour.to_owned(),
            from: from,
            handlebar: handlebar
        }))
    }

    // https://www.hipchat.com/docs/apiv2/method/send_room_notification
    fn remind(&self,
        meta: &OutputMeta,
        _total: &Vec<types::PullRequest>,
        created: &Vec<&types::PullRequest>,
        updated: &Vec<&types::PullRequest>
    ) -> Result<()> {
        let info = json!({
            "now": meta.now.format("%B %d, %l:%M%P").to_string(),
            "recent": duration::nice(meta.recent),
            "num_opened": created.len(),
            "num_updated": updated.len(),

            "opened": created,
            "updated": updated
        });

        let message = self.handlebar.render("hipchat", &info)?;

        let re = regex::Regex::new(r"\s+")?;
        let message = re.replace_all(&message, " ");

        let payload = json!({
            "from": self.from,
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
