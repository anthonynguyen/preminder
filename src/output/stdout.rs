use std::collections::HashMap;

use errors::*;
use output::{OutputData, OutputMeta, OutputPlugin};

#[derive(Clone,Debug,Deserialize)]
pub struct Config {
    template: String
}

#[derive(Debug,Deserialize)]
pub struct Plugin {
    config: Config
}

impl OutputPlugin for Plugin {
    fn remind(&self,
        _meta: &OutputMeta,
        _data: &OutputData,
        templated: &HashMap<String, String>
    ) -> Result<()> {
        println!("{}", templated.get(&self.config.template).unwrap());
        Ok(())
    }
}

pub fn new(config: &Config) -> Box<OutputPlugin> {
    Box::new(Plugin { config: config.clone() })
}
