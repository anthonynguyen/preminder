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

fn run() -> Result<()> {
    let set = settings::load()?;

    println!("{:?}", set);

    let api = Api::new(
        set.github_api_token,
        set.github_host
    )?;

    println!("{:?}", api.list_repos(set.github_subject)?);

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
