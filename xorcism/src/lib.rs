use std::io::{Read, Write};
use std::iter::Cycle;
use std::borrow::Borrow;
use std::slice::Iter;

pub trait HasLifetime<'a> {}
impl<'a, T> HasLifetime<'a> for T {}

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a>(Cycle<Iter<'a, u8>>);

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Self {
        Self(key.as_ref().iter().cycle())
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for byte in data {
            *byte ^= self.0.next().unwrap();
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, T, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + HasLifetime<'a> + 'b
    where
        T: Borrow<u8>,
        Data: IntoIterator<Item = T>,
        Data::IntoIter: 'b,
        'a: 'b,
    {
        data
            .into_iter()
            .map(move |byte| byte.borrow() ^ self.0.next().unwrap())
    }

    pub fn reader<R: Read>(self, reader: R) -> XorReader<'a, R> {
        XorReader::new(reader, self)
    }
    
    pub fn writer<W: Write>(self, writer: W) -> XorWriter<'a, W> {
        XorWriter::new(writer, self)
    }
}

pub struct XorReader<'a, R> {
    reader: R,
    munger: Xorcism<'a>
}

impl<'a, R: Read> XorReader<'a, R> {
    pub fn new(reader: R, munger: Xorcism<'a>) -> Self {
        Self {
            reader,
            munger
        }
    }
}

impl<'a, R: Read> Read for XorReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader
            .read(buf)
            .map(|read| {
                self.munger.munge_in_place(buf);
                read
            })
    }
}

pub struct XorWriter<'a, W> {
    writer: W,
    munger: Xorcism<'a>
}

impl<'a, W: Write> XorWriter<'a, W> {
    pub fn new(writer: W, munger: Xorcism<'a>) -> Self {
        Self {
            writer,
            munger
        }
    }
}

impl<'a, W: Write> Write for XorWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let munged_buf: Vec<_> = self.munger
            .munge(buf)
            .collect();

        self.writer.write(&munged_buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}