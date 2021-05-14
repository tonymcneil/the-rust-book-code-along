use rust_book_ch12_01_minigrep::{run, Config};
use std::{env, process};

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("ISSUE building config, got ERROR: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!(
            "ISSUE: couldn't read file: {}, got ERROR: {}",
            &config.filename, e
        );
        process::exit(1);
    }
}
