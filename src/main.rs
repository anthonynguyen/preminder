extern crate chrono;
extern crate clap;
extern crate config;
#[macro_use] extern crate error_chain;
extern crate rayon;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;

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
        .arg(clap::Arg::with_name("config")
            .help("Path to config file")
            .short("c")
            .long("config")
            .takes_value(true)
            .value_name("FILE"))
        .get_matches();

    let config_path = matches.value_of("config");

    if let Err(ref e) = run::run(config_path) {
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
