#[cfg(test)]
mod tests {
    use std::fs;
    use crate::cursor::Cursor;
    use crate::statement::{InsertError, insert, select};
    use crate::table::Table;

    #[test]
    fn insert_returns_error_if_not_enough_args() {
        let test_database_name = String::from("test_db");
        let mut table = Table::db_open(&test_database_name).unwrap();
        let curr = &mut Cursor::new(&mut table);
        let res = insert(curr,"insert 1 alice");
        assert!(matches!(
            res,
            Err(InsertError::NotEnoughArgs {
                got: 2,
                expected: 3
            })
        ));
    }

    #[test]
    fn insert_returns_id_on_success() {
        let test_database_name = String::from("test_db");
        let mut table = Table::db_open(&test_database_name).unwrap();
        let mut cur = Cursor::new(&mut table);

        let res = insert(&mut cur, "insert 42 alice alice");
        cur.table.db_close().unwrap();

        assert_eq!(res, Ok(42));

        let output = select(&mut cur, "select 42").unwrap();
        assert_eq!(output, 42i32);

        drop(cur);
        fs::remove_file("test_db").unwrap();
    }
}
