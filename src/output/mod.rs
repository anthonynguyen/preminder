use std;
use std::collections::HashMap;

use chrono;
use handlebars;

mod email;
mod hipchat;
mod stdout;

use duration;
use errors::*;
use settings::OutputBlock;
use types;

#[derive(Debug)]
pub struct OutputMeta {
    pub now: chrono::DateTime<chrono::offset::Local>,
    pub recent: chrono::Duration,
    pub stale: chrono::Duration
}

pub struct OutputData<'a> {
    pub total: &'a [types::PullRequest],
    pub created: &'a [&'a types::PullRequest],
    pub updated: &'a [&'a types::PullRequest],
    pub stale: &'a [&'a types::PullRequest]
}

pub trait OutputPlugin {
    fn new(config: &Option<HashMap<String, String>>)
        -> Result<Box<OutputPlugin>> where Self:Sized;

    fn remind(&self,
        meta: &OutputMeta,
        total: &[types::PullRequest],
        created: &[&types::PullRequest],
        updated: &[&types::PullRequest],
        stale: &[&types::PullRequest])
        -> Result<()>;
}

pub struct OutputSet {
    plugins: Vec<Box<OutputPlugin>>
}

impl OutputSet {
    pub fn new(configured: &[OutputBlock]) -> Result<Self> {
        let mut plugins: Vec<Box<OutputPlugin>> = Vec::new();

        for output in configured {
            if output.disable {
                continue;
            }

            let plugin = match output._type.as_ref() {
                "stdout" => stdout::StdoutPlugin::new(&output.config)?,
                "hipchat" => hipchat::HipchatPlugin::new(&output.config)?,
                "email" => email::EmailPlugin::new(&output.config)?,
                _ => return Err(format!("Invalid output type: {}", output._type).into())
            };

            plugins.push(plugin);
        }

        Ok(OutputSet {
            plugins: plugins
        })
    }

    pub fn remind_all(&self,
        meta: &OutputMeta,
        data: &OutputData) {
        for plugin in &self.plugins {
            plugin.remind(meta, data.total, data.created, data.updated, data.stale)
                .unwrap_or_else(|err| error!("Output : {}", err));
        }
    }
}

pub fn handlebars_relative_helper(helper: &handlebars::Helper,
    _: &handlebars::Handlebars,
    rc: &mut handlebars::RenderContext
    ) -> std::result::Result<(), handlebars::RenderError> {
    let param = helper.param(0)
        .ok_or_else(|| handlebars::RenderError::new("No param given?"))?
        .value()
        .as_str()
        .ok_or_else(|| handlebars::RenderError::new("Param is not a string"))?
        .parse::<chrono::DateTime<chrono::Utc>>()
        .map_err(|_| handlebars::RenderError::new("Param could not be parsed as a datetime"))?
        .with_timezone::<chrono::offset::Local>(&chrono::offset::Local);

    let now = chrono::Local::now();
    let fin = duration::relative::<chrono::offset::Local>(param, now);

    rc.writer.write_all(&fin.into_bytes())?;
    Ok(())
}
