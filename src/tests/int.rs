use crate::{End, Muncher};
use std::io::Cursor;

#[test]
fn test_u8_read_write() {
    let original = 42u8;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u8 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_i8_read_write() {
    let original = -42i8;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: i8 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u16_read_write_le() {
    let original = 0x1234u16;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u16 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u16_read_write_be() {
    let original = 0x1234u16;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u16 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u16_read_write_ne() {
    let original = 0x1234u16;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_ne(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u16 = reader.read_ne().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_i16_read_write_le() {
    let original = -12345i16;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: i16 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_i16_read_write_be() {
    let original = -12345i16;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: i16 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u32_read_write_le() {
    let original = 0x12345678u32;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u32 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u32_read_write_be() {
    let original = 0x12345678u32;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u32 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_i32_read_write_le() {
    let original = -1234567890i32;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: i32 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_i32_read_write_be() {
    let original = -1234567890i32;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: i32 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u64_read_write_le() {
    let original = 0x123456789ABCDEF0u64;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u64 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u64_read_write_be() {
    let original = 0x123456789ABCDEF0u64;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u64 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_i64_read_write_le() {
    let original = -1234567890123456789i64;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: i64 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_i64_read_write_be() {
    let original = -1234567890123456789i64;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: i64 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u128_read_write_le() {
    let original = 0x123456789ABCDEF0123456789ABCDEF0u128;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u128 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u128_read_write_be() {
    let original = 0x123456789ABCDEF0123456789ABCDEF0u128;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: u128 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_i128_read_write_le() {
    let original = -0x123456789ABCDEF0123456789ABCDEF0i128;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: i128 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_i128_read_write_be() {
    let original = -0x123456789ABCDEF0123456789ABCDEF0i128;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: i128 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_u16_endian_consistency() {
    let value = 0x1234u16;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    assert_eq!(buffer_le, value.to_le_bytes().to_vec());
    assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_u32_endian_consistency() {
    let value = 0x12345678u32;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    assert_eq!(buffer_le, value.to_le_bytes().to_vec());
    assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_u64_endian_consistency() {
    let value = 0x123456789ABCDEF0u64;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    assert_eq!(buffer_le, value.to_le_bytes().to_vec());
    assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_u128_endian_consistency() {
    let value = 0x123456789ABCDEF0123456789ABCDEF0u128;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    assert_eq!(buffer_le, value.to_le_bytes().to_vec());
    assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_i16_endian_consistency() {
    let value = -12345i16;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    assert_eq!(buffer_le, value.to_le_bytes().to_vec());
    assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_i32_endian_consistency() {
    let value = -1234567890i32;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    assert_eq!(buffer_le, value.to_le_bytes().to_vec());
    assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_i64_endian_consistency() {
    let value = -1234567890123456789i64;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    assert_eq!(buffer_le, value.to_le_bytes().to_vec());
    assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_i128_endian_consistency() {
    let value = -0x123456789ABCDEF0123456789ABCDEF0i128;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    assert_eq!(buffer_le, value.to_le_bytes().to_vec());
    assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_u8_multiple_values() {
    let values = [1u8, 2, 3, 4, 5];
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    for &value in &values {
        writer.write_le(value).unwrap();
    }

    let mut reader = Muncher::new(Cursor::new(buffer));
    for &expected in &values {
        let result: u8 = reader.read_le().unwrap();
        assert_eq!(expected, result);
    }
}

#[test]
fn test_u16_multiple_values() {
    let values = [1u16, 2, 3, 4, 5];
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    for &value in &values {
        writer.write_le(value).unwrap();
    }

    let mut reader = Muncher::new(Cursor::new(buffer));
    for &expected in &values {
        let result: u16 = reader.read_le().unwrap();
        assert_eq!(expected, result);
    }
}

#[test]
fn test_u32_multiple_values() {
    let values = [1u32, 2, 3, 4, 5];
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    for &value in &values {
        writer.write_be(value).unwrap();
    }

    let mut reader = Muncher::new(Cursor::new(buffer));
    for &expected in &values {
        let result: u32 = reader.read_be().unwrap();
        assert_eq!(expected, result);
    }
}

#[test]
fn test_u64_multiple_values() {
    let values = [1u64, 2, 3, 4, 5];
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    for &value in &values {
        writer.write_be(value).unwrap();
    }

    let mut reader = Muncher::new(Cursor::new(buffer));
    for &expected in &values {
        let result: u64 = reader.read_be().unwrap();
        assert_eq!(expected, result);
    }
}

#[test]
fn test_u128_multiple_values() {
    let values = [1u128, 2, 3, 4, 5];
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    for &value in &values {
        writer.write_be(value).unwrap();
    }

    let mut reader = Muncher::new(Cursor::new(buffer));
    for &expected in &values {
        let result: u128 = reader.read_be().unwrap();
        assert_eq!(expected, result);
    }
}

#[test]
fn test_mixed_endian_operations() {
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(0x1234u16).unwrap();
    writer.write_be(0x5678u16).unwrap();
    writer.write_m(0x9ABCu16, End::Little).unwrap();
    writer.write_m(0xDEF0u16, End::Big).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));

    let result1: u16 = reader.read_le().unwrap();
    assert_eq!(result1, 0x1234);

    let result2: u16 = reader.read_be().unwrap();
    assert_eq!(result2, 0x5678);

    let result3: u16 = reader.read_m(End::Little).unwrap();
    assert_eq!(result3, 0x9ABC);

    let result4: u16 = reader.read_m(End::Big).unwrap();
    assert_eq!(result4, 0xDEF0);
}

#[test]
fn test_edge_cases_unsigned() {
    // Test u8
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(u8::MIN).unwrap();
        writer.write_be(u8::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: u8 = reader.read_le().unwrap();
        let result_max: u8 = reader.read_be().unwrap();
        assert_eq!(result_min, u8::MIN);
        assert_eq!(result_max, u8::MAX);
    }

    // Test u16
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(u16::MIN).unwrap();
        writer.write_be(u16::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: u16 = reader.read_le().unwrap();
        let result_max: u16 = reader.read_be().unwrap();
        assert_eq!(result_min, u16::MIN);
        assert_eq!(result_max, u16::MAX);
    }

    // Test u32
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(u32::MIN).unwrap();
        writer.write_be(u32::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: u32 = reader.read_le().unwrap();
        let result_max: u32 = reader.read_be().unwrap();
        assert_eq!(result_min, u32::MIN);
        assert_eq!(result_max, u32::MAX);
    }

    // Test u64
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(u64::MIN).unwrap();
        writer.write_be(u64::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: u64 = reader.read_le().unwrap();
        let result_max: u64 = reader.read_be().unwrap();
        assert_eq!(result_min, u64::MIN);
        assert_eq!(result_max, u64::MAX);
    }

    // Test u128
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(u128::MIN).unwrap();
        writer.write_be(u128::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: u128 = reader.read_le().unwrap();
        let result_max: u128 = reader.read_be().unwrap();
        assert_eq!(result_min, u128::MIN);
        assert_eq!(result_max, u128::MAX);
    }
}

#[test]
fn test_edge_cases_signed() {
    // Test i8
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(i8::MIN).unwrap();
        writer.write_be(i8::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: i8 = reader.read_le().unwrap();
        let result_max: i8 = reader.read_be().unwrap();
        assert_eq!(result_min, i8::MIN);
        assert_eq!(result_max, i8::MAX);
    }

    // Test i16
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(i16::MIN).unwrap();
        writer.write_be(i16::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: i16 = reader.read_le().unwrap();
        let result_max: i16 = reader.read_be().unwrap();
        assert_eq!(result_min, i16::MIN);
        assert_eq!(result_max, i16::MAX);
    }

    // Test i32
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(i32::MIN).unwrap();
        writer.write_be(i32::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: i32 = reader.read_le().unwrap();
        let result_max: i32 = reader.read_be().unwrap();
        assert_eq!(result_min, i32::MIN);
        assert_eq!(result_max, i32::MAX);
    }

    // Test i64
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(i64::MIN).unwrap();
        writer.write_be(i64::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: i64 = reader.read_le().unwrap();
        let result_max: i64 = reader.read_be().unwrap();
        assert_eq!(result_min, i64::MIN);
        assert_eq!(result_max, i64::MAX);
    }

    // Test i128
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(i128::MIN).unwrap();
        writer.write_be(i128::MAX).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result_min: i128 = reader.read_le().unwrap();
        let result_max: i128 = reader.read_be().unwrap();
        assert_eq!(result_min, i128::MIN);
        assert_eq!(result_max, i128::MAX);
    }
}

#[test]
fn test_insufficient_data() {
    // Test u8 (needs 1 byte)
    {
        let buffer: [u8; 0] = [];
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: Result<u8, _> = reader.read_le();
        assert!(result.is_err());
    }

    // Test u16 (needs 2 bytes)
    {
        let buffer = [0u8; 1];
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: Result<u16, _> = reader.read_le();
        assert!(result.is_err());
    }

    // Test u32 (needs 4 bytes)
    {
        let buffer = [0u8; 3];
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: Result<u32, _> = reader.read_le();
        assert!(result.is_err());
    }

    // Test u64 (needs 8 bytes)
    {
        let buffer = [0u8; 7];
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: Result<u64, _> = reader.read_le();
        assert!(result.is_err());
    }

    // Test u128 (needs 16 bytes)
    {
        let buffer = [0u8; 15];
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: Result<u128, _> = reader.read_le();
        assert!(result.is_err());
    }
}

#[test]
fn test_zero_values() {
    // Test u8
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0u8).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: u8 = reader.read_le().unwrap();
        assert_eq!(result, 0u8);
    }

    // Test i8
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0i8).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: i8 = reader.read_le().unwrap();
        assert_eq!(result, 0i8);
    }

    // Test u16
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0u16).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: u16 = reader.read_le().unwrap();
        assert_eq!(result, 0u16);
    }

    // Test i16
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0i16).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: i16 = reader.read_le().unwrap();
        assert_eq!(result, 0i16);
    }

    // Test u32
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0u32).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: u32 = reader.read_le().unwrap();
        assert_eq!(result, 0u32);
    }

    // Test i32
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0i32).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: i32 = reader.read_le().unwrap();
        assert_eq!(result, 0i32);
    }

    // Test u64
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0u64).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: u64 = reader.read_le().unwrap();
        assert_eq!(result, 0u64);
    }

    // Test i64
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0i64).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: i64 = reader.read_le().unwrap();
        assert_eq!(result, 0i64);
    }

    // Test u128
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0u128).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: u128 = reader.read_le().unwrap();
        assert_eq!(result, 0u128);
    }

    // Test i128
    {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(0i128).unwrap();
        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: i128 = reader.read_le().unwrap();
        assert_eq!(result, 0i128);
    }
}
