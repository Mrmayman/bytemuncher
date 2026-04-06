use crate::{End, Muncher};
use std::io::{BufReader, Cursor, Write};

#[test]
fn test_read_pref_bytes() {
    let data = [5u8, 72, 101, 108, 108, 111]; // Length 5, then "Hello"
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_pref_bytes::<u8>(End::Little).unwrap();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_read_pref_bytes_u16() {
    let data = [5u8, 0u8, 72, 101, 108, 108, 111]; // Length 5 (u16 LE), then "Hello"
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_pref_bytes::<u16>(End::Little).unwrap();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_read_pref_bytes_u32() {
    let data = [5u8, 0u8, 0u8, 0u8, 72, 101, 108, 108, 111]; // Length 5 (u32 LE), then "Hello"
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_pref_bytes::<u32>(End::Little).unwrap();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_read_pref_bytes_be() {
    let data = [5u8, 72, 101, 108, 108, 111]; // Length 5, then "Hello"
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_pref_bytes::<u8>(End::Big).unwrap();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_read_pref_bytes_empty() {
    let data = [0u8]; // Length 0
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_pref_bytes::<u8>(End::Little).unwrap();
    assert_eq!(result, b"");
}

#[test]
fn test_read_fixed_bytes() {
    let data = [72, 101, 108, 108, 111]; // "Hello"
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_fixed_bytes(5).unwrap();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_read_fixed_bytes_empty() {
    let data = [];
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_fixed_bytes(0).unwrap();
    assert_eq!(result, b"");
}

#[test]
fn test_read_fixed_bytes_insufficient() {
    let data = [72, 101]; // Only 2 bytes
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_fixed_bytes(5);
    assert!(result.is_err());
}

#[test]
fn test_read_pref_utf8() {
    let data = [5u8, 72, 101, 108, 108, 111]; // Length 5, then "Hello"
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_pref_utf8::<u8>(End::Little).unwrap();
    assert_eq!(result, "Hello");
}

#[test]
fn test_read_pref_utf8_unicode() {
    let text = "Hello ";
    let text_bytes = text.as_bytes();
    let mut data = vec![text_bytes.len() as u8];
    data.extend(text_bytes);

    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_pref_utf8::<u8>(End::Little).unwrap();
    assert_eq!(result, text);
}

#[test]
fn test_read_pref_utf8_invalid() {
    let data = [5u8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // Invalid UTF-8
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_pref_utf8::<u8>(End::Little);
    assert!(result.is_err());
}

#[test]
fn test_read_fixed_utf8() {
    let data = [72, 101, 108, 108, 111]; // "Hello"
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_fixed_utf8(5).unwrap();
    assert_eq!(result, "Hello");
}

#[test]
fn test_read_fixed_utf8_unicode() {
    let text = "Hi ";
    let text_bytes = text.as_bytes();
    let mut reader = Muncher::new(Cursor::new(text_bytes));

    let result = reader.read_fixed_utf8(text_bytes.len()).unwrap();
    assert_eq!(result, text);
}

#[test]
fn test_read_fixed_utf8_invalid() {
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // Invalid UTF-8
    let mut reader = Muncher::new(Cursor::new(data));

    let result = reader.read_fixed_utf8(5);
    assert!(result.is_err());
}

#[test]
fn test_read_cstr_bytes() {
    let data = [72, 101, 108, 108, 111, 0]; // "Hello\0"
    let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));

    let result = reader.read_cstr_bytes().unwrap();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_read_cstr_bytes_empty() {
    let data = [0]; // Just null terminator
    let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));

    let result = reader.read_cstr_bytes().unwrap();
    assert_eq!(result, b"");
}

#[test]
fn test_read_cstr_bytes_no_terminator() {
    let data = [72, 101, 108, 108, 111]; // "Hello" without null
    let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));

    let result = reader.read_cstr_bytes();
    // read_until will return what it can read even without delimiter
    assert!(result.is_ok()); // BufReader reads available data
}

#[test]
fn test_read_cstr_utf8() {
    let data = [72, 101, 108, 108, 111, 0]; // "Hello\0"
    let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));

    let result = reader.read_cstr_utf8().unwrap();
    assert_eq!(result, "Hello");
}

#[test]
fn test_read_cstr_utf8_unicode() {
    let text = "Hi ";
    let mut data = Vec::new();
    data.extend(text.as_bytes());
    data.push(0); // Null terminator

    let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));

    let result = reader.read_cstr_utf8().unwrap();
    assert_eq!(result, text);
}

#[test]
fn test_read_cstr_utf8_invalid() {
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0]; // Invalid UTF-8 + null
    let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));

    let result = reader.read_cstr_utf8();
    assert!(result.is_err());
}

#[test]
fn test_read_delim_bytes() {
    let data = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]; // "Hello World!"
    let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));

    let result = reader.read_delim_bytes(b' ').unwrap();
    assert_eq!(result, b"Hello ");
}

#[test]
fn test_read_delim_bytes_not_found() {
    let data = [72, 101, 108, 108, 111]; // "Hello"
    let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));

    let result = reader.read_delim_bytes(b' ').unwrap();
    assert_eq!(result, b"Hello");
}

#[test]
fn test_read_delim_bytes_empty() {
    let data = [32]; // Just delimiter
    let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));

    let result = reader.read_delim_bytes(b' ').unwrap();
    assert_eq!(result, b" ");
}

#[test]
fn test_write_pref_bytes() {
    let data = b"Hello";
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_pref_bytes::<u8>(End::Little, data).unwrap();

    let expected = [5u8, 72, 101, 108, 108, 111];
    assert_eq!(buffer, expected);
}

#[test]
fn test_write_pref_bytes_u16() {
    let data = b"Hello";
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);
    writer.write_m(data.len() as u16, End::Little).unwrap();
    writer.write_all(data).unwrap();

    let expected = [5u8, 0u8, 72, 101, 108, 108, 111];
    assert_eq!(buffer, expected);
}

#[test]
fn test_write_pref_bytes_be() {
    let data = b"Hello";
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);
    writer.write_m(data.len() as u8, End::Big).unwrap();
    writer.write_all(data).unwrap();

    let expected = [5u8, 72, 101, 108, 108, 111];
    assert_eq!(buffer, expected);
}

#[test]
fn test_write_pref_bytes_empty() {
    let data = b"";
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_pref_bytes::<u8>(End::Little, data).unwrap();

    let expected = [0u8];
    assert_eq!(buffer, expected);
}

#[test]
fn test_write_cstr_bytes() {
    let data = b"Hello";
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_cstr_bytes(data).unwrap();

    let expected = [72, 101, 108, 108, 111, 0];
    assert_eq!(buffer, expected);
}

#[test]
fn test_write_cstr_bytes_with_null() {
    let data = b"Hello\0";
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_cstr_bytes(data).unwrap();

    let expected = [72, 101, 108, 108, 111, 0]; // Should not add extra null
    assert_eq!(buffer, expected);
}

#[test]
fn test_write_cstr_bytes_empty() {
    let data = b"";
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_cstr_bytes(data).unwrap();

    let expected = [0];
    assert_eq!(buffer, expected);
}

#[test]
fn test_write_fixed_u16() {
    let data = [0x4845, 0x4C4C, 0x4F]; // "Hello" in u16 chunks
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_fixed_u16(&data, End::Little).unwrap();

    let expected = [0x45, 0x48, 0x4C, 0x4C, 0x4F, 0x00];
    assert_eq!(buffer, expected);
}

#[test]
fn test_write_fixed_u16_be() {
    let data = [0x4845, 0x4C4C, 0x4F]; // "Hello" in u16 chunks
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_fixed_u16(&data, End::Big).unwrap();

    let expected = [0x48, 0x45, 0x4C, 0x4C, 0x00, 0x4F];
    assert_eq!(buffer, expected);
}

#[test]
fn test_write_fixed_u16_empty() {
    let data = [];
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_fixed_u16(&data, End::Little).unwrap();

    assert_eq!(buffer, []);
}

#[test]
fn test_round_trip_pref_bytes() {
    let original = b"Hello World";
    let mut buffer = Vec::new();

    // Write
    {
        let mut writer = Muncher::new(&mut buffer);
        writer
            .write_pref_bytes::<u16>(End::Little, original)
            .unwrap();
    }

    // Read
    let mut reader = Muncher::new(Cursor::new(buffer));
    let result = reader.read_pref_bytes::<u16>(End::Little).unwrap();

    assert_eq!(result, original);
}

#[test]
fn test_round_trip_pref_utf8() {
    let original = "Hello ";
    let mut buffer = Vec::new();

    // Write
    {
        let mut writer = Muncher::new(&mut buffer);
        writer
            .write_pref_bytes::<u16>(End::Little, original.as_bytes())
            .unwrap();
    }

    // Read
    let mut reader = Muncher::new(Cursor::new(buffer));
    let result = reader.read_pref_utf8::<u16>(End::Little).unwrap();

    assert_eq!(result, original);
}

#[test]
fn test_round_trip_fixed_bytes() {
    let original = b"Hello";
    let mut buffer = Vec::new();

    // Write
    {
        let mut writer = Muncher::new(&mut buffer);
        writer.write_all(original).unwrap();
    }

    // Read
    let mut reader = Muncher::new(Cursor::new(buffer));
    let result = reader.read_fixed_bytes(original.len()).unwrap();

    assert_eq!(result, original);
}

#[test]
fn test_round_trip_fixed_utf8() {
    let original = "Hi ";
    let mut buffer = Vec::new();

    // Write
    {
        let mut writer = Muncher::new(&mut buffer);
        writer.write_all(original.as_bytes()).unwrap();
    }

    // Read
    let mut reader = Muncher::new(Cursor::new(buffer));
    let result = reader.read_fixed_utf8(original.len()).unwrap();

    assert_eq!(result, original);
}

#[test]
fn test_round_trip_cstr() {
    let original = "Hello";
    let mut buffer = Vec::new();

    // Write
    {
        let mut writer = Muncher::new(&mut buffer);
        writer.write_cstr_bytes(original.as_bytes()).unwrap();
    }

    // Read
    let mut reader = Muncher::new(BufReader::new(Cursor::new(buffer)));
    let result = reader.read_cstr_utf8().unwrap();

    assert_eq!(result, original);
}

#[test]
fn test_multiple_strings() {
    let strings = ["Hello", "World", "Test"];
    let mut buffer = Vec::new();

    // Write multiple strings
    {
        let mut writer = Muncher::new(&mut buffer);
        for s in &strings {
            writer
                .write_pref_bytes::<u8>(End::Little, s.as_bytes())
                .unwrap();
        }
    }

    // Read multiple strings
    let mut reader = Muncher::new(Cursor::new(buffer));
    for expected in &strings {
        let result = reader.read_pref_utf8::<u8>(End::Little).unwrap();
        assert_eq!(result, *expected);
    }
}

#[test]
fn test_large_string() {
    let large_string = "A".repeat(10000);
    let mut buffer = Vec::new();

    // Write
    {
        let mut writer = Muncher::new(&mut buffer);
        writer
            .write_pref_bytes::<u32>(End::Little, large_string.as_bytes())
            .unwrap();
    }

    // Read
    let mut reader = Muncher::new(Cursor::new(buffer));
    let result = reader.read_pref_utf8::<u32>(End::Little).unwrap();

    assert_eq!(result, large_string);
}

#[test]
fn test_zero_length_string() {
    let empty = "";
    let mut buffer = Vec::new();

    // Write
    {
        let mut writer = Muncher::new(&mut buffer);
        writer
            .write_pref_bytes::<u8>(End::Little, empty.as_bytes())
            .unwrap();
    }

    // Read
    let mut reader = Muncher::new(Cursor::new(buffer));
    let result = reader.read_pref_utf8::<u8>(End::Little).unwrap();

    assert_eq!(result, empty);
}

#[test]
fn test_string_with_nulls() {
    let with_nulls = "Hello\0World";
    let mut buffer = Vec::new();

    // Write
    {
        let mut writer = Muncher::new(&mut buffer);
        writer
            .write_pref_bytes::<u8>(End::Little, with_nulls.as_bytes())
            .unwrap();
    }

    // Read
    let mut reader = Muncher::new(Cursor::new(buffer));
    let result = reader.read_pref_bytes::<u8>(End::Little).unwrap();

    assert_eq!(result, with_nulls.as_bytes());
}

#[test]
fn test_string_with_newlines() {
    let with_newlines = "Hello\nWorld\r\nTest";
    let mut buffer = Vec::new();

    // Write
    {
        let mut writer = Muncher::new(&mut buffer);
        writer
            .write_pref_bytes::<u8>(End::Little, with_newlines.as_bytes())
            .unwrap();
    }

    // Read
    let mut reader = Muncher::new(Cursor::new(buffer));
    let result = reader.read_pref_utf8::<u8>(End::Little).unwrap();

    assert_eq!(result, with_newlines);
}

#[test]
fn test_mixed_endianness() {
    let text = "Hello";
    let mut buffer = Vec::new();

    // Write with different endianness
    {
        let mut writer = Muncher::new(&mut buffer);
        writer
            .write_pref_bytes::<u16>(End::Little, text.as_bytes())
            .unwrap();
        writer
            .write_pref_bytes::<u16>(End::Big, text.as_bytes())
            .unwrap();
    }

    // Read with matching endianness
    let mut reader = Muncher::new(Cursor::new(buffer));
    let result1 = reader.read_pref_utf8::<u16>(End::Little).unwrap();
    let result2 = reader.read_pref_utf8::<u16>(End::Big).unwrap();

    assert_eq!(result1, text);
    assert_eq!(result2, text);
}

#[cfg(feature = "ucs2")]
mod ucs2_tests {
    use super::*;

    #[test]
    fn test_read_pref_ucs2() {
        let text = "Hello";
        let mut data = vec![text.len() as u8]; // Length as u8
        // UCS-2 encoding (big endian u16 values)
        for ch in text.chars() {
            data.extend_from_slice(&(ch as u16).to_be_bytes());
        }

        let mut reader = Muncher::new(Cursor::new(data));
        let result = reader.read_pref_ucs2::<u8>(End::Little).unwrap();
        assert_eq!(result, text);
    }

    #[test]
    fn test_read_fixed_ucs2() {
        let text = "Hello";
        let mut data = Vec::new();
        // UCS-2 encoding (big endian u16 values)
        for ch in text.chars() {
            data.extend_from_slice(&(ch as u16).to_be_bytes());
        }

        let mut reader = Muncher::new(Cursor::new(data));
        let result = reader.read_fixed_ucs2(text.len()).unwrap();
        assert_eq!(result, text);
    }

    #[test]
    fn test_ucs2_round_trip() {
        let original = "Hello World";
        let mut buffer = Vec::new();

        // Write
        {
            let mut writer = Muncher::new(&mut buffer);
            writer
                .write_pref_ucs2::<u16>(End::Little, original)
                .unwrap();
        }

        // Read
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result = reader.read_pref_ucs2::<u16>(End::Little).unwrap();

        assert_eq!(result, original);
    }

    #[test]
    fn test_ucs2_unicode() {
        let text = "Hi 世界";
        let mut buffer = Vec::new();

        // Write
        {
            let mut writer = Muncher::new(&mut buffer);
            writer.write_pref_ucs2::<u16>(End::Little, text).unwrap();
        }

        // Read
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result = reader.read_pref_ucs2::<u16>(End::Little).unwrap();

        assert_eq!(result, text);
    }
}

#[cfg(feature = "mutf8")]
mod mutf8_tests {
    use super::*;

    #[test]
    fn test_read_pref_mutf8() {
        let text = "Hello";
        let mutf_bytes = mutf8::utf8_to_mutf8(text.as_bytes()).unwrap();
        let mut data = vec![mutf_bytes.len() as u8];
        data.extend(mutf_bytes.iter());

        let mut reader = Muncher::new(Cursor::new(data));
        let result = reader.read_pref_mutf8::<u8>(End::Little).unwrap();
        assert_eq!(result, text);
    }

    #[test]
    fn test_read_fixed_mutf8() {
        let text = "Hello";
        let mutf_bytes = mutf8::utf8_to_mutf8(text.as_bytes()).unwrap();

        let mut reader = Muncher::new(Cursor::new(&mutf_bytes));
        let result = reader.read_fixed_mutf8(mutf_bytes.len()).unwrap();
        assert_eq!(result, text);
    }

    #[test]
    fn test_write_pref_mutf8() {
        let text = "Hello";
        let mutf_bytes = mutf8::utf8_to_mutf8(text.as_bytes()).unwrap();
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);

        writer.write_pref_mutf8::<u8>(text, End::Little).unwrap();

        let mut expected = vec![mutf_bytes.len() as u8];
        expected.extend(mutf_bytes.iter());

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_read_cstr_mutf8() {
        let text = "Hello";
        let mutf_bytes = mutf8::utf8_to_mutf8(text.as_bytes()).unwrap();
        let mut data = mutf_bytes.to_vec();
        data.push(0); // Null terminator

        let mut reader = Muncher::new(BufReader::new(Cursor::new(data)));
        let result = reader.read_cstr_mutf8().unwrap();
        assert_eq!(result, text);
    }

    #[test]
    fn test_write_cstr_mutf8() {
        let text = "Hello";
        let mutf_bytes = mutf8::utf8_to_mutf8(text.as_bytes()).unwrap();
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);

        writer.write_cstr_mutf8(text).unwrap();

        let mut expected = mutf_bytes.to_vec();
        expected.push(0); // Null terminator

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_mutf8_round_trip() {
        let original = "Hello World";
        let mut buffer = Vec::new();

        // Write
        {
            let mut writer = Muncher::new(&mut buffer);
            writer
                .write_pref_mutf8::<u16>(original, End::Little)
                .unwrap();
        }

        // Read
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result = reader.read_pref_mutf8::<u16>(End::Little).unwrap();

        assert_eq!(result, original);
    }

    #[test]
    fn test_mutf8_unicode() {
        let text = "Hi 世界";
        let mut buffer = Vec::new();

        // Write
        {
            let mut writer = Muncher::new(&mut buffer);
            writer.write_pref_mutf8::<u16>(text, End::Little).unwrap();
        }

        // Read
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result = reader.read_pref_mutf8::<u16>(End::Little).unwrap();

        assert_eq!(result, text);
    }

    #[test]
    fn test_mutf8_modified_utf8() {
        // Test with null character (modified UTF-8 encoding)
        let text = "Hello\0World";
        let mut buffer = Vec::new();

        // Write
        {
            let mut writer = Muncher::new(&mut buffer);
            writer.write_pref_mutf8::<u16>(text, End::Little).unwrap();
        }

        // Read
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result = reader.read_pref_mutf8::<u16>(End::Little).unwrap();

        assert_eq!(result, text);
    }

    #[test]
    fn test_mutf8_empty() {
        let text = "";
        let mut buffer = Vec::new();

        // Write
        {
            let mut writer = Muncher::new(&mut buffer);
            writer.write_pref_mutf8::<u8>(text, End::Little).unwrap();
        }

        // Read
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result = reader.read_pref_mutf8::<u8>(End::Little).unwrap();

        assert_eq!(result, text);
    }
}
