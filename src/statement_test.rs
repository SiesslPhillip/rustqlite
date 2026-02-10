#[cfg(test)]
mod tests {
    use crate::statement::{InsertError, insert, select};

    #[test]
    fn insert_returns_error_if_not_enough_args() {
        let res = insert("insert 1 alice");
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
        let res = insert("insert 42 alice alice@example.com");
        assert_eq!(res, Ok(42));
        let output = select("select 42").unwrap();
        assert_eq!(output, 42i32);
    }
}
