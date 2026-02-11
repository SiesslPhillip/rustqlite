use crate::persistence::Pager;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::sync::{LazyLock, Mutex};
use crate::statement::SelectError;

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

pub static TABLE: LazyLock<Mutex<Table>> = LazyLock::new(|| Mutex::new(Table::db_open(String::from("database.db")).expect("Cant Create or Read Database!")));

pub type Page = [u8; PAGE_SIZE];

pub struct Table {
    pub num_rows: usize,
    pub pager: Pager,
}

impl Table {
    pub fn db_open(filename: String) -> io::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&filename)?;

        // lseek(fd, 0, SEEK_END)
        let file_length = file.seek(SeekFrom::End(0))?;

        Ok(Self {
            num_rows: file_length as usize/ROW_SIZE,
            pager: Pager {
                file,
                content_length: file_length as usize,
                pages: std::array::from_fn(|_| None),
            },
        })
    }

    pub fn db_close(&mut self) -> io::Result<()> {
        let num_full_pages = self.pager.content_length / PAGE_SIZE;
        for i in 0..num_full_pages {
            if self.pager.pages[i].is_some() {
                self.pager.flush(i, PAGE_SIZE)?;
            }
        }

        let remainder = self.pager.content_length % PAGE_SIZE;
        if remainder != 0 {
            let last = num_full_pages;
            if self.pager.pages[last].is_some() {
                self.pager.flush(last, remainder)?;
            }
        }

        Ok(())
    }
    pub fn get_page_mut(&mut self, page_num: usize) -> &mut Page {
        self.get_page(page_num).expect("get_page failed")
    }

    pub fn get_page(&mut self, page_num: usize) -> Result<&mut Page, SelectError> {
        if page_num >= TABLE_MAX_PAGES {
            return Err(SelectError::OutOfBounds);
        }

        if self.pager.pages[page_num].is_none() {
            let mut page = [0u8; PAGE_SIZE];

            let mut num_pages = self.pager.content_length / PAGE_SIZE;
            if self.pager.content_length % PAGE_SIZE != 0 {
                num_pages += 1;
            }

            if page_num < num_pages {
                let offset = (page_num * PAGE_SIZE) as u64;
                self.pager.file.seek(SeekFrom::Start(offset)).unwrap();

                let bytes_to_read = if page_num == num_pages - 1 && (self.pager.content_length % PAGE_SIZE != 0) {
                    self.pager.content_length % PAGE_SIZE
                } else {
                    PAGE_SIZE
                };

                self.pager.file.read_exact(&mut page[..bytes_to_read]).unwrap();
            }

            self.pager.pages[page_num] = Some(page);
        }

        Ok(self.pager.pages[page_num].as_mut().unwrap())
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

    let end_of_row = page_num * PAGE_SIZE + byte_offset + ROW_SIZE;
    table.pager.content_length = table.pager.content_length.max(end_of_row);
    table.num_rows = table.num_rows.max(id as usize + 1);
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
