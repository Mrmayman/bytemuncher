use super::Primitive;
use crate::End;

#[cfg(feature = "futures")]
use futures::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(any(feature = "tokio", feature = "futures"))]
use super::AsyncPrimitive;

macro_rules! impl_small_int {
    ($type:ty) => {
        impl Primitive for $type {
            // Ignoring endianness here as it doesn't matter (single byte)
            fn read_endian(reader: &mut impl std::io::Read, _: End) -> Result<Self, std::io::Error>
            where
                Self: Sized,
            {
                let mut buf = [0];
                reader.read_exact(&mut buf)?;
                #[allow(clippy::cast_possible_wrap)]
                Ok(buf[0] as Self)
            }

            fn write_endian(
                self,
                writer: &mut impl std::io::Write,
                _: End,
            ) -> Result<(), std::io::Error> {
                #[allow(clippy::cast_possible_wrap)]
                writer.write(&[self as u8])?;
                Ok(())
            }

            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            fn into_usize(self) -> usize {
                self as usize
            }

            fn from_usize(n: usize) -> Self {
                n as Self
            }
        }

        #[cfg(any(feature = "tokio", feature = "futures"))]
        impl AsyncPrimitive for $type {
            async fn read_endian_a(
                reader: &mut (impl AsyncReadExt + Unpin),
                _: End,
            ) -> Result<Self, std::io::Error>
            where
                Self: Sized,
            {
                let mut buf = [0];
                reader.read_exact(&mut buf).await?;
                #[allow(clippy::cast_possible_wrap)]
                Ok(buf[0] as Self)
            }

            async fn write_endian_a(
                self,
                writer: &mut (impl AsyncWriteExt + Unpin),
                _: End,
            ) -> Result<(), std::io::Error> {
                #[allow(clippy::cast_possible_wrap)]
                writer.write(&[self as u8]).await?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_int {
    ($type:ty) => {
        impl Primitive for $type {
            fn read_endian(
                reader: &mut impl std::io::Read,
                end: crate::End,
            ) -> Result<Self, std::io::Error>
            where
                Self: Sized,
            {
                let mut buf = [0u8; std::mem::size_of::<Self>()];
                reader.read_exact(&mut buf)?;
                Ok(if end.is_le() {
                    Self::from_le_bytes(buf)
                } else {
                    Self::from_be_bytes(buf)
                })
            }

            fn write_endian(
                self,
                writer: &mut impl std::io::Write,
                end: crate::End,
            ) -> Result<(), std::io::Error> {
                let bytes = if end.is_le() {
                    self.to_le_bytes()
                } else {
                    self.to_be_bytes()
                };
                writer.write_all(&bytes)?;
                Ok(())
            }

            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            fn into_usize(self) -> usize {
                self as usize
            }

            fn from_usize(n: usize) -> Self {
                n as Self
            }
        }

        #[cfg(any(feature = "tokio", feature = "futures"))]
        impl AsyncPrimitive for $type {
            async fn read_endian_a(
                reader: &mut (impl AsyncReadExt + Unpin),
                end: End,
            ) -> Result<Self, std::io::Error>
            where
                Self: Sized,
            {
                let mut buf = [0u8; std::mem::size_of::<Self>()];
                reader.read_exact(&mut buf).await?;
                Ok(if end.is_le() {
                    Self::from_le_bytes(buf)
                } else {
                    Self::from_be_bytes(buf)
                })
            }

            async fn write_endian_a(
                self,
                reader: &mut (impl AsyncWriteExt + Unpin),
                end: End,
            ) -> Result<(), std::io::Error>
            where
                Self: Sized,
            {
                let bytes = if end.is_le() {
                    self.to_le_bytes()
                } else {
                    self.to_be_bytes()
                };
                reader.write_all(&bytes).await?;
                Ok(())
            }
        }
    };
}

impl_small_int!(u8);
impl_small_int!(i8);

impl_int!(u16);
impl_int!(i16);

impl_int!(u32);
impl_int!(i32);

impl_int!(u64);
impl_int!(i64);

impl_int!(u128);
impl_int!(i128);

// Not implemented, because if you're parsing binary formats
// you better know the type size beforehand!

// impl_int!(usize);
// impl_int!(isize);
