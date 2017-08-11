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
    let set = settings::load()?;
    let api = Api::new(
        set.github_api_token,
        set.github_host
    )?;

    let repos = api.list_repos(set.github_subject)?;
    for repo in &repos {
        let desc = match repo.description {
            Some(ref s) => s.to_owned(),
            _ => "NO DESCRIPTION".to_owned()
        };
        println!("{}:\n{}\n", repo.full_name, desc);
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
