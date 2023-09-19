use std::process;

use mandelbrot::{self, MandelbrotModel};

fn main() {
    if let Err(err) = mandelbrot::run() {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
