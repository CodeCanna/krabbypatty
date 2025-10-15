# Krabbypatty password generator
Hello, thanks for checking out krabbypatty password generator.  This is a simple generator that generates a 16 character long securily randomized password.  You can customize how long you want the password (no more than 16 characters), and you can even pass in characters you want the password generator to avoid.

## How to install
You can get petfacts by installing it through cargo `cargo install petfacts`.

## krabbypatty usage
* -e --exclude-chars <EXCLUDE_CHARS> Comma seperated characters you wish to exclude from generation "&, $, @"
* -l --length <LENGTH> Set the password genration length defaults to 16 characters [default: 16]
* -h --help Print Help
* -v --version Print Version

## Example
`krabbypatty` - Generates a secure random password.
`krabbypatty -l 8 --exclude-chars "@, 6, A"` - Generates a password 8 characters long excluding the characters @, 6, and A.
