use std::{env, process};

use minigrep::Config;

// cargo run -- search_param
fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    // args = ["target/debug/minigrep", "needle", "haystack"]

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // env::args return an iterator.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
