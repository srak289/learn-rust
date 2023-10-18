use rand::prelude::*;

fn one() {
    println!("Hello from function one");
}

fn two() {
    println!("Hello from function two");
}

fn three() {
    println!("Hello from function three");
}

fn main() {
    let s: u8 = (random::<u8>() % 3) + 1;
    match s {
        1 => one(),
        2 => two(),
        3 => three(),
        _ => println!("None"),
    }
}
