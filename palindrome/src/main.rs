use std::io;

fn check_palindrome(s: &String) -> bool {
    match s.len() {
        0 => {
            true
        },
        _ => {
            let chars: Vec<char> = s.chars().collect();
            for i in 0..(s.len()/2 as usize) {
                if chars[i] != chars[chars.len()-1-i] {
                    return false
                }
            }
            return true
        },
    }
}

fn main() {
    let mut s: String = String::new();
    println!("Enter a string: ");
    match io::stdin().read_line(&mut s) {
        Ok(_) => {},
        Err(_) => panic!("Oh no"),
    };
    s.pop();
    println!("{}", check_palindrome(&s));
}
