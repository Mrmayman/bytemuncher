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

#[cfg(feature = "futures")]
use futures::io::{AsyncRead, AsyncWrite};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncWrite};

/// Async equivalent of [`Primitive`] trait (see that for more info).
#[cfg(any(feature = "tokio", feature = "futures"))]
pub trait AsyncPrimitive: Primitive {
    fn read_endian_a(
        reader: &mut (impl AsyncRead + Unpin),
        end: End,
    ) -> impl Future<Output = std::io::Result<Self>>
    where
        Self: Sized;

    fn write_endian_a(
        self,
        reader: &mut (impl AsyncWrite + Unpin),
        end: End,
    ) -> impl Future<Output = std::io::Result<()>>
    where
        Self: Sized;
}

impl<T: std::io::Read> Muncher<T> {
    /// Reads any [`crate::Primitive`] type (such as integers or floats),
    /// with the endianness specified in the `end` argument.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_m<E: Primitive>(&mut self, end: End) -> std::io::Result<E> {
        E::read_endian(&mut self.inner, end)
    }

    /// Reads any [`crate::Primitive`] type (such as integers or floats),
    /// as little endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_le<E: Primitive>(&mut self) -> std::io::Result<E> {
        self.read_m(End::Little)
    }

    /// Reads any [`crate::Primitive`] type (such as integers or floats),
    /// as big endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_be<E: Primitive>(&mut self) -> std::io::Result<E> {
        self.read_m(End::Big)
    }

    /// Reads any [`crate::Primitive`] type (such as integers or floats),
    /// as native endian (as per the target platform).
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_ne<E: Primitive>(&mut self) -> std::io::Result<E> {
        self.read_m(End::Native)
    }
}

impl<T: Write> Muncher<T> {
    /// Writes any [`crate::Primitive`] type (such as integers or floats),
    /// with the endianness specified in the `end` argument.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_m<E: Primitive>(&mut self, value: E, end: End) -> std::io::Result<()> {
        value.write_endian(&mut self.inner, end)
    }

    /// Writes any [`crate::Primitive`] type (such as integers or floats),
    /// as native endian (as per the target platform).
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_ne<E: Primitive>(&mut self, value: E) -> std::io::Result<()> {
        self.write_m(value, End::Native)
    }

    /// Writes any [`crate::Primitive`] type (such as integers or floats),
    /// as big endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_be<E: Primitive>(&mut self, value: E) -> std::io::Result<()> {
        self.write_m(value, End::Big)
    }

    /// Writes any [`crate::Primitive`] type (such as integers or floats),
    /// as little endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_le<E: Primitive>(&mut self, value: E) -> std::io::Result<()> {
        self.write_m(value, End::Little)
    }
}

#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncRead + Unpin> Muncher<T> {
    /// Reads any [`crate::AsyncPrimitive`] type (such as integers or floats),
    /// with the endianness specified in the `end` argument.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn read_m_a<E: AsyncPrimitive>(&mut self, end: End) -> std::io::Result<E> {
        E::read_endian_a(&mut self.inner, end).await
    }

    /// Reads any [`crate::AsyncPrimitive`] type (such as integers or floats),
    /// as little endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn read_le_a<E: AsyncPrimitive>(&mut self) -> std::io::Result<E> {
        self.read_m_a(End::Little).await
    }

    /// Reads any [`crate::AsyncPrimitive`] type (such as integers or floats),
    /// as big endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn read_be_a<E: AsyncPrimitive>(&mut self) -> std::io::Result<E> {
        self.read_m_a(End::Big).await
    }

    /// Reads any [`crate::AsyncPrimitive`] type (such as integers or floats),
    /// as native endian (as per the target platform).
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn read_ne_a<E: AsyncPrimitive>(&mut self) -> std::io::Result<E> {
        self.read_m_a(End::Native).await
    }
}

#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncWrite + Unpin> Muncher<T> {
    /// Writes any [`crate::AsyncPrimitive`] type (such as integers or floats),
    /// with the endianness specified in the `end` argument.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn write_m_a<E: AsyncPrimitive>(
        &mut self,
        value: E,
        end: End,
    ) -> std::io::Result<()> {
        value.write_endian_a(&mut self.inner, end).await
    }

    /// Writes any [`crate::AsyncPrimitive`] type (such as integers or floats),
    /// as native endian (as per the target platform).
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn write_ne_a<E: AsyncPrimitive>(&mut self, value: E) -> std::io::Result<()> {
        self.write_m_a(value, End::Native).await
    }

    /// Writes any [`crate::AsyncPrimitive`] type (such as integers or floats),
    /// as big endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn write_be_a<E: AsyncPrimitive>(&mut self, value: E) -> std::io::Result<()> {
        self.write_m_a(value, End::Big).await
    }

    /// Writes any [`crate::AsyncPrimitive`] type (such as integers or floats),
    /// as little endian.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn write_le_a<E: AsyncPrimitive>(&mut self, value: E) -> std::io::Result<()> {
        self.write_m_a(value, End::Little).await
    }
}
