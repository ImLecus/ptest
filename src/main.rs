use rand::Rng;
use std::env::args;
use std::process::exit;
use libm::{log2, pow, powf};

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

fn calculate_entropy(password: &str, flags: u8) -> f32 {
    let l = password.len() as f32;
    let r = (((flags & LOWER) as u32 * 26) +
                ((flags & UPPER) as u32 * 26) +
                ((flags & DIGITS) as u32 * 10) +
                ((flags & SYMBOLS) as u32 * 32) +
                ((flags & EXTENDED_ASCII) as u32 * 0)) as f64;

    l * (log2(r) as f32)
}

fn calculate_crack_time(entropy: f32) -> f64 {
    let estimate_attempts = pow(2.0, entropy as f64 - 1.0);
    let attempts_per_second = pow(10.0, 9.0);

    estimate_attempts / attempts_per_second
}

fn get_composition(password: &str) -> (f32, f32, f32, f32, f32) {
    let mut composition = (0.0, 0.0, 0.0, 0.0, 0.0);
    let unit  = 1.0 / password.len() as f32;
    for c in password.chars() {
        if c.is_ascii_lowercase() {
            composition.0 += unit;
        }
        else if c.is_ascii_uppercase() {
            composition.1 += unit;
        }
        else if c.is_ascii_digit() {
            composition.2 += unit;
        }
        else if c.is_ascii_graphic() {
            composition.3 += unit;
        }
        else {
            composition.4 += unit;
        }
    }
    composition
}

fn find_common_words(password: &str) -> Vec<String> {
    unimplemented!();
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
            if args.len() < 3 {
                eprintln!("Usage: ptest evaluate <password>");
                exit(1);
            }
            let password = &args[2];
            let composition = get_composition(password);
            println!("Password composition:");
            println!("- {:.1}% lowercase characters", composition.0 * 100.0);
            println!("- {:.1}% uppercase characters", composition.1 * 100.0);
            println!("- {:.1}% digit characters", composition.2 * 100.0);
            println!("- {:.1}% symbol characters", composition.3 * 100.0);
            println!("- {:.1}% ascii extended characters", composition.4 * 100.0);
            let mut flags = 0;
            if composition.0 > 0.0 {flags |= LOWER;}
            if composition.1 > 0.0 {flags |= UPPER;}
            if composition.2 > 0.0 {flags |= DIGITS;}
            if composition.3 > 0.0 {flags |= SYMBOLS;}
            if composition.4 > 0.0 {flags |= EXTENDED_ASCII;}
            let entropy = calculate_entropy(password, flags);
            println!("Entropy: {} bits", entropy);

            let time = calculate_crack_time(entropy);
            if time > 315576000000.0 {
                println!("Estimated crack time: infinite");
            }
            else if time > 31557600.0 {
                println!("Estimated crack time: {} years", time / 31557600.0);
            }
            else if time > 86400.0 {
                println!("Estimated crack time: {} days", time / 86400.0);
            }
            else if time > 3600.0 {
                println!("Estimated crack time: {} hours", time / 3600.0);
            }
            else if time > 60.0 {
                println!("Estimated crack time: {} minutes", time / 60.0);
            }
            else {
                println!("Estimated crack time: {} seconds", time);
            }
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
