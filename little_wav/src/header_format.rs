use crate::{
    chunk::Chunk,
    codable::{Decodable, Encodable},
    four_cc::FourCC,
};
use std::io::Read;

const FMT_TYPE: &[u8; 4] = b"fmt ";

#[derive(Clone, Copy, Default, Debug)]
pub enum Encoding {
    #[default]
    LPCM = 1,
    IEEE = 3,
    ALAW = 6,
    MULAW = 7,
}

impl Encoding {
    fn as_le_bytes(self) -> [u8; 2] {
        match self {
            Encoding::LPCM => [1, 0],
            Encoding::IEEE => [3, 0],
            Encoding::ALAW => [6, 0],
            Encoding::MULAW => [7, 0],
        }
    }

    fn from_le_bytes(bytes: [u8; 2]) -> Self {
        match bytes {
            [1, 0] => Encoding::LPCM,
            [3, 0] => Encoding::IEEE,
            [6, 0] => Encoding::ALAW,
            [7, 0] => Encoding::MULAW,
            _ => Encoding::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct HeaderFormat {
    tag: Encoding,
    channels: u16,
    sample_rate: u32,

    sample_size: u16,
}

impl HeaderFormat {
    pub fn new(
        encoding: Encoding,
        channels: usize,
        sample_rate: usize,
        sample_size: usize,
    ) -> Self {
        Self {
            tag: encoding,
            channels: channels as u16,
            sample_rate: sample_rate as u32,
            sample_size: sample_size as u16,
        }
    }
    pub fn encoding(&mut self, encoding: Encoding) -> &Self {
        self.tag = encoding;
        self
    }
    pub fn channels(&mut self, channels: u16) -> &Self {
        self.channels = channels;
        self
    }
    pub fn sample_rate(&mut self, sample_rate: u32) -> &Self {
        self.sample_rate = sample_rate;
        self
    }
    pub fn sample_size(&mut self, sample_size: usize) -> &Self {
        self.sample_size = sample_size as u16;
        self
    }

    pub fn get_byte_rate(&self) -> u32 {
        (self.channels as u32) * (self.sample_rate as u32) * (self.sample_size as u32)
    }
    pub fn get_block_align(&self) -> u16 {
        (self.channels as u16) * (self.sample_size as u16)
    }
    pub fn get_bits_per_sample(&self) -> u16 {
        (self.sample_size as u16) * 8
    }
}

impl Chunk for HeaderFormat {
    fn get_be_id(&self) -> &[u8; 4] {
        FMT_TYPE
    }

    fn get_le_size(&self) -> u32 {
        16
    }
}

impl Encodable for HeaderFormat {
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(self.get_be_id());
        vec.extend_from_slice(&self.get_le_size().to_le_bytes());
        vec.extend_from_slice(&self.tag.as_le_bytes());
        vec.extend_from_slice(&self.channels.to_le_bytes());
        vec.extend_from_slice(&self.sample_rate.to_le_bytes());
        vec.extend_from_slice(&self.get_byte_rate().to_le_bytes());
        vec.extend_from_slice(&self.get_block_align().to_le_bytes());
        vec.extend_from_slice(&self.get_bits_per_sample().to_le_bytes());

        vec
    }
}

impl Decodable for HeaderFormat {
    fn decode_new<R: Read>(mut reader: R) -> Self {
        let mut buffer_16 = [0; 4];
        let mut buffer_8 = [0; 2];

        reader.read_exact(&mut buffer_16).unwrap();
        let _id = FourCC::new(buffer_16).unwrap();

        reader.read_exact(&mut buffer_16).unwrap();
        let _size = u32::from_le_bytes(buffer_16);

        reader.read_exact(&mut buffer_8).unwrap();
        let tag = Encoding::from_le_bytes(buffer_8);

        reader.read_exact(&mut buffer_8).unwrap();
        let channels = u16::from_le_bytes(buffer_8);

        reader.read_exact(&mut buffer_16).unwrap();
        let sample_rate = u32::from_le_bytes(buffer_16);

        reader.read_exact(&mut buffer_16).unwrap();
        let _byte_rate = u32::from_le_bytes(buffer_16);

        reader.read_exact(&mut buffer_8).unwrap();
        let _block_align = u16::from_le_bytes(buffer_8);

        reader.read_exact(&mut buffer_8).unwrap();
        let _bits_per_sample = u16::from_le_bytes(buffer_8);

        Self {
            tag,
            channels,
            sample_rate,
            sample_size: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_header_format() {
        let header = HeaderFormat::new(super::Encoding::LPCM, 2, 44100, 2);
        let encoded = header.encode();
        let mut decoded = HeaderFormat::decode_new(&encoded[..]);
        decoded.sample_size(header.sample_size as usize);

        let reincoded = decoded.encode();

        println!("{:?}", header);
        println!("{:?}", decoded);
        assert_eq!(encoded, reincoded);
    }
}
