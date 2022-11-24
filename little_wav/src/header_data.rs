use crate::{
    codable::{Decodable, Encodable},
    four_cc::FourCC,
};
use std::io::Read;

#[derive(Default, Debug)]
pub struct HeaderData {
    id: FourCC,
    size: u32,
}

impl Encodable for HeaderData {
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&self.id.as_bytes());
        vec.extend_from_slice(&self.size.to_le_bytes());

        vec
    }
}

impl Decodable for HeaderData {
    fn decode<R: Read>(&mut self, mut reader: R) -> Result<(), &str> {
        let mut buffer_16 = [0; 4];

        reader.read_exact(&mut buffer_16).unwrap();
        self.id = FourCC::new(buffer_16).unwrap();

        reader.read_exact(&mut buffer_16).unwrap();
        self.size = u32::from_le_bytes(buffer_16);

        Ok(())
    }
}
