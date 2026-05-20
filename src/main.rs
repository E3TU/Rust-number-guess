use rand::RngExt;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 50");

    let mut rng = rand::rng();

    let number = rng.random_range(1..50);
    let mut attempts: i32 = 0;

    loop{
        println!("Enter your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won in {} attempts!", attempts);
                break;
            }
        }
    }
}
