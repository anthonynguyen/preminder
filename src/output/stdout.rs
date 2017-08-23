use std::collections::HashMap;

use errors::*;
use output::{OutputData, OutputMeta, OutputPlugin};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    template: String,
}

#[derive(Debug, Deserialize)]
pub struct Plugin {
    config: Config,
}

impl OutputPlugin for Plugin {
    fn check_templates(&self, templates: &[String]) -> Result<()> {
        if !templates.contains(&self.config.template) {
            return Err(
                format!("Stdout template missing: {}", self.config.template).into(),
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
        println!("{}", templated.get(&self.config.template).unwrap());
        Ok(())
    }
}

impl Plugin {
    pub fn init(config: &Config) -> Result<Box<OutputPlugin>> {
        Ok(Box::new(Plugin { config: config.clone() }))
    }
}
