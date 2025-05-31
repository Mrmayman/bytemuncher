use std::io::Read;

use crate::{Endianness, Muncher};

mod float;
mod int;

pub trait ReadEndian {
    fn read_endian(reader: &mut impl Read, end: Endianness) -> Result<Self, std::io::Error>
    where
        Self: Sized;

    fn into_usize(self) -> usize;
}

impl<T: std::io::Read> Muncher<T> {
    /// Reads any [`crate::ReadEndian`] type (such as integers or floats),
    /// with the endianness specified in the `end` argument.
    ///
    /// For more info on endianness see [`crate::Endianness`].
    pub fn read_m<E: ReadEndian>(&mut self, end: Endianness) -> Result<E, std::io::Error> {
        E::read_endian(&mut self.reader, end)
    }

    /// Reads any [`crate::ReadEndian`] type (such as integers or floats),
    /// as little endian.
    ///
    /// For more info on endianness see [`crate::Endianness`].
    pub fn read_le<E: ReadEndian>(&mut self) -> Result<E, std::io::Error> {
        self.read_m(Endianness::Little)
    }

    /// Reads any [`crate::ReadEndian`] type (such as integers or floats),
    /// as big endian.
    ///
    /// For more info on endianness see [`crate::Endianness`].
    pub fn read_be<E: ReadEndian>(&mut self) -> Result<E, std::io::Error> {
        self.read_m(Endianness::Big)
    }

    /// Reads any [`crate::ReadEndian`] type (such as integers or floats),
    /// as native endian (as per the target platform).
    ///
    /// For more info on endianness see [`crate::Endianness`].
    pub fn read_ne<E: ReadEndian>(&mut self) -> Result<E, std::io::Error> {
        self.read_m(Endianness::Native)
    }
}
