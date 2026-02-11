use crate::table::{Page, Table, ROWS_PER_PAGE, ROW_SIZE, TABLE_MAX_PAGES};

pub struct Cursor<'a> {
    pub(crate) table: &'a mut Table,
    pub(crate) row_num: usize,
    end_of_table: bool,
}

impl<'a> Cursor<'a> {
    pub fn new(table: &'a mut Table) -> Self {
        let end_of_table = table.num_rows == 0;
        Self { table, row_num: 0, end_of_table }
    }

    pub fn table_end(table: &'a mut Table) -> Self {
        let row_num = table.num_rows;
        Self { table, row_num, end_of_table: true }
    }

    pub fn advance(&mut self) {
        self.row_num += 1;
        if self.row_num >= self.table.num_rows {
            self.end_of_table = true;
        }
    }

    pub fn value(&mut self) -> &mut Page {
        let page_num = self.row_num / ROWS_PER_PAGE;
        assert!(page_num < TABLE_MAX_PAGES);

        self.table.get_page_mut(page_num)
    }

    pub fn byte_offset(&self) -> usize {
        let row_offset = self.row_num % ROWS_PER_PAGE;
        row_offset * ROW_SIZE
    }
}
