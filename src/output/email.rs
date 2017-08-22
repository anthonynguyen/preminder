use std::collections::HashMap;

use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::EmailTransport;

use errors::*;
use output::{OutputData, OutputMeta, OutputPlugin};

pub struct EmailPlugin {
    smtp_server: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,

    from_address: String,
    from_name: String,

    to_address: String,

    subject_template_name: String,
    body_template_name: String
}

impl OutputPlugin for EmailPlugin {
    fn new(config: &Option<HashMap<String, String>>,
        templates: &Vec<String>) -> Result<Box<OutputPlugin>> {
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

        let subject_template_name = config.remove("subject_template")
            .ok_or("No `subject_template` found")?.to_owned();
        if !templates.contains(&subject_template_name) {
            return Err(format!("No `{}` template found!", subject_template_name).into())
        }

        let body_template_name = config.remove("body_template")
            .ok_or("No `body_template` found")?.to_owned();
        if !templates.contains(&body_template_name) {
            return Err(format!("No `{}` template found!", body_template_name).into())
        }

        Ok(Box::new(EmailPlugin{
            smtp_server: smtp_server,
            smtp_port: smtp_port,
            smtp_username: smtp_username,
            smtp_password,

            from_address: from_address,
            from_name: from_name,

            to_address: to_address,

            subject_template_name: subject_template_name,
            body_template_name: body_template_name
        }))
    }

    fn remind(&self,
        _meta: &OutputMeta,
        _data: &OutputData,
        templated: &HashMap<String, String>
    ) -> Result<()> {
        let subject = templated.get(&self.subject_template_name).unwrap();
        let body = templated.get(&self.body_template_name).unwrap();

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
