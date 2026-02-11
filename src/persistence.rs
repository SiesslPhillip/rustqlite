use std::io::{Error, Seek, SeekFrom, Write};
use std::fs::File;
use std::io;
use log::error;
use crate::statement::SelectError;
use crate::table::{Page, PAGE_SIZE, TABLE_MAX_PAGES};

pub struct Pager {
    pub(crate) file: File,
    pub(crate) content_length: usize,
    pub(crate) pages: [Option<Page>; TABLE_MAX_PAGES],
}

impl Pager {
    pub fn flush(&mut self, page_num: usize, size: usize) -> io::Result<()> {
        if page_num >= TABLE_MAX_PAGES {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "page out of bounds"));
        }

        let page = self.pages[page_num]
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "tried to flush null page"))?;

        if size > PAGE_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "size > PAGE_SIZE"));
        }

        let offset = (page_num * PAGE_SIZE) as u64;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(&page[..size])?;
        self.file.flush()?; // optional; for durability use sync_data/sync_all
        Ok(())
    }
}