use clap::{arg, value_parser, Command, ArgGroup, ArgAction};
use std::io;
use std::io::Write;

fn print_stars(rows: u8, up: bool, odd: bool) {
    let mut stars: u8 = 0;
    if !up {
        stars = rows;
    }
    let mut row: u8 = rows;
    let mut stdout = io::stdout();
    while row > 0 {
        if up {
            stars += 1;
        } else {
            stars -= 1;
        }
        row -= 1;
        if odd && row % 2 == 1 {
            continue;
        }
        for _ in 1..stars {
            let _ = stdout.write(&[b'*']);
        }
        let _ = stdout.write(&[b'\n']);
        let _ = stdout.flush();
    }
}

fn main() {
    let ap = Command::new("triangles")
        .version("0.1.0")
        .about("prints triangles")
        .arg_required_else_help(true)
        .arg(arg!(<amount> "The number of rows").value_parser(value_parser!(u8)))
        .arg(arg!(--odd "Only odd rows").action(ArgAction::SetTrue))
        .arg(arg!(--up "Pointing up").action(ArgAction::SetTrue))
        .arg(arg!(--down "Pointing down").action(ArgAction::SetTrue))
        .group(
            ArgGroup::new("direction")
            .required(true)
            .args(["up", "down"])
        )
        .get_matches();
    println!("Printing {:?} stars", ap.get_one::<u8>("amount"));
    println!("Printing {:?} up", ap.get_one::<bool>("up"));
    println!("Printing {:?} down", ap.get_one::<bool>("down"));
    println!("Printing {:?} odd", ap.get_one::<bool>("odd"));
    print_stars(
        *ap.get_one::<u8>("amount").unwrap(),
        *ap.get_one::<bool>("up").unwrap(),
        *ap.get_one::<bool>("odd").unwrap(),
    );
}
