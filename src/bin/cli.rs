//! # Rustupolis CLI
//!
//! A tuple space client implementation.
//! Ultimately this will work offline as a self-sufficient tuple space server as well as by
//! connecting to a remote tuple space server.
//!

// TODO list:
//  - input parsing loop
//  - processing of parsed commands

extern crate rustupolis;

use std::io;
use std::io::Write;

fn main() {
    println!("Rustupolis CLI");
    let mut cli = Cli::new(io::stdin(), io::stdout());
    cli.run()
}

enum RequiredAction {
    CLOSE,
    DETACH,
    NONE,
}

struct Cli {
    stdin: io::Stdin,
    stdout: io::Stdout,
    input: String,
}

impl Cli {
    fn new(stdin: io::Stdin, stdout: io::Stdout) -> Cli {
        Cli {
            stdin,
            stdout,
            input: String::new(),
        }
    }

    fn run(&mut self) {
        use self::RequiredAction::*;
        loop {
            print!("> ");
            self.stdout.flush().expect("failed to flush stdout");
            self.stdin
                .read_line(&mut self.input)
                .expect("failed to read input");
            let required_action = process_input(self.input.trim());
            // reset input
            self.input.clear();
            // TODO: implement proper actions
            match required_action {
                CLOSE => break,
                DETACH => break,
                NONE => {}
            }
        }
    }
}

/// User input should always consist of a pre-defined command and user-defined parameters,
/// separated by whitespaces.
///
/// Ideas for pre-defined commands:
///
/// - `create` - create new tuple space \ tuple space server
/// - `close` or `quit` - tear down the tuple space and terminate the program
/// - `detach` - close the CLI, but keep the tuple space server running in the background
///
// TODO: Keep the list updated.
fn process_input(input: &str) -> RequiredAction {
    use self::RequiredAction::*;

    println!("user echo: {}", input);
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return NONE;
    }

    let command = tokens.get(0);
    match command {
        Some(&"create") => cmd_create(&tokens[1..]),
        Some(&"close") => cmd_close(),
        Some(&"detach") => cmd_detach(),
        _ => NONE,
    }
}

fn cmd_create(parameters: &[&str]) -> RequiredAction {
    println!("creation parameters:");
    for p in parameters {
        println!("{}", p)
    }
    RequiredAction::NONE
}

fn cmd_close() -> RequiredAction {
    RequiredAction::CLOSE
}

fn cmd_detach() -> RequiredAction {
    RequiredAction::DETACH
}
