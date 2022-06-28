use dictionary2::DICTIONARY;
use rand::seq::SliceRandom;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let words = if let Some(words) = {
        let mut args = std::env::args();
        let _ = args.next();
        args.next()
    } {
        words
    } else {
        print!("How many words do you want? ");
        stdout().flush()?;
        let mut words = String::new();
        stdin().read_line(&mut words)?;
        words
    };

    let words = words.trim().parse::<usize>()?;

    println!("\n\n\n\n\n");

    let words = DICTIONARY
        .choose_multiple(&mut rand::thread_rng(), words)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");

    println!("{words}");

    Ok(())
}
