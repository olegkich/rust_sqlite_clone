use core::panic;
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

#[derive(Debug)]
enum StatementType {
  StatementInsert,
  StatementSelect,
  StatementUnsupported
}

#[derive(Debug)]
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
    statement.statement_type = StatementType::StatementInsert;
    return PrepareStatementResult::Success;
  }
  
  if buffer.starts_with("select") {
    statement.statement_type = StatementType::StatementSelect;
    return PrepareStatementResult::Success;
  }

  PrepareStatementResult::Unrecognized
  
}

fn execute_statement(statement: &Statement) {
  match statement.statement_type  {
    StatementType::StatementInsert => println!("inserting..."),
    StatementType::StatementSelect => println!("selecting..."),
    StatementType::StatementUnsupported => println!("unsupported statement."),
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
      PrepareStatementResult::Unrecognized => {println!("sqlok >> Unrecognized statement"); continue;},
      _ => execute_statement(&statement),
    }    
    
  }
}


