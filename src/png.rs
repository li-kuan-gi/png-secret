use crate::{chunk::Chunk, result::*};

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|chunk| chunk.chunk_type().to_string() == chunk_type)
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk)
    }

    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        let index = self
            .chunks
            .iter()
            .position(|chunk| chunk.chunk_type().to_string() == chunk_type)
            .ok_or("no chunk with such type")?;

        Ok(self.chunks.remove(index))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let chunks: &Vec<u8> = &self.chunks[..]
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        Self::STANDARD_HEADER
            .iter()
            .chain(chunks)
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let length = bytes.len();

        if length < 8 {
            Err("wrong length for png file")?
        }
        let header: [u8; 8] = bytes[..8].to_vec().try_into().unwrap();
        if header != Self::STANDARD_HEADER {
            Err("wrong header")?
        }

        let mut chunks = Vec::<Chunk>::new();
        let mut next_index = 8;
        while next_index < length {
            if next_index + 4 > length {
                Err("wrong length for png file")?
            }
            let data_length: [u8; 4] = bytes[next_index..next_index + 4]
                .to_vec()
                .try_into()
                .unwrap();
            let data_length = u32::from_be_bytes(data_length);
            let data_length = usize::try_from(data_length)?;

            if next_index + 12 + data_length > length {
                Err("wrong length for png file")?
            }
            let bytes = &bytes[next_index..next_index + 12 + data_length];

            let chunk = Chunk::try_from(bytes)?;
            chunks.push(chunk);

            next_index += 12 + data_length;
        }

        Ok(Self { chunks })
    }
}

impl std::fmt::Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: Vec<String> = self.chunks.iter().map(|chunk| chunk.to_string()).collect();
        write!(f, "{:?}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Result;
    use std::str::FromStr;

    use crate::{chunk::Chunk, chunk_type::ChunkType};

    fn chunk_from_string(chunk_type: &str, data: &str) -> Result<Chunk> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let data = data.into();
        Ok(Chunk::new(chunk_type, data))
    }

    fn testing_chunks() -> Vec<Chunk> {
        let mut chunks = Vec::<Chunk>::new();

        chunks.push(chunk_from_string("FRST", "first chunk").unwrap());
        chunks.push(chunk_from_string("miDl", "middle chunk").unwrap());
        chunks.push(chunk_from_string("IEND", "end chunk").unwrap());

        chunks
    }

    fn testing_png_bytes(header: [u8; 8], chunks: Vec<Chunk>) -> Vec<u8> {
        let chunk_bytes: Vec<u8> = chunks
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        header.iter().chain(chunk_bytes.iter()).copied().collect()
    }

    fn testing_png() -> Png {
        let chunks = testing_chunks();
        Png::from_chunks(chunks)
    }

    #[test]
    fn from_chunks() {
        let chunks = testing_chunks();
        let png = Png::from_chunks(chunks);

        assert_eq!(png.chunks().len(), 3);
    }

    #[test]
    fn ok_try_from_valid_bytes() {
        let bytes = testing_png_bytes(Png::STANDARD_HEADER, testing_chunks());

        let png = Png::try_from(bytes.as_ref());

        assert!(png.is_ok());
    }

    #[test]
    fn err_try_from_invalid_header() {
        let wrong_header = [13, 80, 78, 71, 13, 10, 26, 10];
        let bytes = testing_png_bytes(wrong_header, testing_chunks());

        let png = Png::try_from(bytes.as_ref());

        assert!(png.is_err());
    }

    #[test]
    fn err_try_from_invalid_chunk() {
        let mut chunks = testing_chunks();
        let invalid_chunk = Chunk::new(ChunkType::from_str("RUSt").unwrap(), "test data".into());
        chunks.push(invalid_chunk);
        let bytes = testing_png_bytes(Png::STANDARD_HEADER, chunks);

        let png = Png::try_from(bytes.as_ref());

        assert!(png.is_err());
    }

    #[test]
    fn list_chunks() {
        let png = testing_png();
        let chunks = png.chunks();
        assert_eq!(chunks.len(), 3);
    }

    #[test]
    fn chunk_by_type() {
        let png = testing_png();
        let chunk = png.chunk_by_type("FRST").unwrap();

        assert_eq!(chunk.chunk_type().to_string(), "FRST");
        assert_eq!(chunk.data_as_string().unwrap(), "first chunk");
    }

    #[test]
    fn append_chunk() {
        let mut png = testing_png();
        png.append_chunk(chunk_from_string("TeST", "test data").unwrap());

        let chunk = png.chunk_by_type("TeST").unwrap();

        assert_eq!(chunk.chunk_type().to_string(), "TeST");
        assert_eq!(chunk.data_as_string().unwrap(), "test data");
    }

    #[test]
    fn remove_chunk() {
        let mut png = testing_png();
        png.append_chunk(chunk_from_string("TeST", "test data").unwrap());
        png.remove_chunk("TeST").unwrap();

        let chunk = png.chunk_by_type("TeST");

        assert!(chunk.is_none());
    }

    #[test]
    fn png_fromimage_file() {
        let png = Png::try_from(&PNG_FILE[..]);
        assert!(png.is_ok());
    }

    #[test]
    fn as_bytes() {
        let png = Png::try_from(&PNG_FILE[..]).unwrap();
        let actual = png.as_bytes();
        let expcted = PNG_FILE.to_vec();
        assert_eq!(actual, expcted);
    }

    #[test]
    fn test_png_trait_impls() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png: Png = TryFrom::try_from(bytes.as_ref()).unwrap();

        let _png_string = format!("{}", png);
    }

    const PNG_FILE: [u8; 88] = [
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x02, 0x08, 0x02, 0x00, 0x00, 0x00, 0x12,
        0x16, 0xf1, 0x4d, 0x00, 0x00, 0x00, 0x1f, 0x49, 0x44, 0x41, 0x54, 0x08, 0x1d, 0x01, 0x14,
        0x00, 0xeb, 0xff, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00,
        0x00, 0x00, 0x80, 0x80, 0x80, 0xff, 0xff, 0xff, 0x3a, 0x61, 0x07, 0x7b, 0xcb, 0xca, 0x5c,
        0x63, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ];
}
