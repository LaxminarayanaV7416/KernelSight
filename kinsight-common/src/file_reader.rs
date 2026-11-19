use super::parser_error::{ProcParserError, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

// TODO:
// Need to handle the below error:
// thread 'main' (1251313) panicked at proc-parser/src/common/file_reader.rs:33:65:
// called `Result::unwrap()` on an `Err` value: Os { code: 3, kind: Uncategorized, message: "No such process" }
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

#[derive(Debug)]
pub struct ProcFileReader<const BUFFER_ARRAY_SIZE: usize> {
    proc_file: File,                     // file read once and read many times
    pub buffer: [u8; BUFFER_ARRAY_SIZE], // stack allocated buffer, to read contents of the file
    pub buffer_len: usize,               // length of the buffer we read every time into
}

impl<const BUFFER_ARRAY_SIZE: usize> ProcFileReader<BUFFER_ARRAY_SIZE> {
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
            Ok(len) => {
                self.buffer_len = len;
                true
            }
            Err(_) => false,
        }
    }

    pub fn parse_bytes_to_string(&mut self) -> String {
        // let result: String = String::from_utf8(self.buffer.clone()).unwrap_or_default();
        let result = String::from_utf8(self.buffer.to_vec()).unwrap_or_default();
        result
    }
}
