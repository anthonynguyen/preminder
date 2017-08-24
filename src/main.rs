#![recursion_limit="128"]

extern crate chrono;
extern crate clap;
extern crate config;
#[macro_use]
extern crate error_chain;
extern crate lettre;
#[macro_use]
extern crate log;
extern crate loglog;
extern crate rayon;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tera;

mod api;
mod duration;
mod errors;
mod output;
mod run;
mod settings;
mod types;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = clap::App::new("preminder")
        .version(VERSION)
        .author(AUTHORS)
        .arg(
            clap::Arg::with_name("config")
                .help("Path to config file")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("FILE"),
        )
        .get_matches();

    loglog::build().target(true).init().unwrap_or_else(|err| {
        eprintln!("Oh no! The logger couldn't be started!\n{}", err);
        std::process::exit(1);
    });

    let config_path = matches.value_of("config");

    if let Err(ref e) = run::run(config_path) {
        error!("{}", e);

        for e in e.iter().skip(1) {
            error!("Caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            error!("Backtrace: {:?}", backtrace);
        }

        std::process::exit(1);
    }
}
