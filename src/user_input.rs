use std::{str::FromStr, fmt::Display, io::{self, Write}};

/// Ask the user for `result` named `name` from stdin. If `result` can be parsed to a `T`, return `result`. In any other case,
/// call `ask` again.
/// # Panics
/// If `io::stdout().flush().unwrap()` panics
pub fn ask<T: FromStr + Display>(name: &str) -> T
    where <T as FromStr>::Err: Display {
        print!("Enter the {}:", name);
        io::stdout().flush().unwrap();

        let input = get_user_input();

        //Try to convert input to a T, or ask again
        return match input.parse::<T>() {
            Ok(value) => value,
            Err(err) => {
                println!("\tError: {}: {}", err, input);
                ask(name)
            }
        }
}

///Reads a line from stdin, does not trim the input
pub fn get_user_input_untrimmed() -> String {
    let mut user_input = String::new();
    // Read the user's input from the standard input stdin
    io::stdin()
    .read_line(&mut user_input)
    .expect("Error: Failed to read the user's input from stdin.");

    user_input
}

///Reads a line from stdin, trims the input, removes trailing and leading whitespaces
pub fn get_user_input() -> String {
    let user_input = get_user_input_untrimmed();
    String::from(user_input.trim())
}

///Lets the user pick an option from a `Vec` of options. Returns the picked option. Will ask for an option until a valid option is given
///on stdin.
pub fn pick_option<T: Copy>(options: Vec<(&str, T)>) -> T {
    println!("Please pick an option:");
    for (i, option) in options.iter().enumerate() {
        println!("\t[{}]: {}",i,option.0);
    }
    
    let mut input: usize = ask("option");
    while input >= options.len() {
        println!("\tError: the option is out of bounds: {}", input);
        input = ask("option");
    }
    //Now we know the user has picked a valid option
    let (name, value) = options[input];
    println!("\tPicked: {}", name);
    value
}