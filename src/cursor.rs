use std::thread::current;
use crate::table::Table;

struct Cursor {
    table: Table,
    row_num: usize,
    end_of_table: bool
}

#[derive(Debug, PartialEq)]
pub enum CursorError {
    CreationError,
}

impl Cursor {
    fn new(table: Table) -> Result<Cursor, CursorError> {
        return Ok(Self{table, row_num: 0, end_of_table: false })
    }
    fn table_start(mut self) {
        self.end_of_table = self.table.num_rows == 0;
    }

    fn table_end(mut self) {
        self.row_num = self.table.num_rows;
        self.end_of_table = true;
    }

    fn advance(mut self) {
        self.row_num += 1;
        if self.row_num >= self.table.num_rows {
            self.end_of_table = true;
        }
    }
}