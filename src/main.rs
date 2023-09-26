// Brief intro to basic Rust concepts
// usage of built-in lib std
// usage of external lib rand
// usage of println! macro
// mutability and shadowing
// explicit type annotation and conversion
// pattern matching and match arms with enum variants and Result type
// loop, continue and break

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let rand_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Input your guess!");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
