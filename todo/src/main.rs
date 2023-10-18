extern crate todo;

use std::env;
use clap::Parser;

fn main() {
    let mut vargs: Vec<String> = env::args().collect();
    vargs.remove(0);

    if vargs.len() > 0 {
        let args = todo::cli::Args::parse();
        println!("{:?}", args);
    } else {
        todo::repl::enter();
    }
}
