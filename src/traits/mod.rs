use std::io::{Read, Write};

use crate::{End, Muncher};

mod float;
mod int;

/// This trait allows you to specify your own primitive type
/// that can be read through bytemuncher. It's implemented
/// for all integer (`i*` and `u*`) and floating point (`f*`)
/// types.
///
/// Note: If your type is a single byte you can ignore the `end` field
pub trait Primitive {
    fn read_endian(reader: &mut impl Read, end: End) -> Result<Self, std::io::Error>
    where
        Self: Sized;

    fn write_endian(self, reader: &mut impl Write, end: End) -> Result<(), std::io::Error>
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
    /// Reads any [`crate::Primitive`] type (such as integers or floats),
    /// with the endianness specified in the `end` argument.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_m<E: Primitive>(&mut self, end: End) -> Result<E, std::io::Error> {
        E::read_endian(&mut self.inner, end)
    }

    /// Reads any [`crate::Primitive`] type (such as integers or floats),
    /// as little endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_le<E: Primitive>(&mut self) -> Result<E, std::io::Error> {
        self.read_m(End::Little)
    }

    /// Reads any [`crate::Primitive`] type (such as integers or floats),
    /// as big endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_be<E: Primitive>(&mut self) -> Result<E, std::io::Error> {
        self.read_m(End::Big)
    }

    /// Reads any [`crate::Primitive`] type (such as integers or floats),
    /// as native endian (as per the target platform).
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_ne<E: Primitive>(&mut self) -> Result<E, std::io::Error> {
        self.read_m(End::Native)
    }
}

impl<T: Write> Muncher<T> {
    /// Writes any [`crate::Primitive`] type (such as integers or floats),
    /// with the endianness specified in the `end` argument.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_m<E: Primitive>(&mut self, value: E, end: End) -> Result<(), std::io::Error> {
        value.write_endian(&mut self.inner, end)
    }

    /// Writes any [`crate::Primitive`] type (such as integers or floats),
    /// as native endian (as per the target platform).
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_ne<E: Primitive>(&mut self, value: E) -> Result<(), std::io::Error> {
        self.write_m(value, End::Native)
    }

    /// Writes any [`crate::Primitive`] type (such as integers or floats),
    /// as big endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_be<E: Primitive>(&mut self, value: E) -> Result<(), std::io::Error> {
        self.write_m(value, End::Big)
    }

    /// Writes any [`crate::Primitive`] type (such as integers or floats),
    /// as little endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_le<E: Primitive>(&mut self, value: E) -> Result<(), std::io::Error> {
        self.write_m(value, End::Little)
    }
}
