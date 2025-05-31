use crate::{Endianness, Muncher};
use std::io::{BufRead, Error, ErrorKind, Read};

/// **Size-prefixed string methods**
impl<T: Read> Muncher<T> {
    /// Reads a UTF-8 string prefixed by a `u16` length (number of bytes).
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::Endianness`].
    pub fn read_size16_utf8(&mut self, end: Endianness) -> Result<String, Error> {
        let len = self.read_u16_endian(end)?;
        let mut buf = vec![0u8; len as usize];
        self.reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    /// Reads a UCS-2 string prefixed by a `u16` length (number of characters).
    ///
    /// UCS-2 consists of big endian 16-bit words, each of which represent a Unicode
    /// code point between U+0000 and U+FFFF inclusive.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::Endianness`].
    pub fn read_size16_ucs2(&mut self, end: Endianness) -> Result<String, Error> {
        let char_count = self.read_u16_endian(end)?;
        let mut result = String::with_capacity(char_count as usize);

        for _ in 0..char_count {
            let code_unit = self.read_u16_be()?; // UCS-2 is always big-endian
            match char::from_u32(code_unit as u32) {
                Some(ch) => result.push(ch),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Invalid UCS-2 code unit: {:#X}", code_unit),
                    ));
                }
            }
        }

        Ok(result)
    }

    /// Reads a UTF-8 string prefixed by a `u8` length (number of bytes).
    pub fn read_size8_utf8(&mut self) -> Result<String, Error> {
        let len = self.read_u8()?;
        let mut buf = vec![0u8; len as usize];
        self.reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    /// Reads a UCS-2 string prefixed by a `u8` length (number of characters).
    ///
    /// UCS-2 consists of big endian 16-bit words, each of which represent a Unicode
    /// code point between U+0000 and U+FFFF inclusive.
    pub fn read_size8_ucs2(&mut self) -> Result<String, Error> {
        let char_count = self.read_u8()?;
        let mut result = String::with_capacity(char_count as usize);

        for _ in 0..char_count {
            let code_unit = self.read_u16_be()?; // UCS-2 is always big-endian
            match char::from_u32(code_unit as u32) {
                Some(ch) => result.push(ch),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Invalid UCS-2 code unit: {:#X}", code_unit),
                    ));
                }
            }
        }

        Ok(result)
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
        String::from_utf8(buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))
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
