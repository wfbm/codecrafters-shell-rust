#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::{env, fs, process};

fn main() {
    let mut ctx = ShellContext {
        current_dir: env::current_dir()
            .expect("should have a valid dir")
            .display()
            .to_string(),
    };

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            eprintln!("Failed to read stdin");
            continue;
        }

        let parsed: Vec<&str> = input.trim().split_whitespace().collect();
        if let Some(command) = parse_command(parsed) {
            command.execute(&mut ctx);
        } else {
            println!("Invalid command {}", input.trim());
        }
    }
}

enum Builtin {
    Echo { content: Option<String> },
    Exit { code: Option<i32> },
    Type { command: Option<String> },
    Cd { path: Option<String> },
    Pwd,
    Invalid { attempt: String },
}

struct ShellContext {
    current_dir: String,
}

trait ShellCommand {
    fn handle(&self) -> &str;
    fn execute(&self, ctx: &mut ShellContext);
}

impl ShellCommand for Builtin {
    fn handle(&self) -> &str {
        match self {
            Builtin::Echo { .. } => "echo",
            Builtin::Exit { .. } => "exit",
            Builtin::Type { .. } => "type",
            Builtin::Pwd => "pwd",
            Builtin::Cd { .. } => "cd",
            Builtin::Invalid { attempt } => attempt,
        }
    }

    fn execute(&self, ctx: &mut ShellContext) {
        match self {
            Builtin::Echo { content } => println!("{}", content.as_deref().unwrap_or("")),
            Builtin::Exit { code } => process::exit(code.unwrap_or(0)),
            Builtin::Type { command } => {
                if let Some(cmd_str) = command {
                    let parsed = cmd_str.split_whitespace().collect();
                    if let Some(cmd) = parse_command(parsed) {
                        match cmd {
                            Builtin::Invalid { attempt } => {
                                if let Some(exe) = find_exe(&attempt) {
                                    println!("{attempt} is {exe}");
                                } else {
                                    println!("{attempt}: not found");
                                }
                            }
                            _ => println!("{} is a shell builtin", cmd.handle()),
                        }
                    } else {
                        println!("invalid command");
                    }
                } else {
                    println!("type: missing operand");
                }
            }
            Builtin::Cd { path } => {
                if let Some(choosen_path) = path {
                    let choosen_path = choosen_path.replace(
                        "~",
                        &home::home_dir()
                            .expect("should have a home dir")
                            .display()
                            .to_string(),
                    );

                    if Path::new(&choosen_path).exists() {
                        ctx.current_dir = choosen_path.to_string();
                    } else {
                        println!("cd: {}: No such file or directory", choosen_path);
                    }
                } else {
                    println!("you must inform a path");
                }
            }
            Builtin::Pwd => {
                println!("{}", ctx.current_dir);
            }
            Builtin::Invalid { attempt } => {
                let output = execute(&attempt);

                if output.status.success() {
                    io::stdout().write_all(&output.stdout).unwrap();
                } else {
                    println!("{attempt}: command not found");
                }
            }
        }
    }
}

fn parse_command(command: Vec<&str>) -> Option<Builtin> {
    match command.as_slice() {
        ["echo", args @ ..] => Some(Builtin::Echo {
            content: if args.is_empty() {
                None
            } else {
                Some(args.join(" "))
            },
        }),
        ["exit", code @ ..] => Some(Builtin::Exit {
            code: code.first().and_then(|c| c.parse::<i32>().ok()),
        }),
        ["type", command @ ..] => Some(Builtin::Type {
            command: if command.is_empty() {
                None
            } else {
                Some(command.join(" "))
            },
        }),
        ["cd", path] => Some(Builtin::Cd {
            path: Some(path.to_string()),
        }),
        ["pwd"] => Some(Builtin::Pwd),
        [] => None,
        _ => Some(Builtin::Invalid {
            attempt: command.join(" "),
        }),
    }
}

fn find_exe(name: &str) -> Option<String> {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            if let Some(file) = find_file_in_path(name, path) {
                return Some(file);
            }
        }
    } else {
        println!("PATH not accessible");
    }

    return None;
}

fn find_file_in_path(file: &str, dir: PathBuf) -> Option<String> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_dir() {
                if let Some(file_found) = path.file_name()?.to_str() {
                    if file_found == file {
                        return Some(path.to_str()?.to_string());
                    }
                }
            }
        }
    }

    return None;
}

fn execute(cmd: &str) -> Output {
    return Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to run {cmd}");
}
