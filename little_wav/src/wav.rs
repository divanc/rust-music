use crate::{
    codable::{Decodable, Encodable},
    header_data::HeaderData,
    header_format::HeaderFormat,
    header_riff::HeaderRiff,
};
use std::io::Read;
use std::{fs::File, io::Write, mem, slice};

#[derive(Default, Debug)]
#[repr(C)]
pub struct Wav {
    riff: HeaderRiff,
    format: HeaderFormat,
    data: HeaderData,
    body: Vec<i16>,
}

impl Wav {
    pub fn push_chunk(mut self, chunk: Vec<i16>) -> Self {
        self.body.extend(chunk);
        self
    }

    pub fn new() -> Self {
        Self::default()
    }
    // pub fn set_riff_size(mut self, size: u32) -> Self {
    //     self.header.riff = self.header.riff.set_size(size);
    //     self
    // }

    // from_file loads a wav file to a new instance of structure
    pub fn from_file(path: &str) -> Result<Self, &str> {
        let mut wav = Self::default();

        let maybe_file = File::open(path);
        if maybe_file.is_err() {
            return Err("File not found");
        }
        let mut file = maybe_file.unwrap();

        wav.decode(&mut file).unwrap();

        Ok(wav)
    }
    pub fn write_to_file(&self, path: &str) -> Result<(), &str> {
        let mut file = File::create(path).unwrap();

        file.write_all(&self.encode()).unwrap();

        Ok(())
    }
}

impl Encodable for Wav {
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&self.riff.encode());
        vec.extend_from_slice(&self.format.encode());
        vec.extend_from_slice(&self.data.encode());

        let body = unsafe {
            slice::from_raw_parts(
                self.body.as_ptr() as *const u8,
                self.body.len() * mem::size_of::<i16>(),
            )
        };
        vec.extend_from_slice(body);

        vec
    }
}

impl Decodable for Wav {
    fn decode<R: Read>(&mut self, mut reader: R) -> Result<(), &str> {
        self.riff.decode(&mut reader)?;
        self.format.decode(&mut reader)?;
        self.data.decode(&mut reader)?;

        let mut body_buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut body_buffer).unwrap();

        let body = unsafe {
            slice::from_raw_parts(
                body_buffer.as_ptr() as *const i16,
                body_buffer.len() / mem::size_of::<i16>(),
            )
        };
        self.body = body.to_vec();

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare_encoding() {
        const FILE_PATH: &str = "test_assets/header_only.wav";

        let wav = Wav::from_file(FILE_PATH).unwrap();
        let code_buffer = wav.encode();

        let mut file = File::open(FILE_PATH).unwrap();
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer).unwrap();

        assert_eq!(code_buffer, file_buffer);
    }

    #[test]
    fn rewrite_compare() {
        const FILE_PATH: &str = "test_assets/sine.wav";
        const FILE_PATH_OUT: &str = "test_assets/_sine_out.wav";

        let wav = Wav::from_file(FILE_PATH).expect("File load error");
        wav.write_to_file(FILE_PATH_OUT).expect("Write err")
    }
}
