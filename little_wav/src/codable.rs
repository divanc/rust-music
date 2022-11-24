use std::io::Read;

pub trait Decodable {
    fn decode<R: Read>(&mut self, mutreader: R) -> Result<(), &str>;
}

pub trait Encodable {
    fn encode(&self) -> Vec<u8>;
}

pub trait Codable: Decodable + Encodable {}
