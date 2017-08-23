use std::collections::HashMap;

use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::EmailTransport;

use errors::*;
use output::{OutputData, OutputMeta, OutputPlugin};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    smtp_server: String,
    #[serde(default = "default_smtp_port")]
    smtp_port: u16,
    #[serde(default = "default_smtp_username")]
    smtp_username: String,
    #[serde(default = "default_smtp_password")]
    smtp_password: String,

    from_address: String,
    from_name: String,

    to_address: String,

    subject_template: String,
    body_template: String,
}

fn default_smtp_port() -> u16 {
    25
}

fn default_smtp_username() -> String {
    "".to_string()
}

fn default_smtp_password() -> String {
    "".to_string()
}

pub struct Plugin {
    config: Config,
}

impl OutputPlugin for Plugin {
    fn check_templates(&self, templates: &[String]) -> Result<()> {
        if !templates.contains(&self.config.subject_template) {
            return Err(
                format!(
                    "Email subject_template missing: {}",
                    self.config.subject_template
                ).into(),
            );
        }

        if !templates.contains(&self.config.body_template) {
            return Err(
                format!("Email body_template missing: {}", self.config.body_template).into(),
            );
        }

        Ok(())
    }

    fn remind(
        &self,
        _meta: &OutputMeta,
        _data: &OutputData,
        templated: &HashMap<String, String>,
    ) -> Result<()> {
        let subject = templated.get(&self.config.subject_template).unwrap();
        let body = templated.get(&self.config.body_template).unwrap();

        let mail = EmailBuilder::new()
            .from((
                self.config.from_address.as_ref(),
                self.config.from_name.as_ref(),
            ))
            .to(self.config.to_address.as_ref())
            .subject(subject.as_ref())
            .html(body.as_ref())
            .build()?;

        let mut mailer =
            SmtpTransportBuilder::new((self.config.smtp_server.as_ref(), self.config.smtp_port))?
                .credentials(&self.config.smtp_username, &self.config.smtp_password)
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

impl Plugin {
    pub fn init(config: &Config) -> Result<Box<OutputPlugin>> {
        Ok(Box::new(Plugin { config: config.clone() }))
    }
}
