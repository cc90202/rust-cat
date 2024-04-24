/// cat utility
/// concatena più file passati come parametri
/// da riga di comando
/// author: Cristiano Chieppa
/// date: Aprile 2024

use std::env;
use std::fs;
use text_colorizer::Colorize;

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len()  == 0 {
        eprintln!("{} almeno un file va specificato. {}",
                  "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    let mut cat = String::new();

    for arg in args
    {
        //println!("{}", arg);
        let content = match fs::read_to_string(&arg)
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{} failed to read from file '{}': {:?}",
                          "Error:".red().bold(), arg, e);
                std::process::exit(1);
            }
        };

        match content.len() > 0
        {
            true => {cat.push_str(&content)},
            false => {
                eprintln!("{} {} è vuoto",
                          "Error:".red().bold(), arg);
                std::process::exit(1);
            }
        };
    }

    eprintln!("{}", cat);
}
