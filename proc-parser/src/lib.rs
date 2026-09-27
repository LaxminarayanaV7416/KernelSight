pub mod file_reader;
mod parser_utils;

// pub type ProcFilePIDStat = file_reader::ProcFileReader<1024>;
pub type ProcFilePIDStat = file_reader::ProcFileReader<64>;
pub type ProcFilePIDStatm = file_reader::ProcFileReader<256>; // actually it can hold upto 140 256 choosen to fit the cache line size

pub use file_reader::Parsers;
