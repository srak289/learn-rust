use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the amount of fibonacci you want:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut n: u32 = input.trim_end().parse().expect(&format!("Failed to parse u32 from {}", input));
    let mut c1: u32 = 1;
    let mut c2: u32 = 0;

    while n > 0 {
        n -= 1;
        print!("{} ", c1);
        if c2 == 0 {
            c2 = 1;
        } else {
            c1 = c1 + c2;
            c2 = c1 - c2;
        }
    }
    println!();
}
