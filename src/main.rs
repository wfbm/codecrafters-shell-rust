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
        if stdin.read_line(&mut input).is_err() {
            eprintln!("Failed to read stdin");
            continue;
        }

        let parsed: Vec<&str> = input.trim().split_whitespace().collect();
        if let Some(command) = parse_command(parsed) {
            command.execute();
        } else {
            println!("Invalid command {}", input.trim());
        }
   } 
}

enum Builtin {
    Echo { content: Option<String> },
    Exit { code: Option<i32> },
    Type { command: Option<String> },
    Invalid { attempt: String },
}

trait Command {
    fn handle(&self) -> &str;
    fn execute(&self);
}

impl Command for Builtin {
    fn handle(&self) -> &str {
        match self {
            Builtin::Echo { .. } => "echo",
            Builtin::Exit { .. } => "exit",
            Builtin::Type { .. } => "type",
            Builtin::Invalid { attempt } => attempt,
        }
    }

    fn execute(&self) {
        match self {
            Builtin::Echo{ content } => println!("{}", content.as_deref().unwrap_or("")),
            Builtin::Exit{ code } => process::exit(code.unwrap_or(0)),
            Builtin::Type{ command } => {
                if let Some(cmd_str) = command {
                    let parsed = cmd_str.split_whitespace().collect();
                    if let Some(cmd) = parse_command(parsed) {
                        match cmd {
                            Builtin::Invalid { attempt } => println!("{attempt} not found"),
                            _ => println!("{} is a shell builtin", cmd.handle()),
                        }
                    } else {
                        println!("invalid command");
                    }
                } else {
                    println!("type: missing operand");
                }
            },  
            Builtin::Invalid{ attempt } => println!("{attempt}: command not found"),
        }
    }
}

fn parse_command(command: Vec<&str>) -> Option<Builtin> {

    match command.as_slice() {
        ["echo", args @ ..] => Some(Builtin::Echo{ 
            content: if args.is_empty() { None } else { Some(args.join(" ")) },
        }),
        ["exit", code @ ..] => Some(Builtin::Exit{ 
            code: code.first().and_then(|c| c.parse::<i32>().ok()),
        }),
        ["type", command @ ..] => Some(Builtin::Type{ 
            command: if command.is_empty() { None } else { Some(command.join(" ")) },
        }),
        [] => None,
        _ => Some(Builtin::Invalid { 
            attempt: command.join(" "),
        }),
    }
}

