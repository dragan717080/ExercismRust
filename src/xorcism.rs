fn main() {
    let mut xs1 = Xorcism::new(&[1, 2, 3, 4, 5]);
    let mut xs2 = Xorcism::new(&[1, 2, 3, 4, 5]);
    let input = "This is super-secret, cutting edge encryption, folks.".as_bytes();
    let mut cipher = input.to_owned();
    xs1.munge_in_place(&mut cipher);
    assert_ne!(&input, &cipher);
    let mut output = cipher;
    xs2.munge_in_place(&mut output);
    assert_eq!(&input, &output);
}

use std::{
    borrow::Borrow,
    io::{Read, Write},
    iter::Cycle,
    slice::Iter,
};

#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Self {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .zip(self.key.by_ref())
            .for_each(|(b, k)| *b ^= k);
    }

    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + Captures<'a> + 'b
    where
        Data: IntoIterator,
        Data::IntoIter: 'b,
        Data::Item: Borrow<u8>,
    {
        data.into_iter()
            .zip(self.key.by_ref())
            .map(|(b, k)| b.borrow() ^ k)
    }

    pub fn reader(self, inner: impl Read + 'a) -> impl Read + 'a {
        XorcismReader {
            munger: self,
            inner,
        }
    }

    pub fn writer(self, inner: impl Write + 'a) -> impl Write + 'a {
        XorcismWriter {
            munger: self,
            inner,
        }
    }
}

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

struct XorcismReader<'a, R: Read> {
    munger: Xorcism<'a>,
    inner: R,
}

impl<'a, R: Read> Read for XorcismReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf).map(|b| {
            self.munger.munge_in_place(buf);
            b
        })
    }
}

struct XorcismWriter<'a, W: Write> {
    munger: Xorcism<'a>,
    inner: W,
}

impl<'a, W: Write> Write for XorcismWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner
            .write(&self.munger.munge(buf).collect::<Vec<u8>>())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
