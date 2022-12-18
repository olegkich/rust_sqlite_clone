// part 1, simple REPL
use std::{io::{self, Write}};

fn print_prompt() {print!("sqlok >> ");}
fn main() {
  let stdin = io::stdin();
  

  loop {
    let mut buffer = String::new();

    print_prompt();
    io::stdout().flush().unwrap();
    
    stdin.read_line(&mut buffer).unwrap();
    buffer = buffer.trim().to_string();
    
    if buffer.eq(".exit") {
      println!("Exiting...");
      break;
    } else {
      println!("Unrecongizable command: {}", buffer)
    }
  }
}


