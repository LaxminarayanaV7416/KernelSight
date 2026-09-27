use super::parser_utils::{parse_u64_swar, search_char, search_start_end_chars};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub trait Parsers {
    fn parse_paranthesis(
        &mut self,
        start: usize,
    ) -> Result<(usize, usize), Box<dyn std::error::Error>>;
    fn parse_newlines(&mut self, newline_array: &mut [usize]);
    fn parse_spaces(&mut self, space_array: &mut [usize]);
    fn parse_tabs(&mut self, space_array: &mut [usize]);
    fn parse_squre_paranthesis(
        &mut self,
        start: usize,
    ) -> Result<(usize, usize), Box<dyn std::error::Error>>;
    fn parse_number(&mut self, start: usize, end: usize) -> u64;
    fn parse_string(&mut self, start: usize, end: usize) -> String;
}

pub trait LinearParser {
    fn parse(&mut self, 
        newline_array: &mut [i16],
        space_array: &mut [i16],
        paranthesis_array: &mut [i16],
        square_paranthesis_array: &mut [i16],
        colon_array: &mut [i16],
        tab_array: &mut [i16],
    );
}

pub struct ProcFileReader<const BUFFER_ARRAY_SIZE: usize> {
    proc_file: File,
    pub buffer: [u8; BUFFER_ARRAY_SIZE],
    pub buffer_len: usize,
}

impl<const BUFFER_ARRAY_SIZE: usize> ProcFileReader<BUFFER_ARRAY_SIZE> {
    pub fn new(path: &str) -> Option<Self> {
        let proc_file = File::open(path);
        match proc_file {
            Ok(proc_file) => Some(Self {
                proc_file,
                buffer: [0u8; BUFFER_ARRAY_SIZE],
                buffer_len: 0,
            }),
            Err(_) => {
                println!("Failed to open file: {}", path);
                None
            }
        }
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

impl<const BUFFER_ARRAY_SIZE: usize> Parsers for ProcFileReader<BUFFER_ARRAY_SIZE> {
    fn parse_paranthesis(
        &mut self,
        start: usize,
    ) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        search_start_end_chars(&self.buffer[..self.buffer_len], start, b'(', b')')
    }

    fn parse_newlines(&mut self, newline_array: &mut [usize]) {
        search_char(&self.buffer[..self.buffer_len], newline_array, b'\n');
    }

    fn parse_spaces(&mut self, space_array: &mut [usize]) {
        search_char(&self.buffer[..self.buffer_len], space_array, b' ');
    }

    fn parse_tabs(&mut self, space_array: &mut [usize]) {
        search_char(&self.buffer[..self.buffer_len], space_array, b'\t');
    }

    fn parse_squre_paranthesis(
        &mut self,
        start: usize,
    ) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        search_start_end_chars(&self.buffer[..self.buffer_len], start, b'[', b']')
    }

    fn parse_number(&mut self, start: usize, end: usize) -> u64 {
        parse_u64_swar(&self.buffer[..self.buffer_len][start..end]).unwrap_or(0)
    }

    fn parse_string(&mut self, start: usize, end: usize) -> String {
        let result = String::from_utf8(self.buffer[..self.buffer_len][start..end].to_vec())
            .unwrap_or_default();
        result
    }
}
