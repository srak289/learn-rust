fn main() {
    let r = rand::random_range(1..3+1);
    match r {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => panic!("Surprise"),
    }
}
