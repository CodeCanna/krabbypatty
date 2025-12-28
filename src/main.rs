use clap::Parser;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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
fn sanitize_password(password: String, exclude_chars: String) -> String {
    let mut sanitized_string = String::default();
    for mut c in password.chars() {
        // Generate a new character until it's a different character from the exclude character
        while exclude_chars.contains(c) {
            c = char::from_u32(random_ascii()).unwrap();
        }
        sanitized_string.push(c);
    }
    sanitized_string
}

/// Parse the passed string of comma seperated characters
fn parse_exclude_chars(ec: String) -> Vec<char> {
    let ec = ec.replace(" ", "");
    let mut char_arr = vec![];
    for c in ec.chars() {
        char_arr.push(c);
    }
    char_arr
}

/// Generate a random printable ascii character code
fn random_ascii() -> u32 {
    StdRng::from_rng(&mut rand::rng()).random_range(32..126) // Only go to 126 because 127 is the DEL character
}

fn generate_password(length: u8, exclude_chars: Option<String>) -> String {
    let mut password: String = String::default();
    // Generate the password
    for _ in 0..length {
        let _ = &mut password.push(char::from_u32(random_ascii()).unwrap());
    }

    // Now it needs to sanitized of any characters that the user doesn't want generated,
    if let Some(exclude_chars) = exclude_chars {
        password = sanitize_password(password, String::from(exclude_chars));
    }
    password
}

fn main() {
    let args = Args::parse();
    println!("{}", generate_password(args.length, args.exclude_chars));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_password_test() {
        let dirty_string = "abc123$*(";
        let sanitized_string = sanitize_password(String::from(dirty_string), String::from("$*("));
        assert!(!sanitized_string.contains('$'));
        assert!(!sanitized_string.contains('*'));
        assert!(!sanitized_string.contains('('));
    }

    #[test]
    fn parse_exclude_chars_test() {
        let exclude_test_chars = vec!['*', ')', '@'];
        let exclude_chars = String::from("* ) @");

        assert_eq!(exclude_test_chars, parse_exclude_chars(exclude_chars));
    }

    #[test]
    fn random_ascii_test() {
        let val = random_ascii() as u8;
        assert!(val.is_ascii());
        assert!(val.is_ascii_graphic() || val == b' ');
    }

    #[test]
    fn generate_password_test() {
        let exclude_chars = String::from("abcde");
        let password = generate_password(10, Some(exclude_chars.clone()));
        assert!(!password.is_empty());
        assert_eq!(password.len(), 10);

        for c in exclude_chars.chars() {
            assert!(!password.contains(c));
        }
    }
}
