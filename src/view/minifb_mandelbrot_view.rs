use minifb::{Key, Window, WindowOptions};

use crate::model::mandelbrot_model::MandelbrotModel;

pub struct MandelbrotView {
    pub window: Window,
}

impl MandelbrotView {
    pub fn new(mandelbrot_model: &MandelbrotModel) -> MandelbrotView {
        // Create a new window
        let window = Window::new(
            "Mandelbrot set viewer",
            mandelbrot_model.config.window_width,
            mandelbrot_model.config.window_height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        //Print info about the MandelbrotModel
        mandelbrot_model.p.pixel_plane.print();
        mandelbrot_model.c.print();
        println!(
            "Mandelbrot set parameters: max. iterations is {} and orbit radius is {}",
            mandelbrot_model.config.max_iterations, mandelbrot_model.config.orbit_radius
        );
        println!(
            "Amount of CPU threads that will be used for rendering: {}",
            mandelbrot_model.amount_of_threads
        );
        println!(
            "Supersampling amount used for rendering: {}x",
            mandelbrot_model.config.supersampling_amount
        );
        println!();

        MandelbrotView { window }
    }

    pub fn update(&mut self, mandelbrot_model: &MandelbrotModel) {
        self.window
            .update_with_buffer(
                &mandelbrot_model.p.pixels,
                mandelbrot_model.config.window_width,
                mandelbrot_model.config.window_height,
            )
            .unwrap();
    }

    pub fn exit(&self) {
        if self.window.is_key_down(Key::Escape) {
            println!("Escape pressed, application exited gracefully.");
        } else {
            println!("Window closed, application exited gracefully.")
        }
    }
}
