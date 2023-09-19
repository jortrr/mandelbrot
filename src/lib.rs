#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::perf,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity
)]
#![allow(
    clippy::must_use_candidate,
    clippy::multiple_crate_versions,
    clippy::uninlined_format_args,
    clippy::let_and_return,
    clippy::missing_const_for_fn,
    clippy::use_self,
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    clippy::needless_return,
    clippy::return_self_not_must_use,
    clippy::unreadable_literal,
    clippy::single_match_else,
    clippy::suboptimal_flops,
    clippy::many_single_char_names,
    clippy::cast_sign_loss
)]

pub mod controller;
pub mod model;
pub mod view;

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

//Crate includes
pub use controller::config::Config;
use controller::key_bindings::KeyBindings;
use controller::user_input::{ask, pick_option};
use model::complex_plane::{ComplexPlane, View};
use model::mandelbrot_function::MandelbrotFunction;
pub use model::mandelbrot_model::MandelbrotModel;
use model::{pixel_buffer, pixel_plane, rendering};
use pixel_buffer::PixelBuffer;
use pixel_plane::PixelPlane;
use view::coloring::ColorChannelMapping;
use view::coloring::TrueColor;

//Coloring function
type ColoringFunction = fn(iterations: u32, max_iterations: u32) -> TrueColor;
static COLORING_FUNCTION: ColoringFunction = TrueColor::new_from_bernstein_polynomials;

//Color channel mapping
static COLOR_CHANNEL_MAPPING: ColorChannelMapping = ColorChannelMapping::RGB;

//Views
static VIEW_1: View = View::new(-0.6604166666666667, 0.4437500000000001, 0.1);
static VIEW_2: View = View::new(-1.0591666666666668, 0.2629166666666668, 0.01);
static VIEW_3: View = View::new(-0.4624999999999999, 0.55, 0.1);
static VIEW_4: View = View::new(-0.46395833333333325, 0.5531250000000001, 0.03);
static VIEW_5: View = View::new(-0.4375218333333333, 0.5632133750000003, 0.00002000000000000002);
static VIEW_6: View = View::new(-0.7498100000000001, -0.020300000000000054, 0.00006400000000000002);
static VIEW_7: View = View::new(-1.7862712000000047, 0.000052399999999991516, 0.00001677721600000001);
static VIEW_8: View = View::new(-1.7862581627050718, 0.00005198056959995248, 0.000006039797760000003);
static VIEW_9: View = View::new(-0.4687339999999999, 0.5425518958333333, 0.000010000000000000003);
static VIEW_0: View = View::new(-0.437520465811966, 0.5632133750000006, 0.000004000000000000004);

//Banner values
static VERSION: &str = "1.4";

pub struct InteractionVariables {
    ///Variable determining the amount of rows and columns are translated by pressing the 4 arrow keys
    pub translation_amount: u8,
    ///Variable denoting the user scaling speed; the lower this value, the more aggressive the zooming will become
    pub scale_denominator: f64,
    pub scale_numerator: f64,
}

impl InteractionVariables {
    pub fn new(translation_amount: u8, scale_numerator: f64, scale_denominator: f64) -> InteractionVariables {
        InteractionVariables {
            translation_amount,
            scale_denominator,
            scale_numerator,
        }
    }

    pub fn scaling_factor(&self) -> f64 {
        self.scale_numerator / self.scale_denominator
    }

    pub fn inverse_scaling_factor(&self) -> f64 {
        self.scale_denominator / self.scale_numerator
    }

    pub fn increment_translation_amount(&mut self) {
        self.translation_amount = self.translation_amount.saturating_add(1);
    }

    pub fn decrement_translation_amount(&mut self) {
        if self.translation_amount > 1 {
            self.translation_amount -= 1;
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

impl Default for InteractionVariables {
    fn default() -> Self {
        InteractionVariables {
            translation_amount: 10,
            scale_numerator: 9.0,
            scale_denominator: 10.0,
        }
    }
}

// Handle any key events
fn handle_key_events(
    window: &Window,
    c: &mut ComplexPlane,
    p: &mut PixelBuffer,
    m: &mut MandelbrotFunction,
    vars: &mut InteractionVariables,
    k: &KeyBindings,
    supersampling_amount: &mut u8,
    image_supersampling_amount: &mut u8,
    coloring_function: &mut ColoringFunction,
    config: &Config,
) {
    if let Some(key) = window.get_keys_pressed(minifb::KeyRepeat::No).first() {
        print!("\nKey pressed: ");
        k.print_key(key);
        match key {
            Key::Up => rendering::translate_and_render_efficiently(
                c,
                p,
                m,
                vars.translation_amount.into(),
                0,
                *supersampling_amount,
                *coloring_function,
            ),
            Key::Down => rendering::translate_and_render_efficiently(
                c,
                p,
                m,
                -i16::from(vars.translation_amount),
                0,
                *supersampling_amount,
                *coloring_function,
            ),
            Key::Left => rendering::translate_and_render_efficiently(
                c,
                p,
                m,
                0,
                -i16::from(vars.translation_amount),
                *supersampling_amount,
                *coloring_function,
            ),
            Key::Right => rendering::translate_and_render_efficiently(
                c,
                p,
                m,
                0,
                vars.translation_amount.into(),
                *supersampling_amount,
                *coloring_function,
            ),
            Key::R => c.reset(),
            Key::NumPadPlus => vars.increment_translation_amount(),
            Key::NumPadMinus => vars.decrement_translation_amount(),
            Key::NumPadAsterisk => vars.increment_scale_numerator(),
            Key::NumPadSlash => vars.decrement_scale_numerator(),
            Key::LeftBracket => c.scale(vars.scaling_factor()),
            Key::RightBracket => c.scale(vars.inverse_scaling_factor()),
            Key::V => println!("Center: {:?}, scale: {:?}", c.center(), c.get_scale()),
            Key::Key1 => c.set_view(&VIEW_1),
            Key::Key2 => c.set_view(&VIEW_2),
            Key::Key3 => c.set_view(&VIEW_3),
            Key::Key4 => c.set_view(&VIEW_4),
            Key::Key5 => c.set_view(&VIEW_5),
            Key::Key6 => c.set_view(&VIEW_6),
            Key::Key7 => c.set_view(&VIEW_7),
            Key::Key8 => c.set_view(&VIEW_8),
            Key::Key9 => c.set_view(&VIEW_9),
            Key::Key0 => c.set_view(&VIEW_0),
            Key::K => k.print(),
            Key::S => {
                let time_stamp = chrono::Utc::now().to_string();
                if config.window_scale == 1.0 {
                    p.save_as_png(&time_stamp, &c.get_view(), m, *image_supersampling_amount);
                } else {
                    let mut image_p: PixelBuffer = PixelBuffer::new(PixelPlane::new(config.image_width, config.image_height));
                    let mut image_c: ComplexPlane = ComplexPlane::new(config.image_width, config.image_height);
                    image_p.color_channel_mapping = p.color_channel_mapping;
                    image_c.set_view(&c.get_view());
                    rendering::render_complex_plane_into_buffer(&mut image_p, &image_c, m, *image_supersampling_amount, *coloring_function);
                    image_p.save_as_png(&time_stamp, &c.get_view(), m, *image_supersampling_amount);
                }
            }
            Key::I => c.set_view(&View::new(ask("x"), ask("y"), ask("scale"))),
            Key::A => {
                *coloring_function = pick_option(&[
                    ("HSV", TrueColor::new_from_hsv_colors),
                    ("Bernstein polynomials", TrueColor::new_from_bernstein_polynomials),
                ])
            }
            Key::M => m.max_iterations = ask("max_iterations"),
            Key::O => p.color_channel_mapping = ask("color_channel_mapping"),
            Key::Q => {
                *supersampling_amount = ask::<u8>("supersampling_amount").clamp(1, 64);
                *image_supersampling_amount = *supersampling_amount;
            }
            Key::X => *image_supersampling_amount = ask::<u8>("image_supersampling_amount").clamp(1, 64),
            Key::C => println!("{:?}", config),
            _ => (),
        }
        match key {
            Key::NumPadPlus | Key::NumPadMinus => println!("translation_amount: {}", vars.translation_amount),
            Key::NumPadSlash | Key::NumPadAsterisk => println!("scale factor: {}/{}", vars.scale_numerator, vars.scale_denominator),
            Key::Up | Key::Down | Key::Left | Key::Right => c.print(),
            Key::R
            | Key::Key1
            | Key::Key2
            | Key::Key3
            | Key::Key4
            | Key::Key5
            | Key::Key6
            | Key::Key7
            | Key::Key8
            | Key::Key9
            | Key::Key0
            | Key::LeftBracket
            | Key::RightBracket
            | Key::I
            | Key::A
            | Key::M
            | Key::O
            | Key::Q => {
                rendering::render_complex_plane_into_buffer(p, c, m, *supersampling_amount, *coloring_function);
                c.print();
            }
            _ => (),
        }
    }
}

fn was_clicked(current: bool, previous: bool) -> bool {
    current && !previous
}

fn handle_left_mouse_clicked(x: f32, y: f32, c: &ComplexPlane) {
    println!("\nMouseButton::Left -> Info at ({x}, {y})");
    //let iterations = p.iterations_at_point(x as usize, y as usize, m.max_iterations); //TODO: fix this
    let complex = c.complex_from_pixel_plane(x.into(), y.into());
    println!("Complex: {:?}", complex);
    //println!("iterations: {}", iterations);
    println!();
}

fn handle_right_mouse_clicked(
    x: f32,
    y: f32,
    c: &mut ComplexPlane,
    p: &mut PixelBuffer,
    m: &MandelbrotFunction,
    supersampling_amount: u8,
    coloring_function: ColoringFunction,
) {
    println!("\nMouseButton::Right -> Move to ({x}, {y})");
    let new_center = c.complex_from_pixel_plane(x.into(), y.into());
    println!("c.center: {:?}", c.center());
    println!("new_center: {:?}", new_center);

    rendering::translate_to_center_and_render_efficiently(c, p, m, &new_center, supersampling_amount, coloring_function);
    c.print();
    println!();
}

/////Mouse click recorder with interior mutability to toggle mouse clicks;
/// without such a (static function) variable, clicking the screen once would result in multiple actions
struct MouseClickRecorder {
    mouse_button: MouseButton,
    previous: AtomicBool,
}

impl MouseClickRecorder {
    pub const fn new(mouse_button: MouseButton) -> MouseClickRecorder {
        MouseClickRecorder {
            mouse_button,
            previous: AtomicBool::new(false),
        }
    }

    ///Returns whether the `mouse_button` was clicked once
    pub fn was_clicked(&self, window: &Window) -> bool {
        let current = window.get_mouse_down(self.mouse_button);
        let previous = self.previous.load(Ordering::Relaxed);
        let result = was_clicked(current, previous);
        if current != previous {
            self.previous.store(current, Ordering::Relaxed)
        }
        result
    }
}

fn handle_mouse_events(
    window: &Window,
    c: &mut ComplexPlane,
    p: &mut PixelBuffer,
    m: &MandelbrotFunction,
    supersampling_amount: u8,
    coloring_function: ColoringFunction,
) {
    static LEFT_MOUSE_RECORDER: MouseClickRecorder = MouseClickRecorder::new(MouseButton::Left); //Static variable with interior mutability to toggle mouse clicks; without such a variable, clicking the screen once would result in multiple actions
    static RIGHT_MOUSE_RECORDER: MouseClickRecorder = MouseClickRecorder::new(MouseButton::Right);

    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
        //Left mouse actions
        if LEFT_MOUSE_RECORDER.was_clicked(window) {
            handle_left_mouse_clicked(x, y, c);
        }

        //Right mouse actions
        if RIGHT_MOUSE_RECORDER.was_clicked(window) {
            handle_right_mouse_clicked(x, y, c, p, m, supersampling_amount, coloring_function);
        }
    }
}

///Prints Mandelbrot ASCII art :) </br>
///Prints the `application_banner`, `author_banner`, and `version`
fn print_banner() {
    //Made using: https://patorjk.com/software/taag/#p=display&f=Big&t=Mandelbrot
    let application_banner = r"
__  __                 _      _ _               _   
|  \/  |               | |    | | |             | |  
| \  / | __ _ _ __   __| | ___| | |__  _ __ ___ | |_ 
| |\/| |/ _` | '_ \ / _` |/ _ \ | '_ \| '__/ _ \| __|
| |  | | (_| | | | | (_| |  __/ | |_) | | | (_) | |_ 
|_|  |_|\__,_|_| |_|\__,_|\___|_|_.__/|_|  \___/ \__|";
    //Made using: https://patorjk.com/software/taag/#p=display&f=Small%20Slant&t=by%20Jort
    let author_banner = r"
   __             __         __ 
  / /  __ __  __ / /__  ____/ /_
 / _ \/ // / / // / _ \/ __/ __/
/_.__/\_, /  \___/\___/_/  \__/ 
     /___/                      ";
    let version = VERSION;
    println!("{}{}v{}\n\n", application_banner, author_banner, version);
}

///Prints a command info tip for the users benefit
fn print_command_info() {
    let tip = "Run Mandelbrot using:";
    let command = "cargo run --release -- <width> <height> <max_iterations> <supersampling_amount> <window_scale>";
    let command_info = "where <arg> means substitute with the value of arg\nuse '-' to use the default value of arg";
    println!("{}\n\t{}\n{}\n", tip, command, command_info);
}

///Holds all the logic currently in the main function that isn't involved with setting up configuration or handling errors, to make `main` concise and
///easy to verify by inspection
/// # Panics
/// Will panic if minifb cannot open a Window
/// # Errors
/// Currently does not return any Errors
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Complex plane dimensions and increments
    let mut c = ComplexPlane::new(config.window_width, config.window_height);
    // Pixel plane and buffer
    let mut p = PixelBuffer::new(PixelPlane::new(config.window_width, config.window_height));
    // User interaction variables
    let mut vars = InteractionVariables::default();
    // Multithreading variables
    let amount_of_threads = num_cpus::get(); //Amount of CPU threads to use, TODO: use this value in rendering functions
                                             // Mandelbrot set iterator
    let mut m: MandelbrotFunction = MandelbrotFunction::new(config.max_iterations, config.orbit_radius);
    //Coloring function
    let mut coloring_function = COLORING_FUNCTION;
    //Color channel mapping
    p.color_channel_mapping = COLOR_CHANNEL_MAPPING;
    //SSAA multiplier
    let mut supersampling_amount = config.supersampling_amount;
    //Image SSAA multiplier
    let mut image_supersampling_amount = supersampling_amount;
    // Create a new window
    let mut window = Window::new(
        "Mandelbrot set viewer",
        config.window_width,
        config.window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    //Print the banner
    print_banner();
    //Print command info
    print_command_info();
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
    key_bindings.add(
        Key::LeftBracket,
        "Scale the view by scaling_factor, effectively zooming in",
        empty_closure,
    );
    key_bindings.add(
        Key::RightBracket,
        "Scale the view by inverse_scaling_factor, effectively zooming out",
        empty_closure,
    );
    key_bindings.add(
        Key::V,
        "Prints the current Mandelbrot set view; the center and scale",
        empty_closure,
    );
    key_bindings.add(Key::Key1, "Renders VIEW_1", empty_closure);
    key_bindings.add(Key::Key2, "Renders VIEW_2", empty_closure);
    key_bindings.add(Key::Key3, "Renders VIEW_3", empty_closure);
    key_bindings.add(Key::Key4, "Renders VIEW_4", empty_closure);
    key_bindings.add(Key::Key5, "Renders VIEW_5", empty_closure);
    key_bindings.add(Key::Key6, "Renders VIEW_6", empty_closure);
    key_bindings.add(Key::Key7, "Renders VIEW_7", empty_closure);
    key_bindings.add(Key::Key8, "Renders VIEW_8", empty_closure);
    key_bindings.add(Key::Key9, "Renders VIEW_9", empty_closure);
    key_bindings.add(Key::Key0, "Renders VIEW_0", empty_closure);
    key_bindings.add(Key::K, "Prints the keybindings", empty_closure);
    key_bindings.add(
        Key::S,
        "Saves the current Mandelbrot set view as an image in the saved folder",
        empty_closure,
    );
    key_bindings.add(Key::I, "Manually input a Mandelbrot set view", empty_closure);
    key_bindings.add(Key::A, "Pick an algorithm to color the Mandelbrot set view", empty_closure);
    key_bindings.add(Key::M, "Change the Mandelbrot set view max_iterations", empty_closure);
    key_bindings.add(
        Key::O,
        "Change the Mandelbrot set view color channel mapping, xyz -> RGB, where x,y,z ∈ {{'R','G','B'}} (case-insensitive)",
        empty_closure,
    );
    key_bindings.add(
        Key::Q,
        "Change the window and image quality of the Mandelbrot set rendering by setting the SSAA multiplier, clamped from 1x to 64x",
        empty_closure,
    );
    key_bindings.add(
        Key::X,
        "Change the image quality of the Mandelbrot set rendering by setting the SSAA multiplier, clamped from 1x to 64x",
        empty_closure,
    );
    key_bindings.add(Key::C, "Prints the configuration variables", empty_closure);
    key_bindings.print();

    p.pixel_plane.print();
    c.print();
    println!(
        "Mandelbrot set parameters: max. iterations is {} and orbit radius is {}",
        config.max_iterations, config.orbit_radius
    );
    println!("Amount of CPU threads that will be used for rendering: {}", amount_of_threads);
    println!("Supersampling amount used for rendering: {}x", supersampling_amount);
    println!();

    println!("Rendering Mandelbrot set default view");
    rendering::render_complex_plane_into_buffer(&mut p, &c, &m, supersampling_amount, coloring_function);

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the window with the new buffer
        window
            .update_with_buffer(&p.pixels, config.window_width, config.window_height)
            .unwrap();

        // Handle any window events
        handle_key_events(
            &window,
            &mut c,
            &mut p,
            &mut m,
            &mut vars,
            &key_bindings,
            &mut supersampling_amount,
            &mut image_supersampling_amount,
            &mut coloring_function,
            config,
        );

        //Handle any mouse events
        handle_mouse_events(&window, &mut c, &mut p, &m, supersampling_amount, coloring_function);
    }

    Ok(())
}
