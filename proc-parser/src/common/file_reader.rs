use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub struct ProcFileReader<const BUFFER_ARRAY_SIZE: usize, T> {
    proc_file: File,                     // file read once and read many times
    pub buffer: [u8; BUFFER_ARRAY_SIZE], // stack allocated buffer, to read contents of the file
    pub buffer_len: usize,               // length of the buffer we read every time into
    pub buffer_counter: usize,           // for keeping track of buffer read for each parsing
    pub is_cachable: bool,               // if its cachable, we dont read the file every time
    pub parsed_values: Option<T>,
}

impl<const BUFFER_ARRAY_SIZE: usize, T> ProcFileReader<BUFFER_ARRAY_SIZE, T> {
    pub fn new(path: &str, is_cachable: bool) -> Result<Self, Box<dyn Error>> {
        let proc_file = File::open(path)?;
        Ok(Self {
            proc_file,
            buffer: [0u8; BUFFER_ARRAY_SIZE],
            buffer_len: 0,
            buffer_counter: 0,
            is_cachable,
            parsed_values: None,
        })
    }

    pub fn read(&mut self) {
        // here we dont need to clear the buffer since its
        // an array and it will overwrite dont worry on that
        self.proc_file.seek(SeekFrom::Start(0)).unwrap();
        self.buffer_len = self.proc_file.read(&mut self.buffer).unwrap();
    }

    pub fn parse_bytes(&mut self) -> String {
        // let result: String = String::from_utf8(self.buffer.clone()).unwrap_or_default();
        let result = String::from_utf8(self.buffer.to_vec()).unwrap_or_default();
        result
    }
}
