use std::io::{BufRead, Read};

use crate::{End, Muncher, ReadEndian};

mod error;

pub use error::MutfError;

/// **Size-prefixed MUTF-8 string methods**
impl<T: Read> Muncher<T> {
    /// Reads a MUTF-8 string prefixed by a length (number of bytes) of type `<E>`,
    /// then converts it to a UTF-8 [`String`].
    /// This is a niche format. For most cases, this is
    /// not what you need and you should instead use UTF-8.
    ///
    /// If you want raw MUTF-8, use [`Muncher::read_pref_bytes`].
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    ///
    /// For more info on MUTF-8 see <https://crates.io/crates/mutf8>.
    pub fn read_pref_mutf8<E: ReadEndian>(&mut self, end: End) -> Result<String, MutfError> {
        let buf = self.read_pref_bytes::<E>(end)?;
        mutf2utf(&buf)
    }

    /// Reads `len` number of bytes into a MUTF-8 string,
    /// then converts it to a UTF-8 [`String`].
    /// This is a niche format. For most cases, this is
    /// not what you need and you should instead use UTF-8.
    ///
    /// If you want raw MUTF-8, use [`Muncher::read_fixed_bytes`].
    ///
    /// For more info on MUTF-8 see <https://crates.io/crates/mutf8>.
    pub fn read_fixed_mutf8<E: ReadEndian>(&mut self, len: usize) -> Result<String, MutfError> {
        let buf = self.read_fixed_bytes(len)?;
        mutf2utf(&buf)
    }
}

fn mutf2utf(buf: &[u8]) -> Result<String, MutfError> {
    Ok(String::from_utf8_lossy(&mutf8::mutf8_to_utf8(buf)?).to_string())
}

/// **MUTF-8 string and buffer related methods that require `T: `[`std::io::BufRead`]**
impl<T: BufRead> Muncher<T> {
    /// Reads a C-style string (ending with `\0` null byte)
    /// in the MUTF-8 format (tries to parse, fails if invalid).
    ///
    /// This is a niche format. For most cases, this is
    /// not what you need and you should instead use UTF-8.
    ///
    /// If you want bytes or some other format,
    /// see [`Muncher::read_cstr_bytes`].
    ///
    /// For more info on MUTF-8 see <https://crates.io/crates/mutf8>.
    pub fn read_cstr_mutf8(&mut self) -> Result<String, MutfError> {
        mutf2utf(&self.read_cstr_bytes()?)
    }
}
