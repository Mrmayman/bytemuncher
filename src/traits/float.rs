use super::Primitive;
use crate::End;

#[cfg(feature = "futures")]
use futures::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(any(feature = "tokio", feature = "futures"))]
use super::AsyncPrimitive;

macro_rules! impl_float {
    ($type:ty, $int:ty) => {
        impl Primitive for $type {
            fn read_endian(
                reader: &mut impl std::io::Read,
                end: End,
            ) -> Result<Self, std::io::Error> {
                let mut buf = [0u8; std::mem::size_of::<Self>()];
                reader.read_exact(&mut buf)?;
                let bits = if end.is_le() {
                    <$int>::from_le_bytes(buf)
                } else {
                    <$int>::from_be_bytes(buf)
                };
                Ok(Self::from_bits(bits))
            }

            fn write_endian(
                self,
                writer: &mut impl std::io::Write,
                end: End,
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
                let bits = if end.is_le() {
                    <$int>::from_le_bytes(buf)
                } else {
                    <$int>::from_be_bytes(buf)
                };
                Ok(Self::from_bits(bits))
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

impl_float!(f32, u32);
impl_float!(f64, u64);

// Not stable yet:
// impl_float!(f16, u16);
// impl_float!(f128, u128);
