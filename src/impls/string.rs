use crate::{End, Muncher, ReadEndian};
use std::io::{BufRead, Error, ErrorKind, Read};

/// **Size-prefixed string methods**
impl<T: Read> Muncher<T> {
    /// Reads some bytes prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_pref_bytes<E: ReadEndian>(&mut self, end: End) -> Result<Vec<u8>, Error> {
        let len = self.read_m::<E>(end)?.into_usize();
        self.read_fixed_bytes(len)
    }

    /// Reads `len` number of bytes into a `Vec<u8>`
    pub fn read_fixed_bytes(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        self.verify_len(len)?;
        let mut buf = vec![0u8; len];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Reads a UTF-8 string prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_pref_utf8<E: ReadEndian>(&mut self, end: End) -> Result<String, Error> {
        bytes2utf8(self.read_pref_bytes::<E>(end)?)
    }

    /// Reads `len` number of bytes into a UTF-8 [`String`].
    pub fn read_fixed_utf8(&mut self, len: usize) -> Result<String, Error> {
        bytes2utf8(self.read_fixed_bytes(len)?)
    }

    /// Reads a UCS-2 string prefixed by a length (number of characters) in the type `<E>`.
    ///
    /// UCS-2 consists of big endian 16-bit words, each of which represent a Unicode
    /// code point between U+0000 and U+FFFF inclusive.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_pref_ucs2<E: ReadEndian>(&mut self, end: End) -> Result<String, Error> {
        let char_count = self.read_m::<E>(end)?.into_usize();
        self.read_fixed_ucs2(char_count)
    }

    /// Reads `char_count` number of 16-bit characters as a UCS-2 string,
    /// and converts it to UTF-8 [`String`].
    pub fn read_fixed_ucs2(&mut self, char_count: usize) -> Result<String, Error> {
        self.verify_len(char_count * 2)?;
        let mut result = String::with_capacity(char_count);

        for _ in 0..char_count {
            let ch = self.read_be::<u16>()?; // UCS-2 is always big-endian

            // Based on:
            // https://github.com/rust-osdev/ucs2-rs/blob/aa837529a4999e8c7eacb326fd153cc52792814b/src/lib.rs#L151
            match ch {
                0..128 => {
                    result.push(ch as u8 as char);
                }
                128..2048 => {
                    let first = 0b1100_0000 + ((ch >> 6) & 0b0001_1111) as u8;
                    let last = 0b1000_0000 + (ch & 0b0011_1111) as u8;

                    result.push(first as char);
                    result.push(last as char);
                }
                _ => {
                    let first = 0b1110_0000 + ((ch >> 12) & 0b0000_1111) as u8;
                    let mid = 0b1000_0000 + ((ch >> 6) & 0b0011_1111) as u8;
                    let last = 0b1000_0000 + (ch & 0b0011_1111) as u8;

                    result.push(first as char);
                    result.push(mid as char);
                    result.push(last as char);
                }
            }
        }

        Ok(result)
    }

    fn verify_len(&mut self, len: usize) -> Result<(), Error> {
        if len > self.alloc_limit_bytes {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "length of string is too large ({len} bytes): surpassed the default (customizable) limit of {} bytes",
                    self.alloc_limit_bytes
                ),
            ))
        } else {
            Ok(())
        }
    }
}

/// **String and buffer related methods that require `T: `[`std::io::BufRead`]**
impl<T: BufRead> Muncher<T> {
    /// Reads a C-style string (ending with `\0` null byte)
    /// into a buffer of bytes (`Vec<u8>`).
    ///
    /// **The resulting buffer does not include a null byte!
    /// Add one on your own if necessary!**
    ///
    /// If you want the UTF-8 format (unicode, extension of ASCII, widely used),
    /// see [`Muncher::read_cstr_utf8`].
    pub fn read_cstr_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        let bytes_read = self.reader.read_until(0, &mut buf)?;

        if bytes_read == 0 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "EOF reached before null terminator",
            ));
        }

        // Remove the null terminator
        if buf.last() == Some(&0) {
            buf.pop();
        }
        Ok(buf)
    }

    /// Reads a C-style string (ending with `\0` null byte)
    /// in the UTF-8 format (tries to parse, fails if invalid).
    ///
    /// If you want bytes or some other format,
    /// see [`Muncher::read_cstr_bytes`].
    pub fn read_cstr_utf8(&mut self) -> Result<String, Error> {
        let buf = self.read_cstr_bytes()?;
        bytes2utf8(buf)
    }

    /// Reads a line of UTF8 string (tries to parse, fails if invalid).
    /// This reads until a `\n` newline or an end-of-file is reached.
    pub fn read_line_utf8(&mut self) -> Result<String, Error> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        Ok(buf)
    }

    /// Reads bytes until the specified delimiter is encountered.
    ///
    /// Essentially it reads bytes again and again until it hits a byte that
    /// is the `delim` argument. The resulting buffer will include the delimiter
    /// byte if found.
    pub fn read_delim_bytes(&mut self, delim: u8) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        self.reader.read_until(delim, &mut buf)?;
        Ok(buf)
    }
}

fn bytes2utf8(bytes: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}
