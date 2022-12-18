// part 1, simple REPL
use std::{io::{self, Write}, process::exit};

enum MetaCommandResult {
  Success,
  Unrecognized
}

enum PrepareStatementResult { 
  Success,
  Unrecognized
}

enum StatementType {
  StatementInsert,
  StatementSelect,
  StatementUnsupported
}

struct Statement {
  statement_type: StatementType,
}

fn print_prompt() {print!("sqlok >> ");}

fn execute_meta_command(command: &String) -> MetaCommandResult {
  if command.eq(".exit") {
    exit(0);
  } else {
    MetaCommandResult::Unrecognized
  }
}

fn prepare_statemenet(buffer: &String, statement: &mut Statement) -> PrepareStatementResult {
  if buffer.starts_with("insert") {
    println!("insert");
    statement.statement_type = StatementType::StatementInsert;
    PrepareStatementResult::Success
  } else if buffer.starts_with("select") {
    println!("select");
    PrepareStatementResult::Success
  } else {
    PrepareStatementResult::Unrecognized
  }
}



fn main() {
  let stdin = io::stdin();
  
  loop {
    let mut buffer = String::new();

    print_prompt();
    io::stdout().flush().unwrap();
    
    stdin.read_line(&mut buffer).unwrap();
    buffer = buffer.trim().to_string();
    
    if buffer.starts_with(".") {
      match execute_meta_command(&buffer) {
        MetaCommandResult::Unrecognized => {println!("sqlok >> Unrecognized command"); continue;},
        _ => continue,
      }
    } 

    let mut statement: Statement = Statement { statement_type: StatementType::StatementUnsupported};

    match prepare_statemenet(&buffer, &mut statement) {
      PrepareStatementResult::Success => println!("success"),
      PrepareStatementResult::Unrecognized => println!("sqlok >> Unrecognized statement"),
    }
  }
}


