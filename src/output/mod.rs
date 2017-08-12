pub mod stdout;

use std::collections::HashMap;

use errors::*;
use settings::{OutputBlock, Settings};
use types;

#[derive(Debug,Deserialize)]
pub enum OutputPlugins {
    Stdout
}

pub trait OutputPlugin {
    fn new(config: &HashMap<String, String>) -> Result<Box<OutputPlugin>> where Self:Sized;

    fn remind(&self,
        settings: &Settings,
        total: &Vec<types::PullRequest>,
        created: &Vec<&types::PullRequest>,
        updated: &Vec<&types::PullRequest>
    ) -> ();
}

pub fn init(configured: &Vec<OutputBlock>) -> Result<Vec<Box<OutputPlugin>>> {
    let mut plugins: Vec<Box<OutputPlugin>> = Vec::new();

    for output in configured {
        let plugin = match output._type.as_ref() {
            "stdout" => stdout::StdoutPlugin::new(&output.config)?,
            _ => return Err(Error::from(format!("Invalid plugin name: {} ", output._type)))
        };

        plugins.push(plugin);
    }

    Ok(plugins)
}
