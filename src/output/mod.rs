use std::collections::HashMap;

use chrono;
use tera;

mod email;
mod hipchat;
mod stdout;

mod template_filters;

use duration;
use errors::*;
use types;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum OutputBlock {
    Stdout(stdout::Config),
    Hipchat(hipchat::Config),
    Email(email::Config),
}

#[derive(Debug)]
pub struct OutputMeta {
    pub now: chrono::DateTime<chrono::offset::Local>,
    pub recent: chrono::Duration,
    pub stale: chrono::Duration,
}

pub struct OutputData<'a> {
    pub total: &'a [types::PullRequest],
    pub created: &'a [&'a types::PullRequest],
    pub updated: &'a [&'a types::PullRequest],
    pub stale: &'a [&'a types::PullRequest],
}

pub trait OutputPlugin {
    fn check_templates(&self, templates: &[String]) -> Result<()>;

    fn remind(
        &self,
        meta: &OutputMeta,
        data: &OutputData,
        templated: &HashMap<String, String>,
    ) -> Result<()>;
}

pub struct OutputSet {
    plugins: Vec<Box<OutputPlugin>>,
    templater: tera::Tera,
    templates: Vec<String>,
}

impl OutputSet {
    pub fn new(
        configured: &[OutputBlock],
        ctemplates: Option<HashMap<String, String>>,
    ) -> Result<Self> {
        let mut plugins: Vec<Box<OutputPlugin>> = Vec::new();

        let mut templater = tera::Tera::default();
        templater.register_filter("relative", template_filters::relative);

        let mut templates = Vec::new();
        if let Some(hmap) = ctemplates {
            let template_tuples: Vec<(&str, &str)> = hmap.iter()
                .map(|kvt| (kvt.0.as_ref(), kvt.1.as_ref()))
                .collect();

            templater.add_raw_templates(template_tuples.clone())?;
            templates = template_tuples
                .iter()
                .map(|kvt| kvt.0.to_string())
                .collect();
        }

        for output in configured {
            let plugin = match *output {
                OutputBlock::Stdout(ref cfg) => stdout::Plugin::init(cfg)?,
                OutputBlock::Hipchat(ref cfg) => hipchat::Plugin::init(cfg)?,
                OutputBlock::Email(ref cfg) => email::Plugin::init(cfg)?,
            };

            plugin.check_templates(&templates)?;
            plugins.push(plugin);
        }

        Ok(OutputSet {
            plugins: plugins,
            templater: templater,
            templates: templates,
        })
    }

    pub fn remind_all(&self, meta: &OutputMeta, data: &OutputData) -> Result<()> {
        let info = json!({
            "now": meta.now,
            "recent_period": duration::nice(meta.recent),
            "stale_period": duration::nice(meta.stale),

            "total": data.total,
            "opened": data.created,
            "updated": data.updated,
            "stale": data.stale
        });

        let mut templated: HashMap<String, String> = HashMap::new();
        for name in &self.templates {
            let rendered = self.templater.render(name, &info)?;
            templated.insert(name.to_string(), rendered);
        }

        for plugin in &self.plugins {
            plugin.remind(meta, data, &templated).unwrap_or_else(
                |err| error!("Output : {}", err),
            );
        }

        Ok(())
    }
}
