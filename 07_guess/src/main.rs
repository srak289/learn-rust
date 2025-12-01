use std::io;

use rand::Rng;
use log::{Record, Level, Metadata, LevelFilter, SetLoggerError, info, debug};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, md: &Metadata) -> bool {
        md.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&SimpleLogger).map(|()| log::set_max_level(LevelFilter::Info))
}

fn main() {
    init().expect("Failed to setup logging");

    info!("Enter main");
    let secret = rand::random::<u8>();
    debug!("The secret is {secret}");
    println!("The secret is {secret}");
    let mut guess = String::new();
    loop {
        println!("Enter your guess:");
        io::stdin().read_line(&mut guess).expect("Failed to read stdin");
        match guess.trim_end().parse::<u8>() {
            Ok(x) => {
                if x == secret {
                    println!("You guessed the secret");
                    break;
                }
                guess.truncate(0);
                println!("Try again");
            }
            Err(x) => {
                println!("That is not a valid guess {x}");
            }
        }
    }
    info!("Finished main");
}
