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
                c = char::from_u32(random_ascii() as u32).unwrap(); // Set character to a new value
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
fn random_ascii() -> u8 {
    let mut rng = StdRng::from_rng(&mut rand::rng());
    rng.random_range(32..127) as u8
}

fn main() {
    let args = Args::parse();
    let mut password: String = String::default();

    // Generate the password
    for _ in 0..args.length {
        let random_byte = random_ascii() as u32;
        password.push(char::from_u32(random_byte).unwrap());
    }

    // Now it needs to sanitized of any characters that the user doesn't want generated,
    if let Some(exclude_chars) = args.exclude_chars.as_deref() {
        password =
            sanitize_password(password, parse_exclude_chars(String::from(exclude_chars)));
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
        let test_val: u8 = 10;
        assert_eq!(
            std::any::type_name_of_val(&random_ascii()),
            std::any::type_name_of_val(&test_val)
        );
    }
}
