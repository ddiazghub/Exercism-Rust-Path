use std::io::{Read, Result, Write};

pub type ReadStats<T> = IoStats<T>;
pub type WriteStats<T> = IoStats<T>;

pub struct IoStats<T> {
    bytes: usize,
    operations: usize,
    stream: T
}

impl<T> IoStats<T> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: T) -> IoStats<T> {
        Self {
            bytes: 0,
            operations: 0,
            stream: _wrapped
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.stream
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn operations(&self) -> usize {
        self.operations
    }
}

impl<R: Read> IoStats<R> {
    pub fn reads(&self) -> usize {
        self.operations
    }
}

impl<W: Write> IoStats<W> {
    pub fn writes(&self) -> usize {
        self.operations
    }
}

impl<R: Read> Read for IoStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.stream.read(buf)?;
        self.bytes += bytes;
        self.operations += 1;

        Ok(bytes)
    }
}

impl<W: Write> Write for IoStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.stream.write(buf)?;
        self.bytes += bytes;
        self.operations += 1;

        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }
}
