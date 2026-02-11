// mod btree;
mod cursor;
mod persistence;
mod statement;
mod statement_test;
mod table;

use crate::MetaCommandCode::MetaCommandUnknown;
use crate::PrepareStatementCode::{
    PrepareStatementFailure, PrepareStatementInsert, PrepareStatementSelect,
};
use crate::StatementCode::StatementSuccess;
use crate::statement::{select, InsertError};
use crate::table::{Table};
use std::io;
use std::io::Error;
use std::process::exit;
use clap::Parser;
use crate::cursor::Cursor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the database
    #[arg(short, long)]
    database: String,
}

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
        let args = Args::parse();
        let result = Cursor::new(&mut Table::db_open(&args.database)?).table.db_close();
        match result {
            Ok(res) => println!("Flushed to disk complete!"),
            Err(err) => {}
        }
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

fn exec_statement(cmd: &str, statement_type: PrepareStatementCode) -> Result<StatementCode, Error> {
    let args = Args::parse();
    let mut table = Table::db_open(&args.database)?;
    let curr = &mut Cursor::new(&mut table);
    match statement_type {
        PrepareStatementSelect => {
            let _ = select(curr, cmd);
        }
        PrepareStatementInsert => {
            let _ = statement::insert(curr, cmd);
            let result = curr.table.db_close();
            match result {
                Ok(_) => {},
                Err(err) => {
                    println!("Error flushing after insert!");
                    return Err(err)
                }
            }
        }
        PrepareStatementFailure => {
            println!("Statement failed to be classified")
        }
    }
    Ok(StatementSuccess)
}
