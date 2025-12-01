use std::io;

fn main() {
    let mut data = String::new();
    let mut xrate = String::new();
    println!("Enter amount:");
    io::stdin().read_line(&mut data).expect("Failed to read stdin");
    let dollars: u32 = data.find('.').map(|x| &data[..x]).expect("Failed to find '.' in amount").parse().expect("Failed to parse u32");
    println!("Using dollar amt {}", dollars);
    let cents: u32 = data.find('.').map(|x| &data[x + 1..]).expect("Failed to find '.' in amount").trim_end().parse().expect("Failed to parse u32");
    println!("Using cents amt {}", cents);
    println!("Enter exchange rate as a float from 0.0 to 1.0");
    io::stdin().read_line(&mut xrate).expect("Failed to read stdin");
    let xrate: f32 = xrate.trim_end().parse().expect("Failed to parse f32");
    let xdollars = dollars as f32 * xrate;
    let xcents = cents as f32 * xrate;
    println!("You have {}.{}", xdollars as i32, xcents as i32);
}
