use std::{str::FromStr, fmt::Display};

//Argument default values
static WIDTH: usize = 1200;
static HEIGHT: usize = 800;
static MAX_ITERATIONS: u32 = 1000;
static ORBIT_RADIUS: f64 = 2.0;
static SUPERSAMPLING_AMOUNT: u8 = 1;
static WINDOW_SCALE: f64 = 1.0;

pub struct Config {
    // Window dimensions in pixels
    pub width: usize,
    pub height: usize,
    // Mandelbrot set parameters
    pub max_iterations: u32,
    pub orbit_radius: f64,      //If z remains within the orbit_radius in max_iterations, we assume c does not tend to infinity
    // Rendering parameters
    pub supersampling_amount: u8,
    //Window scaling factor
    pub window_scale: f64,
}


impl Config {
    /// Parse the command line arguments from e.g. env::args() in the following format
    /// ```
    /// cargo run -- width height max_iterations
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        args.next(); //Skip the first argument as it is the name of the executable

        //First argument
        let mut width = Config::parse_argument("width", args.next(), WIDTH).unwrap(); 

        //Second argument
        let mut height = Config::parse_argument("height", args.next(), HEIGHT).unwrap();

        //Third argument
        let max_iterations = Config::parse_argument("max_iterations", args.next(), MAX_ITERATIONS).unwrap();

        //Fourth argument
        let supersampling_amount = Config::parse_argument("supersampling_amount", args.next(), SUPERSAMPLING_AMOUNT).unwrap();

        //Fifth argument
        let window_scale = Config::parse_argument("window_scale", args.next(), WINDOW_SCALE).unwrap();
        let resolution_needs_to_scale = window_scale != 1.0;
        if resolution_needs_to_scale {
            //Scale width and height
            width = (width as f64 * window_scale) as usize;
            height = (height as f64 * window_scale) as usize;
        }

        Ok(Config {width, height, max_iterations, orbit_radius: ORBIT_RADIUS, supersampling_amount, window_scale})
    }

    ///Parses an argument to a T value if possible, returns an error if not. Returns default if argument is None </br>
    ///If Some(arg) == "-", return default
    pub fn parse_argument<T: FromStr + Display>(name: &str, argument: Option<String>, default: T) -> Result<T, String> 
    where <T as std::str::FromStr>::Err: Display{
        match argument {
            Some(arg) => {
                if arg == "-" {
                    Config::print_no_argument_given(name, &default);
                    return Ok(default);
                }
                match arg.parse::<T>() {
                    Ok(val) => return Ok(val),
                    Err(err) => return Err(err.to_string() + &format!(" for {} argument", name)),
                }
            },
            None =>  {
                Config::print_no_argument_given(name, &default);
                Ok(default)
            }
        }
    }

    pub fn print_no_argument_given<T: std::fmt::Display>(name: &str, default: &T) {
        println!("No {} argument given, using default: {}", name, default);
    }
}
