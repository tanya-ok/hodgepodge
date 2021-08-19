use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0..101);

    loop {
        println!("Please enter your guess...");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
        println!("Your number: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number is too small."),
            Ordering::Greater => println!("The number is too large."),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
