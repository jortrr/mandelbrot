use std::{
    fmt::{self, Display},
    str::FromStr,
};

//Argument default values
static WIDTH: usize = 1200;
static HEIGHT: usize = 800;
static MAX_ITERATIONS: u32 = 1000;
static ORBIT_RADIUS: f64 = 2.0;
static SUPERSAMPLING_AMOUNT: u8 = 1;
static WINDOW_SCALE: f64 = 1.0;

#[derive(Clone, Copy)]
pub struct Config {
    // Window dimensions in pixels
    pub window_width: usize,
    pub window_height: usize,
    // Mandelbrot set parameters
    pub max_iterations: u32,
    pub orbit_radius: f64, //If z remains within the orbit_radius in max_iterations, we assume c does not tend to infinity
    // Rendering parameters
    pub supersampling_amount: u8,
    //Window scaling factor
    pub window_scale: f64,
    //Scaled window dimensions in pixels (used in images)
    pub image_width: usize,
    pub image_height: usize,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_width: WIDTH,
            window_height: HEIGHT,
            max_iterations: MAX_ITERATIONS,
            orbit_radius: ORBIT_RADIUS,
            supersampling_amount: SUPERSAMPLING_AMOUNT,
            window_scale: WINDOW_SCALE,
            image_width: WIDTH,
            image_height: HEIGHT,
        }
    }
    /// Parse the command line arguments from e.g. `env::args` in the following format
    /// ```ignore
    /// cargo run -- width height max_iterations
    /// ```
    /// # Errors
    /// Returns an Error if any of the given arguments couldn't be parsed into their types
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        args.next(); //Skip the first argument as it is the name of the executable

        //First argument
        let image_width = Config::parse_argument("width", args.next(), WIDTH)?;

        //Second argument
        let image_height = Config::parse_argument("height", args.next(), HEIGHT)?;

        //Third argument
        let max_iterations = Config::parse_argument("max_iterations", args.next(), MAX_ITERATIONS)?;

        //Fourth argument
        let supersampling_amount = Config::parse_argument("supersampling_amount", args.next(), SUPERSAMPLING_AMOUNT)?;

        //Fifth argument
        let window_scale = Config::parse_argument("window_scale", args.next(), WINDOW_SCALE)?;
        //Scale width and height
        let window_width = (f64::from(image_width as u32) * window_scale) as usize;
        let window_height = (f64::from(image_height as u32) * window_scale) as usize;

        Ok(Config {
            window_width,
            window_height,
            max_iterations,
            orbit_radius: ORBIT_RADIUS,
            supersampling_amount,
            window_scale,
            image_width,
            image_height,
        })
    }

    ///Parses an argument to a T value if possible, returns an error if not. Returns default if argument is None </br>
    ///If Some(arg) == "-", return default
    /// # Errors
    /// Return an Error if the given argument cannot be parsed to a T type
    pub fn parse_argument<T: FromStr + Display>(name: &str, argument: Option<String>, default: T) -> Result<T, String>
    where
        <T as std::str::FromStr>::Err: Display,
    {
        match argument {
            Some(arg) => {
                if arg == "-" {
                    Config::print_no_argument_given(name, &default);
                    return Ok(default);
                }
                match arg.parse::<T>() {
                    Ok(val) => Ok(val),
                    Err(err) => Err(err.to_string() + &format!(" for {} argument", name)),
                }
            }
            None => {
                Config::print_no_argument_given(name, &default);
                Ok(default)
            }
        }
    }

    pub fn print_no_argument_given<T: std::fmt::Display>(name: &str, default: &T) {
        println!("No {} argument given, using default: {}", name, default);
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //TODO: Improve debug printing format legibility
        f.debug_struct("Config")
            .field("window_width", &self.window_width)
            .field("window_height", &self.window_height)
            .field("max_iterations", &self.max_iterations)
            .field("orbit_radius", &self.orbit_radius)
            .field("supersampling_amount", &self.supersampling_amount)
            .field("window_scale", &self.window_scale)
            .field("image_width", &self.image_width)
            .field("image_height", &self.image_height)
            .finish()
    }
}
