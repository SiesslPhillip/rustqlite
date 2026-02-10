use crate::table::{Table, insert_row, Row, fetch_row};

#[derive(Debug)]
pub enum InsertError {
    NotEnoughArgs { got: usize, expected: usize },
}

#[derive(Debug)]
pub enum SelectError {
    NotEnoughArgs { got: usize, expected: usize },
}

pub fn insert(cmd: &str) -> Result<i32, InsertError> {
    let row_to_insert: Vec<&str> = cmd
        .strip_prefix("insert ")
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    if row_to_insert.len() < 3 as usize {
        return Err(InsertError::NotEnoughArgs {
            got: row_to_insert.len(),
            expected: 3,
        });
    } else {
        let id: i32 = row_to_insert[0].parse::<i32>().unwrap();
        insert_row(id, row_to_insert[1], row_to_insert[2]);
        return Ok(id);
    }
    Ok(0)
}

pub fn select(cmd: &str) -> Result<i32, SelectError> {
    let row_to_select: Vec<&str> = cmd
        .strip_prefix("select ")
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    if row_to_select.len() < 1 as usize {
        return Err(SelectError::NotEnoughArgs {
            got: row_to_select.len(),
            expected: 1,
        });
    } else {
        let id: i32 = row_to_select[0].parse::<i32>().unwrap();
        match fetch_row(id) {
            Ok(row) => {
                println!("{row:?}");
                return Ok(id)
            },
            Err(err) => print!("Row does not exist.")
        }
    }
    Ok(0)
}
