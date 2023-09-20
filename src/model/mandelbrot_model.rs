use lazy_static::lazy_static;
use std::{
    env, process,
    sync::{Mutex, MutexGuard},
};

use crate::{
    controller::interaction_variables::InteractionVariables, model::coloring::TrueColor, Config, COLORING_FUNCTION, COLOR_CHANNEL_MAPPING,
};

use super::{complex_plane::ComplexPlane, mandelbrot_function::MandelbrotFunction, pixel_buffer::PixelBuffer, pixel_plane::PixelPlane};

lazy_static! {
    static ref MANDELBROT_MODEL_INSTANCE: Mutex<MandelbrotModel> = Mutex::new(MandelbrotModel::new());
}

pub type ColoringFunction = fn(iterations: u32, max_iterations: u32) -> TrueColor;

pub struct MandelbrotModel {
    pub config: Config,
    pub c: ComplexPlane,
    pub p: PixelBuffer,
    pub vars: InteractionVariables,
    pub amount_of_threads: usize,
    pub m: MandelbrotFunction,
    pub coloring_function: ColoringFunction,
}

impl MandelbrotModel {
    pub fn new() -> MandelbrotModel {
        let config = Config::build(env::args()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        let mut result = MandelbrotModel {
            config: config.clone(),
            c: ComplexPlane::new(config.window_width, config.window_height),
            p: PixelBuffer::new(PixelPlane::new(config.window_width, config.window_height)),
            vars: InteractionVariables::default(),
            amount_of_threads: num_cpus::get(),
            m: MandelbrotFunction::new(config.max_iterations, config.orbit_radius),
            coloring_function: COLORING_FUNCTION,
        };
        //Color channel mapping
        result.p.color_channel_mapping = COLOR_CHANNEL_MAPPING;

        result
    }

    /// Returns the singleton MandelbrotModel instance.
    pub fn get_instance() -> MutexGuard<'static, MandelbrotModel> {
        let lock = MANDELBROT_MODEL_INSTANCE.try_lock();
        if let Ok(instance) = lock {
            return instance;
        }
        panic!("[DEADLOCK]: You have called the singleton twice! This should never happen. It means that within the same scope, MandelbrotModel::get_instance() was called more than once.");
    }
}
