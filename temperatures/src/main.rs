use std::io;
use std::{thread, time};


fn fahr_to_cels () {

    loop {

        println!("Please write the Fahrenheit temperature you want to convert to Celsius:");
    
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You didn't entered a number!");
                thread::sleep(time::Duration::from_secs(1));
                continue;
            }
        };
        let temp_conv = (temp - 32.0) * (5.0/9.0);
        println!("The temperature converted to Celsius is: {:.4}\n", temp_conv);
        break;
    }
}

fn cels_to_fahr () {

    loop {

        println!("Please write the Celsius temperature you want to convert to Fahrenheit:");
    
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You didn't entered a number!");
                thread::sleep(time::Duration::from_secs(1));
                continue;
            }
        };
        let temp_conv = temp * (9.0/5.0) + 32.0;
        println!("The temperature converted to Fahrenheit is: {:.4}\n", temp_conv);
        break;
    }
}

fn main() {
    println!("Hello, world! This is a Fahrenheit and Celsius temperature converter!\n");
    loop {
        println!("Enter your choice (1 => Fahrenheit to Celsius    2 -- Celsius to Fahrenheit): ");
    
        let mut choice = String::new();
    
        io::stdin().read_line(&mut choice)
        .expect("Failed to read line");
    
        match choice.trim().parse() {
            Ok(num) => { match num {
                1 => {
                    println!("You chose to convert Fahrenheit to Celsius!");
                    fahr_to_cels();
                }
                2 => {
                    println!("You chose to convert Celsius to Fahrenheit!");
                    cels_to_fahr();
                }
                _ => println!("You have to choose between 1 and 2..."),
                }
                num
            }
            Err(_) => {
                println!("Only number 1 and 2 are allowed!!");
                thread::sleep(time::Duration::from_secs(1));
                continue;
            }
        };
        println!("Press return to go back to menu");
        io::stdin().read_line(&mut choice)
        .expect("Failed to read line");
            
    }

}
