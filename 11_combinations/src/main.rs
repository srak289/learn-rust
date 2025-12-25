use std::io;

fn permute(buf: String) {
    let mut b = Vec::from(buf);
    println!("{:?}", b);
    let mut res = Vec<String>::new();
    fn rpermute(v: Vec, n: u8) {
    }
    rpermute(b, 0);
    for a in res {
        println!("{a}");
    }
}

fn main() {
    println!("Enter string:");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf);
    buf = buf.trim_end().into();
    permute(buf);
}
