#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterType {
    NewLine,
    Space,
    Tab,
    ParanthesisStart,
    ParanthesisEnd,
    SquareParanthesisStart,
    SquareParanthesisEnd,
    Colon,
    Comma,
    SemiColon,
    EndOfFile,
    Value,
}

#[inline(always)]
fn parse_8_digits_swar(bytes: &[u8]) -> u64 {
    // Read 8 bytes as a single u64 native integer
    let chunk = u64::from_le_bytes(bytes.try_into().unwrap());
    // Subtract ASCII '0' (0x30) from all 8 bytes simultaneously
    let digits = chunk - 0x3030303030303030;
    // SWAR multiplication formula to condense 8 digits into 1 integer
    let mul1 =
        (digits & 0x00FF00FF00FF00FF).wrapping_mul(10) + ((digits >> 8) & 0x00FF00FF00FF00FF);
    let mul2 = (mul1 & 0x0000FFFF0000FFFF).wrapping_mul(100) + ((mul1 >> 16) & 0x0000FFFF0000FFFF);
    let result = (mul2 & 0x00000000FFFFFFFF).wrapping_mul(10000) + (mul2 >> 32);
    result
}

#[inline(always)]
fn parse_up_to_8_digits_swar(bytes: &[u8]) -> u64 {
    if bytes.len() == 8 {
        return parse_8_digits_swar(bytes);
    }
    // Stack allocated — no heap allocation.
    let mut padded = [b'0'; 8];
    // Right-align the digits:
    // "123" becomes
    // "00000123"
    let start = 8 - bytes.len();
    padded[start..].copy_from_slice(bytes);
    parse_8_digits_swar(&padded)
}

#[inline(always)]
pub fn parse_u64_swar(bytes: &[u8]) -> Option<u64> {
    if bytes.is_empty() || bytes.len() > 20 {
        return None;
    }
    let len = bytes.len();
    // First group can contain 1..8 digits.
    // Examples:
    // 123456789012345678
    // first = 12
    // then  = 34567890
    // then  = 12345678
    let first_len = {
        let rem = len % 8;
        if rem == 0 { 8 } else { rem }
    };

    let mut result = parse_up_to_8_digits_swar(&bytes[..first_len]) as u64;
    let mut pos = first_len;
    while pos < len {
        let value = parse_8_digits_swar(&bytes[pos..pos + 8]) as u64;
        result = result.checked_mul(100_000_000)?;
        result = result.checked_add(value)?;
        pos += 8;
    }
    Some(result)
}
