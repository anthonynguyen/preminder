use std::collections::HashMap;

use handlebars;
use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::EmailTransport;

use duration;
use errors::*;
use output::{OutputMeta, OutputPlugin, handlebars_relative_helper};
use types;

const SUBJECT_TEMPLATE_NAME: &'static str = "subject";
const BODY_TEMPLATE_NAME: &'static str = "body";

const DEFAULT_SUBJECT_TEMPLATE: &'static str = "PR REMINDER";
const DEFAULT_BODY_TEMPLATE: &'static str = "Hello everyone!";

pub struct EmailPlugin {
    smtp_server: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,

    from_address: String,
    from_name: String,

    to_address: String,

    handlebar: handlebars::Handlebars
}

impl OutputPlugin for EmailPlugin {
    fn new(config: &Option<HashMap<String, String>>) -> Result<Box<OutputPlugin>> {
        let mut config = config.to_owned()
            .ok_or("No config specified for Email plugin!")?;

        let smtp_server = config.remove("smtp_server")
            .ok_or("No SMTP server found")?.to_owned();
        let smtp_port = config.remove("smtp_port")
            .unwrap_or_else(|| "25".to_owned())
            .parse::<u16>()
            .chain_err(|| "smtp_port must be an integer!")?;
        let smtp_username = config.remove("smtp_username")
            .unwrap_or_else(|| "".to_owned());
        let smtp_password = config.remove("smtp_password")
            .unwrap_or_else(|| "".to_owned());

        let from_address = config.remove("from_address")
            .ok_or("No FROM address found")?.to_owned();
        let from_name = config.remove("from_name")
            .unwrap_or_else(|| "preminder".to_owned());

        let to_address = config.remove("to_address")
            .ok_or("No TO address found")?.to_owned();

        let subject_template = config.remove("subject_template")
            .unwrap_or_else(|| DEFAULT_SUBJECT_TEMPLATE.to_owned());
        let body_template = config.remove("body_template")
            .unwrap_or_else(|| DEFAULT_BODY_TEMPLATE.to_owned());

        let mut handlebar = handlebars::Handlebars::new();
        handlebar.register_template_string(SUBJECT_TEMPLATE_NAME, subject_template)?;
        handlebar.register_template_string(BODY_TEMPLATE_NAME, body_template)?;
        handlebar.register_helper("relative", Box::new(handlebars_relative_helper));

        Ok(Box::new(EmailPlugin{
            smtp_server: smtp_server,
            smtp_port: smtp_port,
            smtp_username: smtp_username,
            smtp_password,

            from_address: from_address,
            from_name: from_name,

            to_address: to_address,

            handlebar: handlebar
        }))
    }

    fn remind(&self,
        meta: &OutputMeta,
        total: &[types::PullRequest],
        created: &[&types::PullRequest],
        updated: &[&types::PullRequest],
        stale: &[&types::PullRequest]
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

        let subject = self.handlebar.render(SUBJECT_TEMPLATE_NAME, &info)?;
        let body = self.handlebar.render(BODY_TEMPLATE_NAME, &info)?;

        let mail = EmailBuilder::new()
            .from((self.from_address.as_ref(), self.from_name.as_ref()))
            .to(self.to_address.as_ref())
            .subject(subject.as_ref())
            .html(body.as_ref())
            .build()?;

        let mut mailer = SmtpTransportBuilder::new((self.smtp_server.as_ref(), self.smtp_port))?
            .credentials(&self.smtp_username, &self.smtp_password)
            .security_level(SecurityLevel::Opportunistic)
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)
            .build();

        let result = mailer.send(mail.clone())?;

        info!("{:?}", result);

        mailer.close();

        Ok(())
    }
}
