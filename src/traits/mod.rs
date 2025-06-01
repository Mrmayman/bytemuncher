use std::io::Read;

use crate::{End, Muncher};

mod float;
mod int;

/// This trait allows you to specify your own primitive type
/// that can be read through bytemuncher. It's implemented
/// for all integer (`i*` and `u*`) and floating point (`f*`)
/// types.
pub trait ReadEndian {
    /// If your type is a single byte you can ignore the `end` field
    /// of the `read_endian` function.
    fn read_endian(reader: &mut impl Read, end: End) -> Result<Self, std::io::Error>
    where
        Self: Sized;

    /// Try to "semantically" convert your type to usize.
    /// Any overflows, truncations and stuff can be ignored.
    ///
    /// If your type isn't really a number
    /// feel free to return `0`.
    ///
    /// For an idea of how to use this, this will
    /// primarily be used for array length.
    fn into_usize(self) -> usize;
}

impl<T: std::io::Read> Muncher<T> {
    /// Reads any [`crate::ReadEndian`] type (such as integers or floats),
    /// with the endianness specified in the `end` argument.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_m<E: ReadEndian>(&mut self, end: End) -> Result<E, std::io::Error> {
        E::read_endian(&mut self.reader, end)
    }

    /// Reads any [`crate::ReadEndian`] type (such as integers or floats),
    /// as little endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_le<E: ReadEndian>(&mut self) -> Result<E, std::io::Error> {
        self.read_m(End::Little)
    }

    /// Reads any [`crate::ReadEndian`] type (such as integers or floats),
    /// as big endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_be<E: ReadEndian>(&mut self) -> Result<E, std::io::Error> {
        self.read_m(End::Big)
    }

    /// Reads any [`crate::ReadEndian`] type (such as integers or floats),
    /// as native endian (as per the target platform).
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_ne<E: ReadEndian>(&mut self) -> Result<E, std::io::Error> {
        self.read_m(End::Native)
    }
}
