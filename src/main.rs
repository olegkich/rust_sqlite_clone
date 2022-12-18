use std::{io::{self, Write}, process::exit, default};

// hardcoded row
#[derive(Default)]
struct Row {
  id: usize,
  name: String,
  email: String
}

enum MetaCommandResult {
  Success,
  Unrecognized,
}

enum PrepareStatementResult { 
  Success,
  Unrecognized,
  SyntaxError
}

#[derive(Default)]
enum StatementType {
  #[default]
  StatementInsert,
  StatementSelect,
}

#[derive(Default)]
struct Statement {
  statement_type: StatementType,
  row_to_insert: Row
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

    let mut args = buffer.split_whitespace();

    // mother of god, this is ugly
    let _ = args.next();
    let id: String = if let Some(s) = args.next() { s.to_string() } else { return PrepareStatementResult::SyntaxError; };

    let name: String = if let Some(s) = args.next() { s.to_string() } else { return PrepareStatementResult::SyntaxError; };
    
    let email: String = if let Some(s) = args.next() { s.to_string() } else { return PrepareStatementResult::SyntaxError; };

    println!("inserting given values: {} {} {}", id, name, email);

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
    StatementType::StatementInsert => {println!("inserting...");},
    StatementType::StatementSelect => {println!("selecting...");},
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

    let mut statement: Statement = Statement::default();

    match prepare_statemenet(&buffer, &mut statement) {
      PrepareStatementResult::Unrecognized => {println!("sqlok >> Unrecognized statement"); continue;}
      PrepareStatementResult::SyntaxError => {println!("sqlok >> Syntax Error."); continue;},
      _ => execute_statement(&statement),
    }    
    
  }
}


