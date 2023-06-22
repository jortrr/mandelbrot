use std::env;
use std::process;

use mandelbrot::{self, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = mandelbrot::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
