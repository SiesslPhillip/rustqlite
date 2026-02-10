use std::sync::{OnceLock, RwLock};

pub const USERNAME_LEN: usize = 32;
pub const EMAIL_LEN: usize = 255;

pub const ID_SIZE: usize = size_of::<i32>();
pub const USERNAME_SIZE: usize = USERNAME_LEN;
pub const EMAIL_SIZE: usize = EMAIL_LEN;

pub const ID_OFFSET: usize = 0;
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: usize = EMAIL_OFFSET + EMAIL_SIZE;

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct Row {
    pub(crate) id: i32,
    pub(crate) name: [u8; USERNAME_LEN],
    pub(crate) email: [u8; EMAIL_LEN],
}

#[derive(Debug, Default)]
pub(crate) struct Table {
    pub(crate) rows: Vec<Row>,
}

static TABLE: OnceLock<RwLock<Table>> = OnceLock::new();

fn table() -> &'static RwLock<Table> {
    TABLE.get_or_init(|| RwLock::new(Table::default()))
}

pub(crate) fn insert_row(id: i32, name: &str, email: &str) {
    let mut t = table().write().unwrap();
    t.rows.push(Row {
        id,
        name: to_fixed_32_truncate(name),
        email: to_fixed_255_truncate(email),
    });
}

pub(crate) fn get_row_by_id(id: i32) -> Option<Row> {
    let t = table().read().unwrap();
    t.rows.iter().find(|r| r.id == id).cloned()
}

pub fn to_fixed_32_truncate(s: &str) -> [u8; 32] {
    let bytes = s.as_bytes();
    let mut out = [0u8; 32];
    let n = bytes.len().min(32);
    out[..n].copy_from_slice(&bytes[..n]);
    out
}

pub fn to_fixed_255_truncate(s: &str) -> [u8; 255] {
    let bytes = s.as_bytes();
    let mut out = [0u8; 255];
    let n = bytes.len().min(255);
    out[..n].copy_from_slice(&bytes[..n]);
    out
}