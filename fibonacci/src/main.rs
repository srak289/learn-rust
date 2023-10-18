use std::io;

fn read_int() -> i32 {
    println!("Enter an integer: ");
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.pop();
    x.parse().unwrap()
}

fn main() {
    let max: i32 = read_int();
    let (mut p1, mut p2, mut pc): (i32, i32, i32) = (0, 0, 1);
    while pc < max {
        println!("{}", pc);
        p2 = p1;
        p1 = pc;
        pc = p1 + p2;
    }
}
