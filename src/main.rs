mod btree;
mod statement;
mod statement_test;
mod table;

use crate::MetaCommandCode::MetaCommandUnknown;
use crate::PrepareStatementCode::{
    PrepareStatementFailure, PrepareStatementInsert, PrepareStatementSelect,
};
use crate::StatementCode::StatementSuccess;
use crate::statement::select;
use std::io;
use std::io::Error;
use std::process::exit;

enum StatementCode {
    StatementSuccess,
    StatementFailure,
}

enum PrepareStatementCode {
    PrepareStatementSelect,
    PrepareStatementInsert,
    PrepareStatementFailure,
}

#[derive(Debug)]
enum MetaCommandCode {
    MetaCommandSuccess,
    MetaCommandFailure,
    MetaCommandUnknown,
}

fn main() -> Result<(), Error> {
    while true {
        println!("----------------------------");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let input = buffer.strip_suffix("\n").unwrap();
        if input.starts_with(".") {
            let output = exec_meta_command(input)?;
            println!("Executed meta command with result: {output:?}");
        } else {
            let output = prepare_statement(input)?;
            exec_statement(input, output).expect("TODO: panic message");
        }
    }
    Ok(())
}

fn exec_meta_command(cmd: &str) -> Result<MetaCommandCode, Error> {
    if cmd == ".exit" {
        println!("Shutting down database.");
        exit(0);
    }
    Ok(MetaCommandUnknown)
}

fn prepare_statement(cmd: &str) -> Result<PrepareStatementCode, Error> {
    if cmd.starts_with("select") && cmd.len() > 6 {
        return Ok(PrepareStatementSelect);
    } else if cmd.starts_with("insert") && cmd.len() > 6 {
        return Ok(PrepareStatementInsert);
    }
    Ok(PrepareStatementFailure)
}

fn exec_statement(cmd: &str, statment_type: PrepareStatementCode) -> Result<StatementCode, Error> {
    match statment_type {
        PrepareStatementSelect => {
            let _ = select(cmd);
            println!("This is a select Statement");
        }
        PrepareStatementInsert => {
            println!("This is a insert Statement");
            let _ = statement::insert(cmd);
        }
        PrepareStatementFailure => {
            println!("Statement failed to be classified")
        }
    }
    Ok(StatementSuccess)
}
