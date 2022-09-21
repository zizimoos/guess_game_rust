use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("=== Guess the number ===");

    let secret_number = rand::thread_rng().gen_range(1..10);
    println!("The secret number is {}:",secret_number);

    loop{
        println!("please input your guess");

        let mut guess = String::new();
        io::stdin()
           .read_line(&mut guess)
           .expect("failed to read_line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
    
        println!("You guessed :{} ", guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Equal => {println!("{}","YOU WIN !".yellow()); break},
            Ordering::Greater => println!("{}","Too big".red()),
        }
    }
}
