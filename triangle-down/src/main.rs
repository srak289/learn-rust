use std::io;

fn main() {
    println!("Enter the triangle base: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop();
    let mut base: u8 = input.parse().unwrap();
    while base > 0 {
        fn layer(x: u8) -> String {
            let mut l: String = String::new();
            for _ in 0..x {
                // l.push('*');
                l += "*";
            }
            l
        }
        println!("{}", layer(base));
        base -= 1;
    }
}
