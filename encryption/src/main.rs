use std::fmt;
use std::fs;
use clap::Parser;


#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    encrypt: bool,
    #[arg(short, long)]
    decrypt: bool,
    #[arg(long)]
    file_path: String,
    #[arg(long)]
    key: Option<String>,
    #[arg(long)]
    key_file: Option<String>,
    #[arg(long)]
    out_file: Option<String>,
}

#[derive(Debug)]
enum KeyError {
    LengthError,
    ExistsError,
    NoKeyError,
}

#[derive(Debug)]
enum EncryptionError {
    KeyError(KeyError),
    DecryptionError,
    EncryptionError,
}

#[derive(Debug)]
struct Key {
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct CipherText {
    bytes: Vec<u8>,
    key: Option<Key>,
}

#[derive(Debug)]
struct PlainText {
    bytes: Vec<u8>,
    key: Option<Key>,
}


impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyError::LengthError => write!(f, "The key is too short"),
            KeyError::ExistsError => write!(f, "The key already exists"),
            KeyError::NoKeyError => write!(f, "The key does not exist"),
        }
    }
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncryptionError::EncryptionError => write!(f, "Encryption failed"),
            EncryptionError::DecryptionError => write!(f, "Decryption failed"),
            EncryptionError::KeyError(e) => e.fmt(f),
        }
    }
}

impl Key {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bytes: bytes,
        }
    }

    fn from_str(s: &str) -> Self {
        Self {
            bytes: s.into(),
        }
    }

    fn xor(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let mut ki: usize = 0;
        let mut buf: Vec<u8> = Vec::new();
        for i in 0..bytes.len() {
            if ki >= self.bytes.len() {
                ki = 0;
            }
            buf.push(bytes[i] ^ self.bytes[ki]);
            ki += 1;
        }
        buf
    }
}

impl CipherText {
    fn from_plain_text(pt: &PlainText) -> Result<Self, EncryptionError> {
        match &pt.key {
            Some(k) => {
                Ok(Self {
                    bytes: k.xor(&pt.bytes),
                    key: None,
                })
            },
            None => Err(EncryptionError::KeyError(KeyError::NoKeyError)),
        }
    }

    fn decrypt(&self) -> Result<PlainText, EncryptionError> {
        match &self.key {
            Some(k) => {
                Ok(PlainText::from_bytes(k.xor(&self.bytes)))
            },
            None => Err(EncryptionError::KeyError(KeyError::NoKeyError))
        }
    }

    fn write_file(&self, s: &String) -> Result<(), EncryptionError> {
        fs::write(&s, &self.bytes);
        Ok(())
    }
}

impl PlainText {
    fn from_bytes(s: Vec<u8>) -> Self {
        Self {
            bytes: s,
            key: None,
        }
    }

    fn from_str(s: String) -> Self {
        Self {
            bytes: s.into(),
            key: None,
        }
    }

    fn from_file(s: &String) -> Self {
        let data = fs::read_to_string(s).expect("Error reading file");
        Self {
            bytes: data.into(),
            key: None,
        }
    }

    fn encrypt(&self) -> Result<CipherText, EncryptionError> {
        match &self.key {
            Some(_) => {
                CipherText::from_plain_text(&self)
            },
            None => {
                Result::Err(EncryptionError::KeyError(KeyError::NoKeyError))
            }
        }
    }

    fn add_key(&mut self, key: Key) -> Result<(), KeyError> {
        match &self.key {
            Some(_) => Err(KeyError::ExistsError),
            None => {
                self.key = Some(key);
                Ok(())
            },
        }
    }
}

fn main() {
    let args = Args::parse();
    let data = fs::read_to_string(&args.file_path).expect("Error reading file");
    println!("{}", data);
    let key = Key::from_str("foo");
    let mut pt = PlainText::from_file(&args.file_path);
    pt.add_key(key);
    let ct = pt.encrypt();
    println!("{:?}", ct);
    ct.unwrap().write_file(&args.out_file.unwrap());
}
