use std::io::{self, Write, stdin};

fn main() {
    let mut line = String::new();
    loop {
        print!("> ");
        if io::stdout().flush().is_err() {
            eprintln!("Failed to flush stdout");
            break;
        }

        line.clear();
        match stdin().read_line(&mut line) {
            // complex match with if. If condition not pass, continue
            Ok(n) if n == 0 => {
                println!("EOF received.");
                break;
            }
            Ok(_) => {
                if line.trim() == "quit" {
                    println!("exit.");
                    break;
                }
                println!("this was received <{line}>");
            }
            Err(err) => {
                eprintln!("Error reading from stdin: <{err}>");
                break;
            }
        }
    }
}
