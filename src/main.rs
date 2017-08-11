extern crate clap;
extern crate config;
#[macro_use]
extern crate error_chain;
extern crate rayon;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod api;
mod errors;
mod settings;
mod types;

use api::Api;
use errors::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn run(config_path: Option<&str>) -> Result<()> {
    let sets = settings::Settings::new(config_path)?;

    let api = Api::new(
        sets.github.token,
        sets.github.host
    )?;

    let repos = api.list_repos(&sets.github.subject)?;
    println!("Found {} repositories", repos.len());

    let prs: Vec<types::PullRequest> = repos.par_iter()
        .flat_map(|repo| api.list_pull_requests(&repo.full_name))
        .flat_map(|ve| ve)
        .collect();

    println!("Found {} pull requests", prs.len());

    for pr in &prs {
        println!("[{}] {} ({})\n{}\n", pr.base.repo.full_name, pr.title,
            pr.state, pr.user.login);
    }

    Ok(())
}

fn main() {
    let matches = clap::App::new("preminder")
        .version(VERSION)
        .author(AUTHORS)
        .arg(clap::Arg::with_name("config")
            .help("Path to config file")
            .short("c")
            .long("config")
            .takes_value(true)
            .value_name("FILE"))
        .get_matches();

    let config_path = matches.value_of("config");

    if let Err(ref e) = run(config_path) {
        eprintln!("error: {}", e);

        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace: {:?}", backtrace);
        }

        std::process::exit(1);
    }
}
