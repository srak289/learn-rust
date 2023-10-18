use std::io;

fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut r: String = String::new();
    match io::stdin().read_line(&mut r) {
        Ok(_) => {},
        Err(_) => panic!("Failed to read string"),
    };
    r
}

fn main() {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let alphabet: Vec<_> = (97..123).map(|x| char::from_u32(x).unwrap()).collect();

    let x: String = read_line("Enter a string");

    let mut vcount: u32 = 0;
    let mut ccount: u32 = 0;

    for c in x.chars() {
        if VOWELS.contains(&c) {
            vcount += 1
        } else if !alphabet.contains(&c) {
            break
        } else {
            ccount += 1
        }
    }
    println!("Vowels: {}\nConsonants: {}", vcount, ccount);
}
