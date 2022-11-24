use crate::{
    chunk::Chunk,
    codable::{Decodable, Encodable},
};
use std::io::Read;

const RIFF_ID: &[u8; 4] = b"RIFF";
const RIFF_TYPE: &[u8; 4] = b"WAVE";

// RIFF Header struct is
#[derive(Default, Debug)]
pub struct HeaderRiff {
    size: u32,
}

impl Chunk for HeaderRiff {
    fn get_be_id(&self) -> &[u8; 4] {
        RIFF_ID
    }

    fn get_le_size(&self) -> u32 {
        self.size
    }
}

impl HeaderRiff {
    pub fn new(size: u32) -> Self {
        Self { size }
    }
    pub fn set_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    // returns a static type for RIFF chunk
    pub fn get_be_type(&self) -> &[u8; 4] {
        RIFF_TYPE
    }
}

impl Encodable for HeaderRiff {
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(self.get_be_id());
        vec.extend_from_slice(&self.size.to_le_bytes());
        vec.extend_from_slice(self.get_be_type());

        vec
    }
}

impl Decodable for HeaderRiff {
    fn decode<R: Read>(&mut self, mut reader: R) -> Result<(), &str> {
        let mut buffer_16 = [0; 4];

        reader.read_exact(&mut buffer_16).unwrap();
        if buffer_16 != *RIFF_ID {
            return Err("RIFF ID not found");
        }

        reader.read_exact(&mut buffer_16).unwrap();
        self.size = u32::from_le_bytes(buffer_16);

        reader.read_exact(&mut buffer_16).unwrap();
        if buffer_16 != *RIFF_TYPE {
            return Err("RIFF Type not found");
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        let header = HeaderRiff::new(0);
        let encoded = header.encode();
        assert_eq!(encoded, [82, 73, 70, 70, 0, 0, 0, 0, 87, 65, 86, 69]);
    }

    #[test]
    fn test_decode() {
        let mut header = HeaderRiff::default();
        let encoded = [82, 73, 70, 70, 2, 0, 0, 0, 87, 65, 86, 69];
        header.decode(&encoded[..]).unwrap();

        assert_eq!(header.size, 2);
    }
}
