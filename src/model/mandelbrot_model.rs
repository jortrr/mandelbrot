use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

use crate::Config;

lazy_static! {
    static ref MANDELBROT_MODEL_INSTANCE: Mutex<MandelbrotModel> = Mutex::new(MandelbrotModel::new());
}

pub struct MandelbrotModel {
    pub config: Config,
}

impl MandelbrotModel {
    pub fn new() -> MandelbrotModel {
        MandelbrotModel { config: Config::new() }
    }

    /// Returns the singleton MandelbrotModel instance.
    pub fn get_instance() -> MutexGuard<'static, MandelbrotModel> {
        MANDELBROT_MODEL_INSTANCE.lock().unwrap()
    }
}
