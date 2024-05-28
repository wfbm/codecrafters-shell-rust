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

        let parsed: Vec<&str> = input.trim().split_whitespace().collect();
        let command = parse_command(parsed);

   } 
}

pub enum Builtin {
    Echo { handle: String, content: String },
    Exit { handle: String, code: i32 },
    Type { handle: String, command: String },
    Invalid,
}

fn parse_command(command: Vec<&str>) -> Builtin {

    match command.as_slice() {
        ["echo", args @ ..] => Builtin::Echo{ handle: "echo".to_string(), content: args.join(" ") },
        ["exit", code] => Builtin::Exit{ handle: "exit".to_string(), code: code::i32<>() },
        ["type", command] => Builtin::Type{ handle: "type".to_string(), command: command.to_string() },
        _ => Builtin::Invalid,
    }
}
