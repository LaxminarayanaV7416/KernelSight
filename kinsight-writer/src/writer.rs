use kinsight_common::{ProcParserError, Result};
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

/// Selects the output destination.
#[derive(Debug, Clone)]
pub enum WriterType {
    Tui,
    ConsoleLog,
    WriteToFile(PathBuf),
}

/// Common interface implemented by each concrete writer backend.
pub trait KinsightWriter<T>
where
    T: Debug,
{
    fn write(&mut self, data: &T) -> Result<()>;
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Placeholder TUI writer.
#[derive(Debug, Default)]
struct TuiWriter;

impl<T> KinsightWriter<T> for TuiWriter
where
    T: Debug,
{
    fn write(&mut self, data: &T) -> Result<()> {
        println!("{data:#?}");
        Ok(())
    }
}

/// Writes records to standard output.
#[derive(Debug, Default)]
struct ConsoleLogWriter;

impl<T> KinsightWriter<T> for ConsoleLogWriter
where
    T: Debug,
{
    fn write(&mut self, data: &T) -> Result<()> {
        println!("{data:#?}");
        Ok(())
    }
}

/// Writes records to a file.
struct WriteToFileWriter {
    writer: BufWriter<File>,
}

impl WriteToFileWriter {
    fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|error| {
                ProcParserError::unknown(std::io::Error::new(
                    error.kind(),
                    format!(
                        "failed to open writer output file {}: {error}",
                        path.display(),
                    ),
                ))
            })?;

        Ok(Self {
            writer: BufWriter::new(file),
        })
    }
}

impl<T> KinsightWriter<T> for WriteToFileWriter
where
    T: Debug,
{
    fn write(&mut self, data: &T) -> Result<()> {
        writeln!(self.writer, "{data:#?}").map_err(ProcParserError::unknown)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush().map_err(ProcParserError::unknown)
    }
}

/// Internal concrete backend.
enum WriterBackend {
    Tui(TuiWriter),
    Console(ConsoleLogWriter),
    File(WriteToFileWriter),
}

/// Generic output writer.
pub struct Writer {
    backend: WriterBackend,
}

impl Writer {
    pub fn new(writer_type: WriterType) -> Result<Self> {
        let backend = match writer_type {
            WriterType::Tui => WriterBackend::Tui(TuiWriter),
            WriterType::ConsoleLog => WriterBackend::Console(ConsoleLogWriter),
            WriterType::WriteToFile(path) => WriterBackend::File(WriteToFileWriter::new(path)?),
        };

        Ok(Self { backend })
    }

    /// Writes any debuggable value using the selected backend.
    pub fn write<T>(&mut self, data: &T) -> Result<()>
    where
        T: Debug,
    {
        match &mut self.backend {
            WriterBackend::Tui(writer) => writer.write(data),
            WriterBackend::Console(writer) => writer.write(data),
            WriterBackend::File(writer) => writer.write(data),
        }
    }

    /// Flushes buffered output.
    pub fn flush(&mut self) -> Result<()> {
        match &mut self.backend {
            WriterBackend::Tui(writer) => <TuiWriter as KinsightWriter<()>>::flush(writer),
            WriterBackend::Console(writer) => {
                <ConsoleLogWriter as KinsightWriter<()>>::flush(writer)
            }
            WriterBackend::File(writer) => <WriteToFileWriter as KinsightWriter<()>>::flush(writer),
        }
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        // Drop cannot return an error. Explicitly call `flush()` when the
        // caller needs to handle flush failures.
        if let WriterBackend::File(writer) = &mut self.backend {
            let _ = writer.writer.flush();
        }
    }
}
