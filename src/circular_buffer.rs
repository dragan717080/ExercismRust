fn main() {
    let mut buffer = CircularBuffer::new(3);
    assert!(buffer.write(1).is_ok());
    assert!(buffer.write(2).is_ok());
    assert_eq!(buffer.read(), Ok(1));
    assert!(buffer.write(3).is_ok());
    assert_eq!(buffer.read(), Ok(2));
    assert_eq!(buffer.read(), Ok(3));
}

pub struct CircularBuffer<T> {
    n: usize,
    elements: Vec<T>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
{
    pub fn new(capacity: usize) -> Self {
        Self { n: capacity, elements: Vec::with_capacity(capacity) }
    }

    fn is_full(&self) -> bool {
        self.elements.len() == self.n
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }

        Ok(self.elements.push(_element))
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.elements.is_empty() {
            return Err(Error::EmptyBuffer);
        }

        Ok(self.elements.remove(0))
    }

    pub fn clear(&mut self) {
        if self.elements.is_empty() {
            return;
        }

        self.elements = Vec::new();

        self.n = 1;
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.is_full() {
            self.elements.remove(0);
        }

        self.elements.push(_element);
    }
}
