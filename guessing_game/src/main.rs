use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::{thread, time};

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            // parse turns any string into a number and returns a Result type: enum with variants Ok or Err 
            Ok(num) => num,
            Err(_) => {
                println!("You didn't entered a number!");
                thread::sleep(time::Duration::from_secs(1));
                continue;
            }
        };
            

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
