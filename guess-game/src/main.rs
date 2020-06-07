use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");

    // Generate a random number between 1 and 10
    let answer = rand::thread_rng().gen_range(1, 11);

    loop {
        // Get number from stdin
        println!("Please guess a number between 1 and 10.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Integer");

        // Convert stdin string to u32 integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);

        // Compare number and random number
        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small, try again!"),
            Ordering::Greater => println!("Too big, try again!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number!");
                break;
            }
        }
    }
}
