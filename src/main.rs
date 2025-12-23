use rand::Rng;
use std::env::args;
use std::process::exit;

const LOWER:  u8         = 0b0000_0001;
const UPPER:  u8         = 0b0000_0010;
const DIGITS: u8         = 0b0000_0100;
const SYMBOLS: u8        = 0b0000_1000;
const EXTENDED_ASCII: u8 = 0b0001_0000;

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

fn create_password(len: usize, flags: u8) -> String {
    let mut password = String::new();
    for _ in 0..len {
        password.push(get_random_char(flags));
    }
    password
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
            let flags : u8 = LOWER | UPPER | DIGITS | SYMBOLS;
            if args.len() == 4 {
                unimplemented!();
            }
            println!("{}", create_password(length, flags));
        },
        "evaluate" | "eval" | "e" => {
            unimplemented!();
        },
        _ => {
            eprintln!("Unknown command");
        }
    }
}
