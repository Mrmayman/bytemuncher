use std::io::{BufRead, Read};

use crate::Muncher;

impl<T: Read> std::io::Read for Muncher<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<T: BufRead> std::io::BufRead for Muncher<T> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amount: usize) {
        self.inner.consume(amount);
    }
}
