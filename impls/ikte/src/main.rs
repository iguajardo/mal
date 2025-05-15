use std::io::{self, Write, stdin};

// mods are filed called like this or folders with a mod.rs file
// you can declare sub modules in folder modules. You need to make them pub in the main module
// file. Like pub mod parser.
mod processor;

// use is different. It allows you to import something from a module. Making it usable without
// using the name of the module.

fn main() {
    let mut line = String::new();
    loop {
        print!("> ");
        // always push the changes from buffer to stdout
        // is_err only if need to check for err and ignore Ok
        if io::stdout().flush().is_err() {
            eprintln!("Failed to flush stdout");
            break;
        }

        line.clear();
        match stdin().read_line(&mut line) {
            // complex match with if. If condition not pass, continue
            Ok(n) if n == 0 => {
                // EOF: CTRL+D
                println!("EOF received.");
                break;
            }
            Ok(_) => {
                if line.trim() == "quit" {
                    println!("exit.");
                    break;
                }
                println!("this was received <{line}>");
                // add processing here. Create new module. It should receive result and print it
                processor::process_command(&line);
            }
            Err(err) => {
                eprintln!("Error reading from stdin: <{err}>");
                break;
            }
        }
    }
}
