pub struct Config {
    pub width: usize,
    pub height: usize,
    pub max_iterations: u32,
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

        Ok(Config {width, height, max_iterations})
    }
}
