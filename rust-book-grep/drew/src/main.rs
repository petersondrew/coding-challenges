use std::env;
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).expect("Sorry, I didn't get that");

    match minigrep::run(config) {
        Err(err) => eprintln!("Unexpected error: {err}"),
        _ => {}
    }
}

