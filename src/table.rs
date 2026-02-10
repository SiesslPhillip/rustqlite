use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone)]
pub(crate) struct Row {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) email: String,
}

#[derive(Debug, Default)]
pub(crate) struct Table {
    pub(crate) rows: Vec<Row>,
}

static TABLE: OnceLock<RwLock<Table>> = OnceLock::new();

fn table() -> &'static RwLock<Table> {
    TABLE.get_or_init(|| RwLock::new(Table::default()))
}

pub(crate) fn insert_row(id: i64, name: impl Into<String>, email: impl Into<String>) {
    let mut t = table().write().unwrap();
    t.rows.push(Row {
        id,
        name: name.into(),
        email: email.into(),
    });
}

pub(crate) fn get_row_by_id(id: i64) -> Option<Row> {
    let t = table().read().unwrap();
    t.rows.iter().find(|r| r.id == id).cloned()
}
