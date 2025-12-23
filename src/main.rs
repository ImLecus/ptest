use rand::Rng;
use std::env::args;
use std::process::exit;

const LOWER:  u8         = 0b0000_0001;
const UPPER:  u8         = 0b0000_0010;
const DIGITS: u8         = 0b0000_0100;
const SYMBOLS: u8        = 0b0000_1000;
const EXTENDED_ASCII: u8 = 0b0001_0000;

/// Returns a random character in a range determined by the flags
fn get_random_char(flags: u8) -> char {
    let mut pool = String::new();

    if flags & LOWER          != 0 { pool.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if flags & UPPER          != 0 { pool.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if flags & DIGITS         != 0 { pool.push_str("0123456789"); }
    if flags & SYMBOLS        != 0 { pool.push_str("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"); }
    if flags & EXTENDED_ASCII != 0 {
        unimplemented!();
    }
    let index = rand::rng().random_range(0..pool.len());

    pool.chars().nth(index).unwrap()
}

/// Creates a password with a determined length
fn create_password(len: usize, flags: u8) -> String {
    let mut password = String::new();
    for _ in 0..len {
        password.push(get_random_char(flags));
    }
    password
}

/// Parses the flags from format '-(flag)+' to a single byte
fn parse_flags(flags_str: &str) -> u8 {
    let mut flags : u8 = 0;

    if flags_str.chars().nth(0).unwrap_or('\0') != '-' || flags_str.len() < 2 {
        eprintln!("Unknown flag: {}", flags_str);
        exit(1);
    }
    for c in flags_str.chars().skip(1) {
        match c {
            'l' => flags |= LOWER,
            'u' => flags |= UPPER,
            'd' => flags |= DIGITS,
            's' => flags |= SYMBOLS,
            'e' => flags |= EXTENDED_ASCII,
            _ => {
                eprintln!("Unknown flag: {}", c);
                exit(1);
            }
        }
    }
    flags
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ptest <command>");
        exit(1);
    }

    let command = args[1].as_str();

    match command {
        "generate" | "gen" | "g" => {
            if args.len() < 3 {
                eprintln!("Usage: ptest generate <length> <flags>");
                exit(1);
            }
            let length = args[2].parse::<usize>().unwrap();
            let mut flags: u8 = LOWER | UPPER | DIGITS | SYMBOLS;
            if args.len() == 4 {
                flags = parse_flags(&args[3]);
            }
            println!("{}", create_password(length, flags));
        },
        "evaluate" | "eval" | "e" => {
            unimplemented!();
        },
        "--version" => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        },
        _ => {
            eprintln!("Unknown command");
            exit(1);
        }
    }
}
