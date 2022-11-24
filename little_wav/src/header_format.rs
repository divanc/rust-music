use crate::{
    codable::{Decodable, Encodable},
    four_cc::FourCC,
};
use std::io::Read;

enum Encoding {
    LPCM = 1,
    IEEE = 3,
    ALAW = 6,
    MULAW = 7,
}

impl Encoding {
    fn default() -> Self {
        Encoding::LPCM
    }
}

pub struct HeaderFormat {
    id: FourCC,
    size: u32,

    tag: Encoding,
    channels: u16,
    samples_per_sec: u32,
    bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
}

impl Encodable for HeaderFormat {
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&self.id.as_bytes());
        vec.extend_from_slice(&self.size.to_le_bytes());
        vec.extend_from_slice(&self.tag.to_le_bytes());
        vec.extend_from_slice(&self.channels.to_le_bytes());
        vec.extend_from_slice(&self.samples_per_sec.to_le_bytes());
        vec.extend_from_slice(&self.bytes_per_sec.to_le_bytes());
        vec.extend_from_slice(&self.block_align.to_le_bytes());
        vec.extend_from_slice(&self.bits_per_sample.to_le_bytes());

        vec
    }
}

impl Decodable for HeaderFormat {
    fn decode<R: Read>(&mut self, mut reader: R) -> Result<(), &str> {
        let mut buffer_16 = [0; 4];
        let mut buffer_8 = [0; 2];

        reader.read_exact(&mut buffer_16).unwrap();
        self.id = FourCC::new(buffer_16).unwrap();

        reader.read_exact(&mut buffer_16).unwrap();
        self.size = u32::from_le_bytes(buffer_16);

        reader.read_exact(&mut buffer_8).unwrap();
        self.tag = u16::from_le_bytes(buffer_8);

        reader.read_exact(&mut buffer_8).unwrap();
        self.channels = u16::from_le_bytes(buffer_8);

        reader.read_exact(&mut buffer_16).unwrap();
        self.samples_per_sec = u32::from_le_bytes(buffer_16);

        reader.read_exact(&mut buffer_16).unwrap();
        self.bytes_per_sec = u32::from_le_bytes(buffer_16);

        reader.read_exact(&mut buffer_8).unwrap();
        self.block_align = u16::from_le_bytes(buffer_8);

        reader.read_exact(&mut buffer_8).unwrap();
        self.bits_per_sample = u16::from_le_bytes(buffer_8);

        Ok(())
    }
}
