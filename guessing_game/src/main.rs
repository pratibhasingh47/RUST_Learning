use std::io;
use std::cmp::Ordering;
use colored::*;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_no = rand::thread_rng().gen_range(1, 100);

    println!("The secret no. is : {}",secret_no);

    loop{

        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guesses: {}", guess);
    
        match guess.cmp(&secret_no){
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Greater => println!("{}","Too Big".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
        }
    }
}
