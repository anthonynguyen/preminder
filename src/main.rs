extern crate config;
#[macro_use]
extern crate error_chain;
extern crate rayon;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::process::exit;

mod api;
use api::Api;

mod errors;
use errors::*;

mod settings;
mod types;

fn run() -> Result<()> {
    let sets = settings::Settings::new()?;

    let api = Api::new(
        sets.github_api_token,
        sets.github_host
    )?;

    let repos = api.list_repos(&sets.github_subject)?;
    let prs: Vec<types::PullRequest> = repos.par_iter()
        .flat_map(|repo| api.list_pull_requests(&repo.full_name))
        .flat_map(|ve| ve)
        .collect();

    for pr in &prs {
        println!("[{}] {} ({})\n{}\n", pr.base.repo.full_name, pr.title,
            pr.state, pr.user.login);
    }

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        eprintln!("error: {}", e);

        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace: {:?}", backtrace);
        }

        exit(1);
    }
}
