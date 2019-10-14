use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter a temperature: ");
        io::stdout().flush().unwrap();

        let mut original_temp = String::new();
        io::stdin()
            .read_line(&mut original_temp)
            .expect("failed to read!");
        let original_temp: f64 = match original_temp.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        };

        print!("Is it fahrenheit (f) or celsius (c)? ");
        io::stdout().flush().unwrap();

        let mut temp_scale = String::new();
        io::stdin()
            .read_line(&mut temp_scale)
            .expect("failed to read!");
        match temp_scale.trim() {
            "f" | "F" | "f°" | "F°" => {
                let final_temp = fahrenheit_to_celsius(original_temp);
                println!("The result is {}C°", final_temp);
            }
            "c" | "C" | "c°" | "C°" => {
                let final_temp = celsius_to_fahrenheit(original_temp);
                println!("The result is {}F°", final_temp);
            }
            _ => {
                println!("Invalid temperature scale. Enter f or c!");
                continue;
            }
        };
    }
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    ((9.0 / 5.0) * temp) + 32.0
}
