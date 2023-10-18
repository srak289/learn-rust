// List all possible combinations of letters in a 4-letter word. Eg 'TEST' can be unscrambled as TEST, TETS, TSET, TSTE, TTSE, TTES, etc.
use std::io;

fn main() {
    let mut s = String::new();
    match io::stdin().read_line(&mut s) {
        Ok(_) => {},
        Err(_) => panic!("Oh no!"),
    };
    s.pop();
    if s.len() != 4 {
        panic!("{} should be exactly four characters", &s);
    }
    println!("The string {} is {} chars", &s, &s.len());

    let ours: Vec<char> = s.chars().collect();

    let mut mask: u16 = 0;

    let mut c = Vec::<char>::new();

    for i in 0..=65535u16 {
        for j in (0..3).rev() {
            mask = 0xf<<(4*j);
            c.push(ours[<u16 as Into<usize>>::into((i&mask)>>(4*j)%4)]);
        }
        println!("{:?}", &c);
        c = [].to_vec();
    }
}
