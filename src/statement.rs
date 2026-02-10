use crate::table::{Table, get_row_by_id, insert_row, Row};

#[derive(Debug)]
pub enum InsertError {
    NotEnoughArgs { got: usize, expected: usize },
}

#[derive(Debug)]
pub enum SelectError {
    NotEnoughArgs { got: usize, expected: usize },
}

pub fn insert(cmd: &str) -> Result<i64, InsertError> {
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
        let id: i64 = row_to_insert[0].parse::<i64>().unwrap();
        insert_row(id, row_to_insert[1], row_to_insert[2]);
        return Ok(id);
    }
    Ok(0)
}

pub fn select(cmd: &str) -> Result<i64, SelectError> {
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
        let id: i64 = row_to_select[0].parse::<i64>().unwrap();
        match get_row_by_id(id) {
            Some(row) => {
                println!("{row:?}");
                return Ok(id)
            },
            None => print!("Row does not exist.")
        }
    }
    Ok(0)
}
