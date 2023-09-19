use std::{env, process};

use mandelbrot::{self};

fn main() {
    //Turn on Rust backtrace
    env::set_var("RUST_BACKTRACE", "1");

    if let Err(err) = mandelbrot::run() {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
