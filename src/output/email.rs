use std::collections::HashMap;

use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::EmailTransport;

use errors::*;
use output::{OutputMeta, OutputPlugin};
use types;

#[derive(Debug,Deserialize)]
pub struct EmailPlugin {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,

    pub from_address: String,
    pub from_name: String,

    pub to_address: String
}

impl OutputPlugin for EmailPlugin {
    fn new(config: &Option<HashMap<String, String>>) -> Result<Box<OutputPlugin>> {
        let mut config = config.to_owned()
            .ok_or("No config specified for Email plugin!")?;

        let smtp_server = config.remove("smtp_server")
            .ok_or("No SMTP server found")?.to_owned();
        let smtp_port = config.remove("smtp_port")
            .unwrap_or("25".to_owned())
            .parse::<u16>()
            .chain_err(|| "smtp_port must be an integer!")?;
        let smtp_username = config.remove("smtp_username")
            .unwrap_or("".to_owned());
        let smtp_password = config.remove("smtp_password")
            .unwrap_or("".to_owned());

        let from_address = config.remove("from_address")
            .ok_or("No FROM address found")?.to_owned();
        let from_name = config.remove("from_name")
            .unwrap_or("preminder - Github PR reminder".to_owned());

        let to_address = config.remove("to_address")
            .ok_or("No TO address found")?.to_owned();

        Ok(Box::new(EmailPlugin{
            smtp_server: smtp_server,
            smtp_port: smtp_port,
            smtp_username: smtp_username,
            smtp_password,

            from_address: from_address,
            from_name: from_name,

            to_address: to_address
        }))
    }

    fn remind(&self,
        _meta: &OutputMeta,
        _total: &Vec<types::PullRequest>,
        _created: &Vec<&types::PullRequest>,
        _updated: &Vec<&types::PullRequest>,
        _stale: &Vec<&types::PullRequest>
    ) -> Result<()> {
        let mail = EmailBuilder::new()
            .from((self.from_address.as_ref(), self.from_name.as_ref()))
            .to(self.to_address.as_ref())
            .subject("Hello from preminder!")
            .text("Hope you're having a good day!")
            .build()?;

        let mut mailer = SmtpTransportBuilder::new((self.smtp_server.as_ref(), 25))?
            .credentials(&self.smtp_username, &self.smtp_password)
            .security_level(SecurityLevel::Opportunistic)
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)
            .build();

        let result = mailer.send(mail.clone())?;

        println!("{:?}", result);

        mailer.close();

        Ok(())
    }
}
