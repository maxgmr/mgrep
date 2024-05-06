use std::{env, process};

use mgrep::Config;

fn main() {
    // env::args() returns an iterator
    // pass iterator ownership to Config::build
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = mgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
