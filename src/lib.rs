#![doc = "../README.md"]

mod impls;
mod io_read;
#[cfg(feature = "mutf8")]
mod mutf;
mod traits;

/// A helpful wrapper around anything that implements [`std::io::Read`].
///
/// You can put in any [`std::io::Read`] type and get access to the additional
/// bytemuncher methods for it, such as:
/// - Reading various signed/unsigned integer types in various endianness (see [`Endianness`]).
/// - Reading floating point values in various endianness.
/// - Reading strings in various formats (UTF-8, MUTF-8, UCS-2, raw bytes)
//    from various storage types (Null terminated, length prefix, newline, ...)
///
/// # Example
/// ```
/// use bytemuncher::Muncher;
/// use std::io::Cursor;
///
/// let data = [
///     0x34, 0x12, 0x56, 0x78,
///     0x00, 0x01,
/// ];
/// let mut muncher = Muncher::new(Cursor::new(data));
///
/// assert_eq!(muncher.read_le::<u16>().unwrap(), 0x1234);
/// assert_eq!(muncher.read_be::<u16>().unwrap(), 0x5678);
///
/// assert_eq!(muncher.read_le::<u8>().unwrap(), 0);
/// assert_eq!(muncher.read_le::<u8>().unwrap(), 1);
///
/// // End of data
/// assert!(muncher.read_le::<u8>().is_err());
/// ```
///
/// # Errors
/// All methods return [`std::io::Error`] on failure, except
/// for MUTF-8 methods (crate feature: `mutf8`) which return `MutfError`
/// on failure.
///
/// They can come from:
/// - I/O errors from the underlying reader
/// - Unexpected end-of-input
/// - Invalid encoding (e.g., malformed UTF-8 or MUTF-8)
///
/// # Note on strings
/// The string-related methods are named in the
/// `read_<format>_<encoding>[_destination]` scheme.
///
/// ## Format
/// Specifies how the size of the string is found.
/// It can be one of:
/// - `size8`/`size16`: A u8 or u16 prefix before the
///   contents indicating the length of the string.
/// - `cstr`: The string ends with a null `\0` byte.
/// - `line` (`BufRead` only): Reads till a newline or end-of-file is met.
/// - `delim` (`BufRead` only): Reads till the byte specified in the argument
///   is met or the input has ended, and returns a string containing the byte if found.
///
/// ## Encoding
/// Specifies the character encoding of the string.
/// It can be one of:
/// - `bytes`: Returns a raw `Vec<u8>` containing the bytes,
///   feel free to interpret as you wish.
/// - `utf8`: Reads and returns a UTF-8 encoded [`String`]
///   (tries to parse, fails if invalid). This is the default
///   rust type of string.
/// - `mutf8`: (with crate feature `mutf8`)
///   Reads a MUTF-8 encoded string and converts it to
///   a UTF-8 [`String`]. This is usually not used unless
///   you specifically need MUTF-8 support.
/// - `ucs2`: Reads as the UCS-2 string format (explained below),
///   and converts it to a UTF-8 [`String`].
///
/// UCS-2 consists of big endian 16-bit words, each of which represent
/// a Unicode code point between U+0000 and U+FFFF inclusive.
///
/// ## Destination (optional)
/// Specifies where to store the data to. If it's not specified then
/// that usually means a new heap-allocated buffer ([`String`] or [`Vec`])
/// is made. Otherwise:
/// - `to`: (TODO) Writes to a `&mut String` or `&mut Vec`
/// - `to_exact`: (TODO) Writes to any `&mut [u8]`,
///   returns `bool` if it fit in or not.
///
/// More destinations may come in future versions.
pub struct Muncher<T> {
    reader: T,
}

impl<T> Muncher<T> {
    /// Creates a new [`Muncher`].
    ///
    /// If you want to iterate over some `[u8]`
    /// then use
    /// ```no_run
    /// # use bytemuncher::Muncher;
    /// # let SOMETHING = [0];
    /// # _ =
    /// Muncher::new(std::io::Cursor::new(SOMETHING))
    /// # ;
    /// ```
    pub fn new(reader: T) -> Self {
        Self { reader }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
    Native,
}

impl Endianness {
    pub fn is_le(self) -> bool {
        match self {
            Endianness::Little => true,
            Endianness::Big => false,
            Endianness::Native => IS_TARGET_LITTLE_ENDIAN,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Endianness::Little => Endianness::Big,
            Endianness::Big => Endianness::Little,
            Endianness::Native => {
                if IS_TARGET_LITTLE_ENDIAN {
                    Endianness::Big
                } else {
                    Endianness::Little
                }
            }
        }
    }
}

#[cfg(target_endian = "little")]
pub const IS_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(target_endian = "big")]
pub const IS_LITTLE_ENDIAN: bool = false;

#[cfg(feature = "mutf8")]
pub use mutf::MutfError;
#[cfg(feature = "mutf8")]
pub use mutf8::error::Error as MutfInnerError;

// #[cfg(feature = "mutf8")]
// pub use mutf8::{mutf8_to_utf8, utf8_to_mutf8};

pub use traits::ReadEndian;
