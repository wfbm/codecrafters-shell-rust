#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    
    loop {
     
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();
        let parsed: Vec<&str> = command.split_whitespace().collect();

        match parsed[..] {
            ["exit", code] => process::exit(code.parse::<i32>().unwrap()),
            _ => println!("{command}: command not found"),
        }
    } 
}

