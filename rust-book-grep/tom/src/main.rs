use minigrep::{run, Config};

fn main() {
    match Config::new() {
        Ok(config) => run(config),
        Err(e) => println!("{}", e),
    }
}
