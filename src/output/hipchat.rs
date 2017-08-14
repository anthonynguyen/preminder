use handlebars;
use regex;
use reqwest;

use std::collections::HashMap;
use std::io::Read;

use duration;
use errors::*;
use output::{OutputMeta, OutputPlugin, handlebars_relative_helper};
use types;

const TEMPLATE_NAME: &'static str = "hipchat";
const DEFAULT_TEMPLATE: &'static str = "Hello everyone!
    As of <em>{{ now }}</em>, there have been
    <strong>{{ num_opened }}</strong> pull requests opened, and
    <strong>{{ num_updated }}</strong> pull requests updated
    in the last {{ recent_period }}.

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
        handlebar.register_template_string(TEMPLATE_NAME, template)?;
        handlebar.register_helper("relative", Box::new(handlebars_relative_helper));

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
        total: &Vec<types::PullRequest>,
        created: &Vec<&types::PullRequest>,
        updated: &Vec<&types::PullRequest>,
        stale: &Vec<&types::PullRequest>
    ) -> Result<()> {
        let info = json!({
            "now": meta.now.format("%B %d, %l:%M%P").to_string(),
            "recent_period": duration::nice(meta.recent),
            "stale_period": duration::nice(meta.stale),

            "num_total": total.len(),
            "num_opened": created.len(),
            "num_updated": updated.len(),
            "num_stale": stale.len(),

            "opened": created,
            "updated": updated,
            "stale": stale
        });

        let message = self.handlebar.render(TEMPLATE_NAME, &info)?;

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
        let mut res = client.post(&self.url)?
            .header(reqwest::header::ContentType::json())
            .body(payload.to_string())
            .send()?;

        let mut content = String::new();
        res.read_to_string(&mut content)?;

        println!("{:?}", res);
        println!("{}", content);

        Ok(())
    }
}
