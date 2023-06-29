//This entire file is a work-in-progress; currently unused
use crate::Config;

pub struct ImageParameters {
    pub width: usize,
    pub height: usize,
    pub supersampling_amount: u8
}

impl ImageParameters {
    pub fn new_from_config(config: &Config) -> ImageParameters {
        ImageParameters { width: config.width, height: config.height, supersampling_amount: config.supersampling_amount }
    }

    pub fn save_image(&self) {
        //TODO
    }
}