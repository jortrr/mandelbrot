use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref MANDELBROT_MODEL_INSTANCE: Mutex<MandelbrotModel> = Mutex::new(MandelbrotModel::new());
}

pub struct MandelbrotModel {
    number: i32,
}

impl MandelbrotModel {
    pub fn new() -> MandelbrotModel {
        MandelbrotModel { number: 0 }
    }

    pub fn add_one(&mut self) {
        self.number += 1;
    }

    /// Returns the singleton MandelbrotModel instance.
    pub fn get_instance() -> MutexGuard<'static, MandelbrotModel> {
        MANDELBROT_MODEL_INSTANCE.lock().unwrap()
    }
}
