use std::collections::HashMap;

use tera;

use duration;
use errors::*;
use output::{OutputData, OutputMeta, OutputPlugin};
use types;

#[derive(Debug,Deserialize)]
pub struct StdoutPlugin {
    template_name: String
}

impl OutputPlugin for StdoutPlugin {
    fn new(config: &Option<HashMap<String, String>>,
        templates: &Vec<String>) -> Result<Box<OutputPlugin>> {
        let mut config = config.to_owned()
            .ok_or("No config specified for Stdout Plugin")?;

        let template_name = config.remove("template")
            .ok_or("No `template` found")?.to_owned();

        if !templates.contains(&template_name) {
            return Err(format!("No `{}` template found!", template_name).into())
        }

        Ok(Box::new(StdoutPlugin {
            template_name: template_name
        }))
    }

    fn remind(&self,
        _meta: &OutputMeta,
        _data: &OutputData,
        templated: &HashMap<String, String>
    ) -> Result<()> {
        info!("hi");

        println!("{}", templated.get(&self.template_name).unwrap());

        Ok(())
    }
}
