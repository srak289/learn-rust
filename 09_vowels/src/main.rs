use std::io;

fn main() {
    let mut data = String::new();
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    io::stdin().read_line(&mut data).expect("Failed to read stdin");
    let mut vows: u8 = 0;
    let mut cons: u8 = 0;
    for c in data.trim_end().chars() {
        if vowels.contains(&c) {
            vows += 1;
        } else {
            cons += 1;
        }
    }
    println!("String: {} - Vowels: {} - Consonants: {}", data.trim_end(), vows, cons);
}
