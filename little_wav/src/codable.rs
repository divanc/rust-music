use std::io::Read;

pub trait Decodable {
    fn decode_new<R: Read>(mutreader: R) -> Self;
}

pub trait Encodable {
    fn encode(&self) -> Vec<u8>;
}

pub trait Codable: Decodable + Encodable {}
