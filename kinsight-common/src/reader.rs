use super::errors::{ProcParserError, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug)]
pub struct KinsightReader<const BUFFER_ARRAY_SIZE: usize> {
    proc_file: File,                     // file read once and read many times
    pub buffer: [u8; BUFFER_ARRAY_SIZE], // stack allocated buffer, to read contents of the file
    pub buffer_len: usize,               // length of the buffer we read every time into
}

impl<const BUFFER_ARRAY_SIZE: usize> KinsightReader<BUFFER_ARRAY_SIZE> {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let proc_file = File::open(path)
            .map_err(|_| ProcParserError::FileReadError(path.to_string_lossy().into_owned()))?;
        Ok(Self {
            proc_file,
            buffer: [0u8; BUFFER_ARRAY_SIZE],
            buffer_len: 0,
        })
    }

    #[inline(always)]
    pub fn read(&mut self) -> bool {
        // here we dont need to clear the buffer since its
        // an array and it will overwrite dont worry on that
        self.proc_file.seek(SeekFrom::Start(0)).unwrap();
        match self.proc_file.read(&mut self.buffer) {
            // NOTE: The we dont need to map error here
            // idealogy is that we will return true/false
            // based on this we will raise the filedontexist
            // error such that it will be easier for us
            Ok(len) => {
                self.buffer_len = len;
                true
            }
            Err(_) => false,
        }
    }

    pub fn parse_bytes_to_string(&mut self) -> String {
        let result = String::from_utf8(self.buffer.to_vec()).unwrap_or_default();
        result
    }
}

pub trait KinsightDataParserTrait<T: Clone, FF> {
    fn values(&self) -> &T;
    fn prev_values(&self) -> &T;
    fn read_and_parse(&mut self, field_filter: &FF) -> bool;
    fn parse_buffer(&mut self, field_filter: &FF);
}
