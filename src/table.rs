use std::error::Error;
use std::io;
use std::io::Read;
use std::sync::{LazyLock, Mutex};

pub const USERNAME_LEN: usize = 32;
pub const EMAIL_LEN: usize = 255;

pub const ID_SIZE: usize = size_of::<i32>();
pub const USERNAME_SIZE: usize = USERNAME_LEN;
pub const EMAIL_SIZE: usize = EMAIL_LEN;

pub const ID_OFFSET: usize = 0;
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: usize = EMAIL_OFFSET + EMAIL_SIZE;

pub const PAGE_SIZE: usize = 4096;
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_PAGES: usize = 100;
pub const TABLE_MAX_ROWS: usize = TABLE_MAX_PAGES * ROWS_PER_PAGE;

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct Row {
    pub(crate) id: i32,
    pub(crate) name: [u8; USERNAME_LEN],
    pub(crate) email: [u8; EMAIL_LEN],
}

static TABLE: LazyLock<Mutex<Table>> = LazyLock::new(|| Mutex::new(Table::new()));

pub type Page = Box<[u8; PAGE_SIZE]>;

pub struct Table {
    pub num_rows: u32,
    pub pages: [Option<Page>; TABLE_MAX_PAGES],
}

impl Table {
    pub fn new() -> Self {
        Self {
            num_rows: 0,
            pages: std::array::from_fn(|_| None),
        }
    }

    pub fn get_page_mut(&mut self, page_num: usize) -> &mut [u8; PAGE_SIZE] {
        assert!(page_num < TABLE_MAX_PAGES);
        self.pages[page_num]
            .get_or_insert_with(|| Box::new([0u8; PAGE_SIZE]))
            .as_mut()
    }
}

pub fn insert_row(id: i32, name: &str, email: &str) {
    let calculated_offset = calc_offset(id);
    let page_num = calculated_offset.1;
    let byte_offset = calculated_offset.0;

    let mut table = TABLE.lock().unwrap();
    let page: &mut [u8; PAGE_SIZE] = table.get_page_mut(page_num);

    page[byte_offset + ID_OFFSET..byte_offset + ID_OFFSET + ID_SIZE]
        .copy_from_slice(&id.to_le_bytes());

    let name_byte = to_fixed_32_truncate(name);
    page[byte_offset + USERNAME_OFFSET..byte_offset + USERNAME_OFFSET + USERNAME_SIZE]
        .copy_from_slice(&name_byte);

    let email_byte = to_fixed_255_truncate(email);
    page[byte_offset + EMAIL_OFFSET..byte_offset + EMAIL_OFFSET + EMAIL_SIZE]
        .copy_from_slice(&email_byte);
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

fn calc_offset(id: i32) -> (usize, usize) {
    let page_num: usize = id as usize / ROWS_PER_PAGE;
    let row_offset: usize = id as usize % ROWS_PER_PAGE;
    let byte_offset: usize = row_offset * ROW_SIZE;
    (byte_offset, page_num)
}

pub fn fetch_row(id: i32) -> Result<Row, std::io::Error> {
    let calculated_offset = calc_offset(id);
    let page_num = calculated_offset.1;
    let byte_offset = calculated_offset.0;
    let mut table = TABLE.lock().unwrap();

    let page: &mut [u8; PAGE_SIZE] = table.get_page_mut(page_num);

    let name_byte: &[u8] =
        &page[byte_offset + USERNAME_OFFSET..byte_offset + USERNAME_OFFSET + USERNAME_SIZE];
    let email_byte: &[u8] =
        &page[byte_offset + EMAIL_OFFSET..byte_offset + EMAIL_OFFSET + EMAIL_SIZE];

    Ok(Row {
        id,
        name: slice_to_32(name_byte).unwrap(),
        email: slice_to_255(email_byte).unwrap(),
    })
}
fn slice_to_32(b: &[u8]) -> Result<[u8; 32], &'static str> {
    b.try_into().map_err(|_| "expected 32 bytes")
}

fn slice_to_255(b: &[u8]) -> Result<[u8; 255], &'static str> {
    b.try_into().map_err(|_| "expected 255 bytes")
}
