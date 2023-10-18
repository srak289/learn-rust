use std::io;

fn main() {
    let mut s: String = String::new();
    println!("Enter a non-negative integer");
    io::stdin().read_line(&mut s);
    s.pop();
    let x: u32 = s.parse().unwrap();

    s += " = ";

    // Write a programme which will print all the pairs of prime numbers whose sum equals
    // the number entered by the user. Eg 10 = 5 + 5, 7 + 3; 12 = 11 + 1, 5 + 7

    fn find_prime_sums(s: &mut String, x: u32) -> &mut String {
        fn find_or_cache(primes: &mut Vec<u32>) -> u32 {
            let mut p = primes[primes.len() - 1];
            let mut prime: bool = true;
            loop {
                p += 1;
                prime = true;
                for c in &mut *primes {
                    if (p as f32 / *c as f32).fract() == 0.0 {
                        prime = false;
                        break
                    }
                }
                if prime {
                    primes.push(p);
                    break
                }
            }
            p
        }

        let mut primes: Vec<u32> = Vec::from([2]);

        let mut p: u32 = 0;
        while p < x {
            p = find_or_cache(&mut primes);
            println!("Found prime: {}", p);
        }
        for i in &primes {
            for j in &primes {
                if i + j == x {
                    s.push_str(format!(" {} + {} ", i, j).as_str());
                }
            }
        }
        s
    }

    println!("{}", find_prime_sums(&mut s, x));
}
