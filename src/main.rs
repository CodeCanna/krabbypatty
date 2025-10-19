use clap::Parser;
use rand::Rng;
use rand::prelude::*;
use rand::rngs::StdRng;

/// krabbypatty password generator, securely generate passwords
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Comma seperated characters you wish to exclude from generation "&, $, @"
    #[arg(short, long)]
    exclude_chars: Option<String>,

    /// Set the password genration length defaults to 16 characters
    #[arg(short, long, default_value_t = 16)]
    length: u8,
}

/// Remove the characters passed by the user and replace them with another generated character
fn sanitize_password(password: String, chars: Vec<char>) -> String {
    let mut sanitized_string = String::default();
    for mut c in password.chars() {
        for ch in chars.clone() {
            if c == ch {
                c = char::from_u32(random_ascii()).unwrap(); // Set character to a new value
            }
        }
        sanitized_string.push(c);
    }
    sanitized_string
}

/// Parse the passed string of comma seperated characters
fn parse_exclude_chars(ec: String) -> Vec<char> {
    let mut char_arr = vec![];
    for c in ec.chars() {
        if c == ',' || c == ' ' {
            continue;
        }
        char_arr.push(c);
    }
    char_arr
}

/// Generate a random printable ascii character code
fn random_ascii() -> u32 {
    StdRng::from_rng(&mut rand::rng()).random_range(32..126) // Only go to 126 because 127 is the DEL character
}

fn main() {
    let args = Args::parse();
    let mut password: String = String::default();

    // Generate the password
    for _ in 0..args.length {
        password.push(char::from_u32(random_ascii()).unwrap());
    }

    // Now it needs to sanitized of any characters that the user doesn't want generated,
    if let Some(exclude_chars) = args.exclude_chars.as_deref() {
        password = sanitize_password(password, parse_exclude_chars(String::from(exclude_chars)));
    }
    println!("{}", password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_password_test() {
        let dirty_string = "abc123$*(";
        let sanitized_string = sanitize_password(String::from(dirty_string), vec!['$', '*', '(']);

        assert!(!sanitized_string.contains('$'));
        assert!(!sanitized_string.contains('*'));
        assert!(!sanitized_string.contains('('));
    }

    #[test]
    fn parse_exclude_chars_test() {
        let exclude_test_chars = vec!['*', ')', '@'];
        let exclude_chars = String::from("*, ), @");

        assert_eq!(exclude_test_chars, parse_exclude_chars(exclude_chars));
    }

    #[test]
    fn random_ascii_test() {
        let val = random_ascii() as u8;
        assert!(val.is_ascii());
        assert!(val.is_ascii_graphic() || val == b' ');
    }
}
