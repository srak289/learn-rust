use std::io;

fn read_float(prompt: &str) -> f32 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input.parse().unwrap()
}

fn main() {
    let c: f32 = read_float("Enter an amount: ");
    let x: f32 = read_float("Enter an exchange rate: ");
    println!("You have {} {}", c * x, "dollars");
}
