use std::io;
use std::io::Write;
use rand::prelude::*;

fn main() {
    let secret_number = random::<u8>();
    let mut guess: String = String::new();
    loop {
        print!("Guess a number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess).unwrap();
        guess.pop();
        let iguess: u8 = guess.parse().unwrap();
        if iguess < secret_number {
            println!("Too low!");
        } else if iguess > secret_number {
            println!("Too high!");
        } else if iguess == secret_number {
            println!("That's right!");
            break;
        }
        println!("You guessed: {}", &guess);
        guess = String::from("");
    }
}
