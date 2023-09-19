use lazy_static::lazy_static;
use std::{
    env, process,
    sync::{Mutex, MutexGuard},
};

use crate::{Config, InteractionVariables};

use super::{complex_plane::ComplexPlane, mandelbrot_function::MandelbrotFunction, pixel_buffer::PixelBuffer, pixel_plane::PixelPlane};

lazy_static! {
    static ref MANDELBROT_MODEL_INSTANCE: Mutex<MandelbrotModel> = Mutex::new(MandelbrotModel::new());
}

pub struct MandelbrotModel {
    pub config: Config,
    pub c: ComplexPlane,
    pub p: PixelBuffer,
    pub vars: InteractionVariables,
    pub amount_of_threads: usize,
    pub m: MandelbrotFunction,
}

impl MandelbrotModel {
    pub fn new() -> MandelbrotModel {
        let config = Config::build(env::args()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        MandelbrotModel {
            config: config.clone(),
            c: ComplexPlane::new(config.window_width, config.window_height),
            p: PixelBuffer::new(PixelPlane::new(config.window_width, config.window_height)),
            vars: InteractionVariables::default(),
            amount_of_threads: num_cpus::get(),
            m: MandelbrotFunction::new(config.max_iterations, config.orbit_radius),
        }
    }

    /// Returns the singleton MandelbrotModel instance.
    pub fn get_instance() -> MutexGuard<'static, MandelbrotModel> {
        let lock = MANDELBROT_MODEL_INSTANCE.try_lock();
        if let Ok(instance) = lock {
            return instance;
        }
        panic!("You have called the singleton twice! This should never happen. It means that within the same scope, MandelbrotModel::get_instance() was called more than once.");
    }
}
