use std::io;

fn main() {
    println!("Enter the triangle base: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.pop();

    let base: u8 = input.parse().unwrap();

    for x in 1..=base {
        if x % 2 == 0 {
            continue
        }
        println!("{}", 
        {
            "*".repeat(x.into())
        });
    }
}
