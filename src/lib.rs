use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};

use angular_units::Deg;
use mandelbrot_set::MandelbrotSet;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use prisma::{Hsv, Rgb, FromColor};
use num_cpus;

use crate::complex_plane::{ComplexPlane, View};
use crate::key_bindings::{KeyBindings, KeyAction};
use crate::pixel_buffer::PixelBuffer;
use crate::pixel_buffer::pixel_plane::PixelPlane;

mod complex_plane;
mod complex;
mod pixel_buffer;
mod mandelbrot_set;
mod rendering;
mod key_bindings;

//Views
static VIEW_1: View = View::new(-0.6604166666666667, 0.4437500000000001, 0.1);
static VIEW_2: View = View::new(-1.0591666666666668, 0.2629166666666668, 0.01);
static VIEW_3: View = View::new(-0.4624999999999999, 0.55, 0.1);
static VIEW_4: View = View::new(-0.46395833333333325, 0.5531250000000001, 0.03);
static VIEW_5: View = View::new(-0.4375218333333333, 0.5632133750000003, 0.00002000000000000002);
static VIEW_6: View = View::new(-0.7498100000000001, -0.020300000000000054, 0.00006400000000000002);

pub struct Config {
    // Window dimensions in pixels
    pub width: usize,
    pub height: usize,
    // Mandelbrot set parameters
    pub max_iterations: u32,
    pub orbit_radius: f64,      //If z remains within the orbit_radius in max_iterations, we assume c does not tend to infinity
}


impl Config {
    /// Parse the command line arguments from e.g. env::args() in the following format
    /// ```
    /// cargo run -- width height max_iterations
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        args.next(); //Skip the first argument as it is the name of the executable

        let width = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no width argument given")),
        };

        let width = match width.parse::<usize>() {
            Ok(val) => val,
            Err(err) => return Err(err.to_string() + &String::from(" for width")),
        };

        let height = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no height argument given")),
        };

        let height = match height.parse::<usize>() {
            Ok(val) => val,
            Err(err) => return Err(err.to_string() + &String::from(" for height")),
        };

        let max_iterations = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no max_iterations argument given")),
        };

        let max_iterations = match max_iterations.parse::<u32>() {
            Ok(val) => val,
            Err(err) => return Err(err.to_string() + &String::from(" for max_iterations")),
        };

        Ok(Config {width, height, max_iterations, orbit_radius: 2.0})
    }
}

pub struct InteractionVariables{
    ///Variable determining the amount of rows and columns are translated by pressing the 4 arrow keys
    pub translation_amount: u8,
    ///Variable denoting the user scaling speed; the lower this value, the more aggressive the zooming will become
    pub scale_denominator: f64,
    pub scale_numerator: f64,
}

impl InteractionVariables{
    pub fn new(translation_amount: u8, scale_numerator: f64, scale_denominator: f64) -> InteractionVariables {
        InteractionVariables { translation_amount, scale_numerator, scale_denominator }
    }

    pub fn scaling_factor(&self) -> f64 {
        self.scale_numerator / self.scale_denominator
    }

    pub fn inverse_scaling_factor(&self) -> f64 {
        self.scale_denominator / self.scale_numerator
    }

    pub fn increment_translation_amount(&mut self) {
        if self.translation_amount < u8::MAX {
            self.translation_amount+=1;
        }
    }

    pub fn decrement_translation_amount(&mut self) {
        if self.translation_amount > 1 {
            self.translation_amount -=1;
        }
    }

    pub fn increment_scale_numerator(&mut self) {
        if self.scale_numerator < 9.0 {
            self.scale_numerator += 1.0;
        }
    }

    pub fn decrement_scale_numerator(&mut self) {
        if self.scale_numerator > 1.0 {
            self.scale_numerator -= 1.0;
        }
    }
}

impl Default for InteractionVariables{
    fn default() -> Self {
        InteractionVariables { translation_amount:10, scale_numerator: 9.0, scale_denominator: 10.0 }
    }
}

// Handle any key events
fn handle_key_events(window: &Window, c: &mut ComplexPlane, p: &mut PixelBuffer, m: &MandelbrotSet, vars: &mut InteractionVariables, k: &KeyBindings) {
    for key in window.get_keys_pressed(minifb::KeyRepeat::No) {
        println!("\nKey pressed: {:?}", key);
        match key {
            Key::Up => rendering::translate_and_render_efficiently(c, p, m, vars.translation_amount.into(), 0),
            Key::Down => rendering::translate_and_render_efficiently(c, p, m, -(vars.translation_amount as i16), 0),
            Key::Left => rendering::translate_and_render_efficiently(c, p, m, 0, -(vars.translation_amount as i16)),
            Key::Right => rendering::translate_and_render_efficiently(c, p, m, 0, vars.translation_amount.into()),
            Key::R => c.reset(),
            Key::NumPadPlus => vars.increment_translation_amount(),
            Key::NumPadMinus => vars.decrement_translation_amount(),
            Key::NumPadAsterisk => vars.increment_scale_numerator(),
            Key::NumPadSlash => vars.decrement_scale_numerator(),
            Key::LeftBracket => c.scale(vars.scaling_factor()),
            Key::RightBracket => c.scale(vars.inverse_scaling_factor()),
            Key::C => println!("Center: {:?}, scale: {:?}", c.center(), c.get_scale()),
            Key::Key1 => c.set_view(&VIEW_1),
            Key::Key2 => c.set_view(&VIEW_2),
            Key::Key3 => c.set_view(&VIEW_3),
            Key::Key4 => c.set_view(&VIEW_4),
            Key::Key5 => c.set_view(&VIEW_5),
            Key::Key6 => c.set_view(&VIEW_6),
            Key::K => k.print(),
            _ => (),
        }
        match key {
            Key::NumPadPlus | Key::NumPadMinus => println!("translation_amount: {}", vars.translation_amount),
            Key::NumPadSlash | Key::NumPadAsterisk => println!("scale factor: {}/{}",vars.scale_numerator,vars.scale_denominator),
            Key::Up | Key::Down | Key::Left | Key::Right => c.print(),
            Key::R | Key::Key1 | Key::Key2 | Key::Key3 | Key::Key4 | Key::Key5 | Key::Key6 | Key::LeftBracket | Key::RightBracket => {
                rendering::render_complex_plane_into_buffer(p, c, m);
                c.print();
            },
            _ => (),
        }
    }
}

fn handle_mouse_events(window: &Window, c: &mut ComplexPlane, p: &mut PixelBuffer, m: &MandelbrotSet) {
    static LEFT_MOUSE_DOWN_PREVIOUSLY: AtomicBool = AtomicBool::new(false); //Static variable with interior mutability to toggle mouse clicks; without such a variable, clicking the screen once would result in multiple actions
    static RIGHT_MOUSE_DOWN_PREVIOUSLY: AtomicBool = AtomicBool::new(false); 

    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
        let x: usize = x as usize;
        let y: usize = y as usize;

        //Left mouse status
        let left_mouse_down = window.get_mouse_down(MouseButton::Left);
        let left_mouse_down_previously = LEFT_MOUSE_DOWN_PREVIOUSLY.load(Ordering::Relaxed);
        let left_mouse_clicked = left_mouse_down && !left_mouse_down_previously;
        //Left mouse actions
        if left_mouse_clicked {
            println!("({x}, {y})");
            let iterations = p.iterations_at_point(x, y, m.max_iterations);
            let complex = c.complex_from_pixel_plane(x, y);
            println!("{:?}", complex);
            println!("iterations: {}", iterations);
            println!();
        }

        //Right mouse status
        let right_mouse_down = window.get_mouse_down(MouseButton::Right);
        let right_mouse_down_previously = RIGHT_MOUSE_DOWN_PREVIOUSLY.load(Ordering::Relaxed);
        let right_mouse_clicked = right_mouse_down && !right_mouse_down_previously;
        //Right mouse actions
        if right_mouse_clicked {
            println!("({x}, {y})");
            let new_center = c.complex_from_pixel_plane(x, y);
            println!("c.center: {:?}", c.center());
            println!("new_center: {:?}", new_center);

            rendering::translate_to_center_and_render_efficiently(c, p, m, &new_center);
            c.print();
            println!();
        }

        //Store the current mouse values, to allow for single-time mouse clicking
        if left_mouse_down != left_mouse_down_previously {LEFT_MOUSE_DOWN_PREVIOUSLY.store(left_mouse_down, Ordering::Relaxed)};
        if right_mouse_down != right_mouse_down_previously {RIGHT_MOUSE_DOWN_PREVIOUSLY.store(right_mouse_down, Ordering::Relaxed)};
    }
}

///Holds all the logic currently in the main function that isn't involved with setting up configuration or handling errors, to make `main` concise and
///easy to verify by inspection
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Complex plane dimensions and increments
    let mut c = ComplexPlane::new(config.width, config.height);
    // Pixel plane and buffer
    let mut p = PixelBuffer::new(PixelPlane::new(config.width, config.height));
    // User interaction variables
    let mut vars = InteractionVariables::default();
    // Multithreading variables
    let amount_of_threads = num_cpus::get(); //Amount of CPU threads to use, TODO: use this value in rendering functions
    // Mandelbrot set iterator
    let m: MandelbrotSet = MandelbrotSet::new(config.max_iterations, config.orbit_radius);
    // Create a new window
    let mut window = Window::new(
        "Mandelbrot set viewer",
        config.width,
        config.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    //Initialize keybindings TODO: I want to have a vector of structs containing functions with different signatures, this is not easily possible. All functionality should be placed here, in the future, when 
    //I've figured out how to have closures with different signatures in the same struct field
    //For now, use empty_closure, to have a closure that does nothing as action
    let mut key_bindings: KeyBindings = KeyBindings::new(Vec::new());
    let empty_closure = || ();
    key_bindings.add(Key::Up, "Move up translation_amount pixels", empty_closure);
    key_bindings.add(Key::Down, "Move down translation_amount pixels", empty_closure);
    key_bindings.add(Key::Left, "Move left translation_amount pixels", empty_closure);
    key_bindings.add(Key::Right, "Move right translation_amount pixels", empty_closure);
    key_bindings.add(Key::R, "Reset the Mandelbrot set view to the starting view", empty_closure);
    key_bindings.add(Key::NumPadPlus, "Increment translation_amount", empty_closure);
    key_bindings.add(Key::NumPadMinus, "Decrement translation amount", empty_closure);
    key_bindings.add(Key::NumPadAsterisk, "Increment scale_numerator", empty_closure);
    key_bindings.add(Key::NumPadSlash, "Decrement scale_numerator", empty_closure);
    key_bindings.add(Key::LeftBracket, "Scale the view by scaling_factor, effectively zooming in",empty_closure);
    key_bindings.add(Key::RightBracket, "Scale the view by inverse_scaling_factor, effectively zooming out", empty_closure);
    key_bindings.add(Key::C, "Prints the current Mandelbrot set view; the center and scale", empty_closure);
    key_bindings.add(Key::Key1, "Renders VIEW_1", empty_closure);
    key_bindings.add(Key::Key2, "Renders VIEW_2", empty_closure);
    key_bindings.add(Key::Key3, "Renders VIEW_3", empty_closure);
    key_bindings.add(Key::Key4, "Renders VIEW_4", empty_closure);
    key_bindings.add(Key::Key5, "Renders VIEW_5", empty_closure);
    key_bindings.add(Key::Key6, "Renders VIEW_6", empty_closure);
    key_bindings.add(Key::K, "Prints the keybindings", empty_closure);
    key_bindings.print();



    p.pixel_plane.print();
    c.print();
    println!("Mandelbrot set parameters: max. iterations is {} and orbit radius is {}", config.max_iterations, config.orbit_radius);
    println!("Amount of CPU threads that will be used for rendering: {}", amount_of_threads);
    println!();

    rendering::render_complex_plane_into_buffer(&mut p, &c, &m);

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        // Update the window with the new buffer
        window.update_with_buffer(&p.buffer, config.width, config.height).unwrap();

        // Handle any window events
        handle_key_events(&window, &mut c, &mut p, &m, &mut vars, &key_bindings);

        //Handle any mouse events
        handle_mouse_events(&window, &mut c, &mut p, &m);
    }

    Ok(())
}

/// Get the amount of Mandelbrot iterations from a HSV colored pixel //TODO: This function is wonky, it should go
fn iterations_from_hsv_pixel(pixel: u32, max_iterations: u32) -> u32 {
    let r = (pixel >> 16) & 0xFF;
    let g = (pixel >> 8) & 0xFF;
    let b = pixel & 0xFF;
    let rgb = Rgb::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
    let hsv: Hsv<f64, _> = Hsv::from_color(&rgb);
    let hue: Deg<f64> = hsv.hue();
    let value = hsv.value();
    if value == 0.0 { //If value == 0.0, the pixel is an element of the Mandelbrot set
        return max_iterations;
    }
    let iterations: u32 = (max_iterations as f64 * (hue.0 / 359.0) as f64) as u32; 
    iterations
}