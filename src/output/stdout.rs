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
        total: &Vec<types::PullRequest>,
        created: &Vec<&types::PullRequest>,
        updated: &Vec<&types::PullRequest>,
        stale: &Vec<&types::PullRequest>
    ) -> Result<()> {
        println!("\nTotal open pull requests: {}", total.len());
        println!("Recently opened pull requests: {}", created.len());
        println!("Recently updated pull requests: {}", updated.len());
        println!("Stale pull requests: {}", stale.len());

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
