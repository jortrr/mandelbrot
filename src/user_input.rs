use std::{str::FromStr, fmt::Display, io::{self, Write}};

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

pub fn get_user_input_untrimmed() -> String {
    let mut user_input = String::new();
    // Read the user's input from the standard input stdin
    io::stdin()
    .read_line(&mut user_input)
    .expect("Error: Failed to read the user's input from stdin.");

    user_input
}

pub fn get_user_input() -> String {
    let user_input = get_user_input_untrimmed();
    String::from(user_input.trim())
}