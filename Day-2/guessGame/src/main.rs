use std::io;//, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{

        // println!("The secret number is: {secret_number}");

        println!("Please input your guess.");

        let mut guess = String::new();//In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change

        io::stdin()//it append that into a string (without overwriting its contents),like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable. 
            .read_line(&mut guess)//& indicates that this argument is a reference,
            .expect("Failed to read line");//If you don’t call expect, the program will compile, but you’ll get a warning:

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        // println!("You guessed: {}", guess);
        // or
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {// It takes a reference to whatever you want to compare with:: here it’s comparing guess to secret_number. Then it returns a variant of the Ordering enum
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
            
}