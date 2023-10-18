use std::io;
use std::fmt;

macro_rules! enum_error {
    ($x:ident, $($k:tt),*) => {
        #[derive(Debug)]
        enum $x {
            $($k,)*
        }
    }
}

macro_rules! impl_error_enum {
    ($x:ident, $($k:ident, $v:literal),*) => {
        enum_error!($x, $($k),*);
        impl fmt::Display for $x {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match &self {
                    $($x::$k => write!(f, $v),)*
                }
            }
        }
    }
}

impl_error_enum!(CustomError,
    SomeError, "Ohno",
    SomeOtherError, "Ohlawd",
    FileNotFound, "No such file",
    SomeIssue, "An issue"
);

fn main() -> Result<(), CustomError> {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s);
    s.pop();
    let x: u8 = match s.parse() {
        Ok(x) => x,
        Err(_) => panic!("{}", CustomError::SomeOtherError)
    };
    if x > 5 {
        println!("Raising an error");
        return Err(CustomError::SomeError);
    }
    println!("Hello, world!");
    Ok(())
}

