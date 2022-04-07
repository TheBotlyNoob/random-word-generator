use dictionary2::DICTIONARY;
use rand::seq::SliceRandom;
use std::io::{stdin, stdout, Error as IoError, Write};

fn main() {
    let words = input("How many words do you want? ")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let words = DICTIONARY
        .choose_multiple(&mut rand::thread_rng(), words)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", words);
}

fn input(prompt: impl AsRef<str>) -> Result<String, IoError> {
    print!("{}", prompt.as_ref());
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}
