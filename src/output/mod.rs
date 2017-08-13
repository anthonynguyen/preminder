use std::collections::HashMap;

use chrono;

mod hipchat;
mod stdout;

use errors::*;
use settings::OutputBlock;
use types;

#[derive(Debug)]
pub struct OutputMeta {
    pub now: chrono::DateTime<chrono::offset::Local>,
    pub period: chrono::Duration
}

pub trait OutputPlugin {
    fn new(config: &Option<HashMap<String, String>>)
        -> Result<Box<OutputPlugin>> where Self:Sized;

    fn remind(&self,
        meta: &OutputMeta,
        total: &Vec<types::PullRequest>,
        created: &Vec<&types::PullRequest>,
        updated: &Vec<&types::PullRequest>)
        -> Result<()>;
}

pub fn init(configured: &Vec<OutputBlock>) -> Result<Vec<Box<OutputPlugin>>> {
    let mut plugins: Vec<Box<OutputPlugin>> = Vec::new();

    for output in configured {
        let plugin = match output._type.as_ref() {
            "stdout" => stdout::StdoutPlugin::new(&output.config)?,
            "hipchat" => hipchat::HipchatPlugin::new(&output.config)?,
            _ => return Err(Error::from(format!("Invalid output type: {} ", output._type)))
        };

        plugins.push(plugin);
    }

    Ok(plugins)
}
