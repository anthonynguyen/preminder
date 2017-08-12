use std::collections::HashMap;

mod stdout;

use errors::*;
use settings::{OutputBlock, Settings};
use types;

#[derive(Debug,Deserialize)]
pub enum OutputPlugins {
    Stdout
}

pub trait OutputPlugin {
    fn new(config: &Option<HashMap<String, String>>)
        -> Result<Box<OutputPlugin>> where Self:Sized;

    fn remind(&self,
        settings: &Settings,
        total: &Vec<types::PullRequest>,
        created: &Vec<&types::PullRequest>,
        updated: &Vec<&types::PullRequest>)
        -> ();
}

pub fn init(configured: &Vec<OutputBlock>) -> Result<Vec<Box<OutputPlugin>>> {
    let mut plugins: Vec<Box<OutputPlugin>> = Vec::new();

    for output in configured {
        let plugin = match output._type.as_ref() {
            "stdout" => stdout::StdoutPlugin::new(&output.config)?,
            _ => return Err(Error::from(format!("Invalid output type: {} ", output._type)))
        };

        plugins.push(plugin);
    }

    Ok(plugins)
}
