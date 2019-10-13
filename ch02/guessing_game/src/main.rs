use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess a number!");

    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read!");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, baby!"),
            Ordering::Greater => println!("Too big, man!"),
            Ordering::Equal => {
                println!("Right on!");
                break;
            }
        }
    }
}
