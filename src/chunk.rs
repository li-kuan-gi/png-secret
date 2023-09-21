use super::chunk_type::ChunkType;
use crate::result::*;

pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let data_length = u32::try_from(data.len()).unwrap();

        let iso_crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut bytes = chunk_type.bytes().to_vec();
        let mut clone_data = data.clone();
        bytes.append(&mut clone_data);
        let crc = iso_crc.checksum(&bytes);

        Self {
            data_length,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn data_length(&self) -> u32 {
        self.data_length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.to_owned())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.data_length
            .to_be_bytes()
            .into_iter()
            .chain(self.chunk_type().bytes())
            .chain(self.data().to_owned())
            .chain(self.crc().to_be_bytes())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let length = bytes.len();

        if length < 12 {
            Err("cannot less than 12 bytes")?
        }

        let data_length: [u8; 4] = bytes[..4].to_owned().try_into().unwrap();
        let data_length = u32::from_be_bytes(data_length);

        let chunk_type: [u8; 4] = bytes[4..8].to_owned().try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type)?;

        if !chunk_type.is_valid() {
            Err("invalid chunk type")?
        }

        let data = bytes[8..length - 4].to_owned();

        if usize::try_from(data_length).unwrap() != data.len() {
            Err("wrong data length")?
        }

        let crc: [u8; 4] = bytes[length - 4..length].to_owned().try_into().unwrap();
        let crc: u32 = u32::from_be_bytes(crc);

        let iso_crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let computed_crc = iso_crc.checksum(&bytes[4..length - 4]);

        if crc != computed_crc {
            Err("crc is wrong")?
        }

        Ok(Self {
            data_length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = if self.data().len() > 5 { "..." } else { "" };
        let data = self
            .data()
            .into_iter()
            .take(5)
            .map(|b| format!("0x{:02x}", b))
            .chain(std::iter::once(suffix.to_string()))
            .collect::<Vec<String>>()
            .join(" ");

        write!(
            f,
            "Chunk:\n\tlength: {}\n\ttype: {}\n\tdata: {} \n\tcrc: {}",
            self.data_length,
            self.chunk_type.to_string(),
            data,
            self.crc
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::chunk_type::ChunkType;
    use crate::result::Result;

    use super::Chunk;

    fn testing_chunk_try_from_with(
        data_length: u32,
        chunk_type: &[u8],
        chunk_data: &[u8],
        crc: u32,
    ) -> Result<Chunk> {
        let chunk_bytes: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(chunk_data.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_bytes.as_ref())
    }

    #[test]
    fn ok_try_fron_valid_bytes() {
        let data_length: u32 = 9;
        let chunk_type = "RuST".as_bytes();
        let chunk_data = "test data".as_bytes();
        let crc: u32 = 3202946391;

        let result = testing_chunk_try_from_with(data_length, chunk_type, chunk_data, crc);

        assert!(result.is_ok());
        let chunk = result.unwrap();

        assert_eq!(chunk.data_length(), 9);
        assert_eq!(chunk.chunk_type().to_string(), "RuST");
        assert_eq!(chunk.data_as_string().unwrap(), "test data");
        assert_eq!(chunk.data, chunk_data);
        assert_eq!(chunk.crc, crc);
    }

    #[test]
    fn err_try_from_less_than_12_bytes() {
        let data_length: u32 = 9;
        let chunk_type = "rus".as_bytes();
        let chunk_data = [];
        let crc: u32 = 1;

        let result = testing_chunk_try_from_with(data_length, chunk_type, &chunk_data, crc);

        assert!(result.is_err());
    }

    #[test]
    fn err_try_from_invalid_chunk_type() {
        let data_length: u32 = 9;
        let chunk_type = "rust".as_bytes();
        let chunk_data = "test data".as_bytes();
        let crc: u32 = 3294024357;

        let result = testing_chunk_try_from_with(data_length, chunk_type, chunk_data, crc);

        assert!(result.is_err());
    }

    #[test]
    fn err_try_from_wrong_crc() {
        let data_length: u32 = 9;
        let chunk_type = "RUST".as_bytes();
        let chunk_data = "test data".as_bytes();
        let crc: u32 = 1;

        let result = testing_chunk_try_from_with(data_length, chunk_type, chunk_data, crc);

        assert!(result.is_err());
    }

    #[test]
    fn err_try_from_wrong_data_length() {
        let data_length: u32 = 1;
        let chunk_type = "RuST".as_bytes();
        let chunk_data = "test data".as_bytes();
        let crc: u32 = 3202946391;

        let result = testing_chunk_try_from_with(data_length, chunk_type, chunk_data, crc);

        assert!(result.is_err());
    }

    #[test]
    fn err_data_as_string_for_invalid_utf_8() {
        let data_length: u32 = 1;
        let chunk_type = "RuST".as_bytes();
        let invalid_utf_8: [u8; 1] = [0x80];
        let chunk_data = &invalid_utf_8[..];
        let crc: u32 = 2240131201;
        let chunk = testing_chunk_try_from_with(data_length, chunk_type, chunk_data, crc).unwrap();

        let result = chunk.data_as_string();

        assert!(result.is_err());
    }

    #[test]
    fn new_chunk() {
        let chunk_type = ChunkType::from_str("RUST").unwrap();
        let chunk_data = "test data".as_bytes().to_owned();

        let chunk: Chunk = Chunk::new(chunk_type, chunk_data);

        assert_eq!(chunk.data_length(), 9);
        assert_eq!(chunk.crc(), 2799226543);
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 9;
        let chunk_type = "RUST".as_bytes();
        let chunk_data = "test data".as_bytes();
        let crc: u32 = 2799226543;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(chunk_data.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
