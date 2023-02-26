use std::mem;

pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    cursor: usize,
    start: usize,
    len: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: (0..capacity).map(|_| None).collect(),
            cursor: 0,
            start: 0,
            len: 0,
            capacity
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.len == self.capacity {
            Err(Error::FullBuffer)
        } else {
            self.overwrite(element);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            Err(Error::EmptyBuffer)
        } else {
            let data = mem::replace(&mut self.data[self.start], None).unwrap();
            self.len -= 1;
            self.start += 1;

            if self.start == self.capacity {
                self.start = 0;
            }

            Ok(data)
        }
    }

    pub fn clear(&mut self) {
        while let Ok(_) = self.read() {}
    }

    pub fn overwrite(&mut self, element: T) {
        if self.data[self.cursor].is_some() {
            self.start += 1;
        }

        self.data[self.cursor] = Some(element);
        self.len += 1;
        self.cursor += 1;

        if self.cursor == self.capacity {
            self.cursor = 0;
        }
    }
}
