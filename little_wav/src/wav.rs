use crate::{
    codable::{Decodable, Encodable},
    header_data::HeaderData,
    header_format::{Encoding, HeaderFormat},
    header_riff::HeaderRiff,
};
use std::io::Read;
use std::{fs::File, io::Write, mem, slice};

const HEADER_SIZE: usize = 36;
const SAMPLE_MAX: usize = 32_767;

// #[derive(Debug)]
#[repr(C)]
pub struct Wav<T: Copy = i16> {
    riff: HeaderRiff,
    format: HeaderFormat,
    data: HeaderData,
    body: Vec<T>,
}

impl<T: Copy> Wav<T> {
    pub fn new(encoding: Encoding, channels: usize, sample_rate: usize) -> Self {
        let sample_size = mem::size_of::<T>(); // method not yet accessible
        let fmt = HeaderFormat::new(encoding, channels, sample_rate, sample_size);

        Self {
            riff: HeaderRiff::new(),
            format: fmt,
            data: HeaderData::new(),
            body: Vec::new(),
        }
    }

    pub fn set_channels(&mut self, channels: u16) -> &Self {
        self.format.channels(channels);
        self
    }
    pub fn set_encoding(&mut self, encoding: Encoding) -> &Self {
        self.format.encoding(encoding);
        self
    }
    pub fn set_sample_rate(&mut self, sample_rate: u32) -> &Self {
        self.format.sample_rate(sample_rate);
        self
    }

    // Appends audio data to the end of the wav & updates the header
    pub fn push_body(&mut self, chunk: Vec<T>) -> &Self {
        self.body.extend(chunk);

        // Update fields dependent on body size
        let ssize = self.get_sample_size();
        let samples = self.body.len();

        // RIFF
        self.riff.set_size(self.get_header_size(), samples, ssize);
        // DATA
        self.data.set_size(samples, ssize);

        self
    }

    fn get_sample_size(&self) -> usize {
        mem::size_of::<T>()
    }
    fn get_header_size(&self) -> usize {
        // No need to use mem::size_of() here, since we know the size of the struct
        HEADER_SIZE
    }

    // read_new reads a wav file to a new instance of structure
    pub fn read_new(path: &str) -> Result<Self, &str> {
        let maybe_file = File::open(path);
        if maybe_file.is_err() {
            return Err("File not found");
        }

        let mut file = maybe_file.unwrap();

        let wav = Self::decode_new(&mut file);

        Ok(wav)
    }

    pub fn write_to_file(&mut self, path: &str) -> Result<(), &str> {
        let mut file = File::create(path).unwrap();

        let buffer_vec = self.encode();
        let buffer = buffer_vec.as_slice();

        file.write_all(buffer).unwrap();

        Ok(())
    }
}

impl<T: Copy> Encodable for Wav<T> {
    fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&self.riff.encode());
        vec.extend_from_slice(&self.format.encode());
        vec.extend_from_slice(&self.data.encode());

        let body = unsafe {
            slice::from_raw_parts(
                self.body.as_ptr() as *const u8,
                self.body.len() * self.get_sample_size(),
            )
        };
        vec.extend_from_slice(body);

        vec
    }
}

impl<T: Copy> Decodable for Wav<T> {
    fn decode_new<R: Read>(mut reader: R) -> Self {
        let mut wav = Wav {
            riff: HeaderRiff::decode_new(&mut reader),
            format: HeaderFormat::decode_new(&mut reader),
            data: HeaderData::decode_new(&mut reader),
            body: Vec::<T>::new(),
        };

        // Set sample size for format afterwards to not spoil trait decode interface :(
        wav.format.sample_size(wav.get_sample_size());

        let mut body_buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut body_buffer).unwrap();

        let body = unsafe {
            slice::from_raw_parts(
                body_buffer.as_ptr() as *const T,
                body_buffer.len() / wav.get_sample_size(),
            )
        };
        wav.body = body.to_vec();

        wav
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_size() {
        let riff_size = HeaderRiff::default().encode().len();
        let format_size = HeaderFormat::default().encode().len();
        let data_size = HeaderData::default().encode().len();

        println!("riff_size: {}", riff_size);
        println!("format_size: {}", format_size);
        println!("data_size: {}", data_size);

        assert_eq!(riff_size + format_size + data_size, HEADER_SIZE + 8);
    }

    #[test]
    fn compare_encoding() {
        const FILE_PATH: &str = "test_assets/header_only.wav";

        let wav = Wav::<i16>::read_new(FILE_PATH).unwrap();
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

        let mut wav = Wav::<i16>::read_new(FILE_PATH).expect("File load error");
        wav.write_to_file(FILE_PATH_OUT).expect("Write err")
    }

    #[test]
    fn do_sine() {
        const FILE_PATH_OUT: &str = "test_assets/_sine_out_2.wav";

        let sr = 44_100;
        let ch = 1;
        let encoding = Encoding::LPCM;

        let duration = 5;
        let num_samples = sr * duration * ch;

        let mut wav = Wav::<i16>::new(encoding, ch, sr);

        let mut samples = Vec::<i16>::new();
        for i in 0..num_samples {
            let t = i as f64 / sr as f64;
            let sample = (t * 440.0 * 2.0 * std::f64::consts::PI).sin() * SAMPLE_MAX as f64;
            samples.push(sample as i16);
        }

        wav.push_body(samples);
        wav.write_to_file(FILE_PATH_OUT).expect("Write err")
    }

    #[test]
    fn do_melody() {
        const FILE_PATH_OUT: &str = "test_assets/_melody_out.wav";

        let sr = 8_000;
        let ch = 1;
        let encoding = Encoding::LPCM;

        let duration = 60;
        let num_samples = sr * duration * ch;

        let mut wav = Wav::<u8>::new(encoding, ch, sr);

        let mut samples = Vec::<u8>::new();
        for i in 0..num_samples {
            // let sample = i * 5 & i >> 7 | i * 3 & i >> 10;
            let sample = i * 1 & i >> 7 | i * 3 & i >> 10;
            samples.push(sample as u8);
        }

        wav.push_body(samples);
        wav.write_to_file(FILE_PATH_OUT).expect("Write err")
    }
}
