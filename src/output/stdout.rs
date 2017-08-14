use std::collections::HashMap;

use duration;
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
        total: &[types::PullRequest],
        created: &[&types::PullRequest],
        updated: &[&types::PullRequest],
        stale: &[&types::PullRequest]
    ) -> Result<()> {
        println!("\nTotal open pull requests: {}", total.len());
        println!("Recently opened pull requests: {}", created.len());
        println!("Recently updated pull requests: {}", updated.len());
        println!("Stale pull requests: {}", stale.len());

        println!("\nRecently created: {}", created.len());
        for pr in created {
            println!("[{}] {} -- {} ({})", pr.base.repo.full_name, pr.title,
                pr.user.login, duration::relative_helper(&pr.created_at)?);
        }

        println!("\nRecently updated: {}", updated.len());
        for pr in updated {
            println!("[{}] {} -- {} ({})", pr.base.repo.full_name, pr.title,
                pr.user.login, duration::relative_helper(&pr.updated_at)?);
        };

        println!("\nStale: {}", stale.len());
        for pr in stale {
            println!("[{}] {} -- {} ({})", pr.base.repo.full_name, pr.title,
                pr.user.login, duration::relative_helper(&pr.updated_at)?);
        };

        Ok(())
    }
}
