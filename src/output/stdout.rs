use std::collections::HashMap;

use errors::*;
use output::{OutputMeta, OutputPlugin};
use types;

#[derive(Debug,Deserialize)]
pub struct StdoutPlugin {}

impl OutputPlugin for StdoutPlugin {
    fn new(_config: &Option<HashMap<String, String>>) -> Result<Box<OutputPlugin>> {
        Ok(Box::new(StdoutPlugin{}))
    }

    fn remind(&self,
        _meta: &OutputMeta,
        _total: &Vec<types::PullRequest>,
        created: &Vec<&types::PullRequest>,
        updated: &Vec<&types::PullRequest>
    ) -> Result<()> {
        println!("\nFound {} pull requests recently created:", created.len());
        for pr in created {
            println!("[{}] {} -- {}", pr.base.repo.full_name, pr.title,
                pr.user.login);
        }

        println!("\nFound {} pull requests recently updated:", updated.len());
        for pr in updated {
            println!("[{}] {} -- {}", pr.base.repo.full_name, pr.title,
                pr.user.login);
        };

        Ok(())
    }
}