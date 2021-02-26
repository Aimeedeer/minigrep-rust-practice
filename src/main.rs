use minigrep::Config;
use std::{env, process};

// run:
// $ cargo run to poem.txt
// $ CASE_INSENSITIVE=1 cargo run to poem.txt
// $ cargo run to poem.txt > output.txt
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Run(config) error: {}", e);
        process::exit(1);
    }
}
