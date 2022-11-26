use crate::{
    chunk::Chunk,
    codable::{Decodable, Encodable},
    four_cc::FourCC,
};
use std::io::Read;

const DATA_TYPE: &[u8; 4] = b"data";

#[derive(Default, Debug)]
pub struct HeaderData {
    size: u32,
}

impl HeaderData {
    pub fn new() -> Self {
        Self { size: 0 }
    }

    pub fn set_size(&mut self, samples_len: usize, sample_size: usize) {
        self.size = sample_size as u32 * samples_len as u32;
    }
}

impl Chunk for HeaderData {
    fn get_be_id(&self) -> &[u8; 4] {
        DATA_TYPE
    }

    fn get_le_size(&self) -> u32 {
        self.size
    }
}

impl Encodable for HeaderData {
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(self.get_be_id());
        vec.extend_from_slice(&self.size.to_le_bytes());

        vec
    }
}

impl Decodable for HeaderData {
    fn decode_new<R: Read>(mut reader: R) -> Self {
        let mut buffer_16 = [0; 4];

        reader.read_exact(&mut buffer_16).unwrap();
        let _id = FourCC::new(buffer_16).unwrap();

        reader.read_exact(&mut buffer_16).unwrap();
        let size = u32::from_le_bytes(buffer_16);

        Self { size }
    }
}
