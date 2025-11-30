use std::io;

fn factorial(x: u32) -> u32 {
    if x > 1 {
        return x * factorial(x-1);
    }
    return 1;

}

fn main() {
    let mut data = String::new();

    println!("Enter an integer:");
    io::stdin().read_line(&mut data).expect("Failed to read line");
    let x: u32 = data.trim_end().parse().expect(&format!("Failed to parse integer {}", data));
    println!("{}", factorial(x));
}
