# bytemuncher
Simple, flexible extension methods for [Rust](rust-lang.org)'s `std::io::Read` trait.

You can wrap any [`std::io::Read`] type in the `bytemuncher::Muncher<T>` type and get access to the additional
bytemuncher methods for it, such as:
- Reading various signed/unsigned integer types in various endianness (see `bytemuncher::Endianness`).
- Reading floating point values in various endianness.
- Reading strings in various formats (UTF-8, MUTF-8, UCS-2, raw bytes)
  from various storage types (Null terminated, length prefix, newline, ...)

For more information, see the documentation of `bytemuncher::Muncher` and its methods.

# Example
```rust
use bytemuncher::Muncher;
use std::io::Cursor;

let data = [
    0x34, 0x12, 0x56, 0x78,
    0x00, 0x01,
];
let mut muncher = Muncher::new(Cursor::new(data));

assert_eq!(muncher.read_u16_le().unwrap(), 0x1234);
assert_eq!(muncher.read_u16_be().unwrap(), 0x5678);

assert_eq!(muncher.read_bool().unwrap(), false);
assert_eq!(muncher.read_bool().unwrap(), true);

// End of data
assert!(muncher.read_bool().is_err());
```

# TODO
- [x] Add MUTF-8 support (crate feature: `mutf8`)
- [ ] Add `tokio` async read support
- [ ] Add destinations for reading strings into
- [ ] Add test suite
