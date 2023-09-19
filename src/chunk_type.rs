#[derive(Debug, PartialEq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_letter(b: u8) -> bool {
        (b > 65 && b < 90) || (b > 97 && b < 122)
    }

    pub fn is_critical(&self) -> bool {
        let first = self.bytes[0];
        first >> 5 & 1 == 0
    }

    pub fn is_public(&self) -> bool {
        let second = self.bytes[1];
        second >> 5 & 1 == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let third = self.bytes[2];
        third >> 5 & 1 == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let fourth = self.bytes[3];
        fourth >> 5 & 1 == 1
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && !(self.is_critical() && self.is_safe_to_copy())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        if bytes.into_iter().all(ChunkType::is_letter) {
            Ok(Self { bytes })
        } else {
            Err("should be all letters !!")
        }
    }
}

impl std::str::FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.as_bytes().len();

        if length == 4 && s.bytes().all(ChunkType::is_letter) {
            let bytes = s.as_bytes().to_owned().try_into().unwrap();
            Ok(Self { bytes })
        } else {
            Err("Should consist of 4 letters !!")
        }
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.bytes).unwrap();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn ok_from_bytes_of_all_letter() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from(expected).unwrap();

        assert_eq!(expected, actual.bytes());
        assert_eq!(expected, actual.bytes());
    }

    #[test]
    fn err_from_bytes_of_some_non_letter() {
        let invalid_bytes = [0, 117, 83, 116];
        let result = ChunkType::try_from(invalid_bytes);

        assert!(result.is_err());
    }

    #[test]
    fn ok_from_str_with_all_letters() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn err_from_str_with_some_non_letter() {
        let invalid_str = "R2st";
        let result = ChunkType::from_str(invalid_str);

        assert!(result.is_err());
    }

    #[test]
    fn critical_case() {
        let chunk = ChunkType::from_str("Rust").unwrap();

        assert!(chunk.is_critical());
    }

    #[test]
    fn ancillary_case() {
        let chunk = ChunkType::from_str("rust").unwrap();

        assert!(!chunk.is_critical());
    }

    #[test]
    fn public_case() {
        let chunk = ChunkType::from_str("rUst").unwrap();

        assert!(chunk.is_public());
    }

    #[test]
    fn private_case() {
        let chunk = ChunkType::from_str("rust").unwrap();

        assert!(!chunk.is_public());
    }

    #[test]
    fn valid_reserved_bit_case() {
        let chunk = ChunkType::from_str("ruSt").unwrap();

        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    fn invalid_reserved_bit_case() {
        let chunk = ChunkType::from_str("rust").unwrap();

        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    fn safe_to_copy_case() {
        let chunk = ChunkType::from_str("rust").unwrap();

        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    fn unsafe_to_copy_case() {
        let chunk = ChunkType::from_str("rusT").unwrap();

        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    fn valid_case() {
        let chunk1 = ChunkType::from_str("RuST").unwrap();
        let chunk2 = ChunkType::from_str("ruSt").unwrap();

        assert!(chunk1.is_valid());
        assert!(chunk2.is_valid());
    }

    #[test]
    fn invalid_if_reserved_bit_is_invalid() {
        let chunk = ChunkType::from_str("rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());

        assert!(!chunk.is_valid());
    }

    #[test]
    fn invalid_if_critical_but_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
        assert!(chunk.is_safe_to_copy());

        assert!(!chunk.is_valid());
    }

    #[test]
    fn test_to_string() {
        let s = "rust";
        let chunk = ChunkType::from_str(s).unwrap();

        assert_eq!(chunk.to_string(), s);
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
