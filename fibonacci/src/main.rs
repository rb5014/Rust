use std::io;

fn nth_fib(number: i32) -> u128{
    let mut sum:u128 = 0;
    let mut last:u128 = 0;
    let mut curr:u128 = 1;

    for _ in 0..number {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

fn main() {
    println!("Hello, world! Welcome to the \"Nth fibonacci number calculator!!\"");

    loop {
        println!("Please enter a positive number greater than 0: ");
    
        let mut number = String::new();
    
        io::stdin().read_line(&mut number).expect("Failed to read line");
    
        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You didn't enter a number...");
                continue;
            }
        };

        if number < 0 {
            println!("Can't generate the nth Fibonacci number with a negative number...");
            continue;
        }
        match number {
            0 => {
                println!("Can't generate the nth Fibonacci number with a 0...");
                continue;
            }
            1 => {println!("1");}
            _ => {
                println!("The {}th Fibonacci number is:\n{}", number, nth_fib(number));
            }
        }
        println!("\nPress Return to try again");
        let mut ret = String::new();
        io::stdin().read_line(&mut ret).expect("Failed to read line");

    }
}
