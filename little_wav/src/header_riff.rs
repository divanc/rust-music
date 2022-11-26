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
    pub fn new() -> Self {
        Self { size: 0 }
    }
    pub fn set_size(&mut self, header_size: usize, samples_len: usize, sample_size: usize) {
        self.size = (header_size + sample_size * samples_len) as u32;
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
    fn decode_new<R: Read>(mut reader: R) -> Self {
        let mut buffer_16 = [0; 4];

        reader.read_exact(&mut buffer_16).unwrap();
        if buffer_16 != *RIFF_ID {
            panic!("RIFF ID not found");
        }

        reader.read_exact(&mut buffer_16).unwrap();
        let size = u32::from_le_bytes(buffer_16);

        reader.read_exact(&mut buffer_16).unwrap();
        if buffer_16 != *RIFF_TYPE {
            panic!("RIFF Type not found");
        }

        Self { size }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        let encoded = [82, 73, 70, 70, 2, 0, 0, 0, 87, 65, 86, 69];
        let header = HeaderRiff::decode_new(&encoded[..]);

        assert_eq!(header.size, 2);
    }
}
