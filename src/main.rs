use std::env;
use std::process;

use mandelbrot::{self, MandelbrotModel};

fn main() {
    if let Err(err) = mandelbrot::run(&MandelbrotModel::get_instance().config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
