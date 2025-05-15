use std::io::{self, Write, stdin};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed == "quit" {
                    println!("exit.");
                    break;
                }
            }
            Err(err) => {
                eprintln!("Error reading from stdin: <{err}>");
                break;
            }
        }

        let line = line.trim();
        println!("this was received <{line}>");
    }
}
