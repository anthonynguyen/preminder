extern crate config;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::process::exit;

mod api;
use api::Api;

mod errors;
use errors::*;

mod settings;
mod types;

fn run() -> Result<()> {
    let sets = settings::load()?;
    let api = Api::new(
        sets.github_api_token,
        sets.github_host
    )?;

    let repos = api.list_repos(&sets.github_subject)?;
    for repo in &repos {
        let desc = match repo.description {
            Some(ref s) => s.to_owned(),
            _ => "NO DESCRIPTION".to_owned()
        };
        println!("{}:\n{}\n", repo.full_name, desc);
    }

    if let Some(repo) = repos.first() {
        let prs = api.list_pull_requests(&repo.full_name)?;
        for pr in &prs {
            println!("{} ({})\n{}\n", pr.title, pr.state, pr.user.login);
        }
    }

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        exit(1);
    }
}
