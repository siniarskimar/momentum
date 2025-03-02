use std::fmt::Write;

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexDecodeError {
    InvalidCharacter { c: char, index: usize },
    InsufficientBufferSize,
}

impl std::error::Error for HexDecodeError {}

impl std::fmt::Display for HexDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            HexDecodeError::InvalidCharacter { c, index } => {
                write!(f, "Invalid character {:?} at position {}", c, index)
            }
            HexDecodeError::InsufficientBufferSize => write!(f, "Insufficient buffer size"),
        }
    }
}

pub fn to_str<T: AsRef<[u8]>>(bytes: T) -> String {
    let bytes = bytes.as_ref();
    let mut result = String::with_capacity(bytes.len() * 4);

    for byte in bytes.iter() {
        let (high, low) = encode_byte(*byte);
        result.push(high);
        result.push(low);
    }

    return result;
}

pub fn encode_byte(byte: u8) -> (char, char) {
    let high: usize = (byte >> 4).into();
    let low: usize = (byte & 0xf).into();

    return (HEX_CHARS[high] as char, HEX_CHARS[low] as char);
}

pub fn decode_byte(buffer: [char; 2]) -> Result<u8, HexDecodeError> {
    let low = decode_bytepart(buffer[1]);
    let high = decode_bytepart(buffer[0]);

    if low.is_none() {
        return Err(HexDecodeError::InvalidCharacter {
            c: buffer[1],
            index: 1,
        });
    }
    if high.is_none() {
        return Err(HexDecodeError::InvalidCharacter {
            c: buffer[0],
            index: 0,
        });
    }

    let low = low.unwrap();
    let high = high.unwrap();

    return Ok(high << 4 | low);
}

pub fn decode_bytepart(c: char) -> Option<u8> {
    let val = c as u8;
    if c >= 'a' && c <= 'f' {
        return Some(val - ('a' as u8) + 10);
    }
    if c >= 'A' && c <= 'F' {
        return Some(val - ('A' as u8) + 10);
    }
    if c >= '0' && c <= '9' {
        return Some(val - ('0' as u8));
    }

    return None;
}

pub fn from_chars<T: AsRef<[u8]>>(string: T, bytes: &mut [u8]) -> Result<(), HexDecodeError> {
    let string = string.as_ref();
    let mut buffer: [char; 2] = ['0'; 2];

    for (idx, c) in string.iter().rev().enumerate() {
        buffer[1 - idx % 2] = *c as char;

        if idx % 2 == 1 || (idx == string.len() - 1 && idx % 2 == 0) {
            let dest_idx = idx / 2;
            if dest_idx > bytes.len() - 1 {
                return Err(HexDecodeError::InsufficientBufferSize);
            }

            bytes[bytes.len() - 1 - idx / 2] = match decode_byte(buffer) {
                Ok(byte) => byte,
                Err(err) => match err {
                    HexDecodeError::InvalidCharacter {
                        c,
                        index: inner_idx,
                    } => {
                        return Err(HexDecodeError::InvalidCharacter {
                            c,
                            index: idx + inner_idx,
                        })
                    }
                    _ => return Err(err),
                },
            };
            buffer[0] = '0';
            buffer[1] = '0';
        }
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_chars() {
        let mut bytes: [u8; 2] = [0; 2];
        let expected: [u8; 2] = [10, 11];

        assert_eq!(from_chars("0A0B", &mut bytes), Ok(()));
        assert_eq!(expected, bytes);
    }

    #[test]
    fn test_from_chars_odd() {
        let mut bytes: [u8; 2] = [0; 2];
        let expected: [u8; 2] = [10, 11];

        assert_eq!(from_chars("A0B", &mut bytes), Ok(()));
        assert_eq!(expected, bytes);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(to_str([(0xA << 4) | 0xB]), "ab");
    }
}
