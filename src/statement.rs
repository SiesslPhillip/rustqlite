use crate::cursor::Cursor;
use crate::table::{fetch_row, insert_row};

#[derive(Debug, PartialEq)]
pub enum InsertError {
    NotEnoughArgs { got: usize, expected: usize },
    FlushError
}

#[derive(Debug)]
pub enum SelectError {
    NotEnoughArgs { got: usize, expected: usize },
    OutOfBounds,
}

pub fn insert(curr: &mut Cursor,cmd: &str) -> Result<i32, InsertError> {
    let row_to_insert: Vec<&str> = cmd
        .strip_prefix("insert ")
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    if row_to_insert.len() < 3usize {
        Err(InsertError::NotEnoughArgs {
            got: row_to_insert.len(),
            expected: 3,
        })
    } else {
        let id: i32 = row_to_insert[0].parse::<i32>().unwrap();
        insert_row(curr, id, row_to_insert[1], row_to_insert[2]);
        Ok(id)
    }
}

pub fn select(curr: &mut Cursor, cmd: &str) -> Result<i32, SelectError> {
    let row_to_select: Vec<&str> = cmd
        .strip_prefix("select ")
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    if row_to_select.len() < 1usize {
        return Err(SelectError::NotEnoughArgs {
            got: row_to_select.len(),
            expected: 1,
        });
    } else {
        let id: i32 = row_to_select[0].parse::<i32>().unwrap();
        match fetch_row(curr, id) {
            Ok(row) => {
                let name = std::str::from_utf8(&row.name).unwrap();
                let email = std::str::from_utf8(&row.email).unwrap();
                let id = row.id;
                println!("ID: {id}; name: {name}; email: {email}");
                return Ok(id);
            }
            Err(_) => print!("Row does not exist."),
        }
    }
    Ok(0)
}
