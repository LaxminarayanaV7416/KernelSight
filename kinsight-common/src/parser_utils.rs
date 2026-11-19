use super::cgroup_constants::CGROUP_V2_MOUNT_PATH;
use std::collections::HashMap;
use std::process::Command;

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

pub fn line_tracker(
    buffer: &[u8],
    config: &HashMap<&str, (usize, bool)>,
    seperator: u8,
) -> HashMap<usize, usize> {
    // this will give me the line number I need to look for
    // Logic of finding the line number, to keyword matching
    // we get buffers, hashmaps which contains the keyword as key,
    // values include struct position id and boolean result yes/no
    // do the strcmp with buffer and get the line number
    // when you do strcmp, you immediately know the field id and required yes/no
    // return line number, field id
    //
    // now parser will take the line numbers to look for when encountered
    // it will pass to the set field which takes the input of the field id
    // and assigns the value to the field in struct
    let mut result: HashMap<usize, usize> = HashMap::new();
    let mut line_number: usize = 1;
    let mut line_start: usize = 0;
    let mut colon_index: Option<usize> = None;
    let mut buffer_counter = 0;
    while buffer_counter < buffer.len() {
        let byte = buffer[buffer_counter];
        if byte == seperator && colon_index.is_none() {
            colon_index = Some(buffer_counter);
        }
        if byte == b'\n' {
            if let Some(colon_pos) = colon_index {
                let key_bytes = &buffer[line_start..colon_pos];

                if let Ok(key) = std::str::from_utf8(key_bytes) {
                    if let Some((field_id, enabled)) = config.get(key) {
                        if *enabled {
                            result.insert(line_number, field_id.clone());
                        }
                    }
                }
            }
            line_number = line_number.saturating_add(1);
            line_start = buffer_counter + 1;
            colon_index = None;
        }
        buffer_counter += 1;
    }
    if !config.is_empty() && line_start < buffer.len() {
        if let Some(colon_pos) = colon_index {
            let key_bytes = &buffer[line_start..colon_pos];
            if let Ok(key) = std::str::from_utf8(key_bytes) {
                if let Some((field_id, enabled)) = config.get(key) {
                    if *enabled {
                        result.insert(line_number, field_id.clone());
                    }
                }
            }
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CgroupType {
    V1,
    V2,
}

// TODO: Work around here, you can figure out from the mount path
// that is /proc/mounts search for cgroup2 line it will help
// you determine if the cgroup version is V1 or V2
pub fn cgroup_classifier() -> CgroupType {
    let output = Command::new("stat")
        .arg("-fc")
        .arg("%T")
        .arg(CGROUP_V2_MOUNT_PATH)
        .output()
        .expect("failed to execute cgroup command");
    let result = String::from_utf8_lossy(&output.stdout).to_string();
    if result == "cgroup2fs\n" {
        CgroupType::V2
    } else {
        CgroupType::V1
    }
}
