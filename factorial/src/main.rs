use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.pop();
    let r: u64 = input.parse().unwrap();
    println!("You entered {}", r);
    fn factorial(n: u64) -> u64 {
        if n < 2 {
            1
        } else {
            n * factorial(n-1)
        }
    }
    println!("The factorial is {}", factorial(r));
    Ok(())
}
