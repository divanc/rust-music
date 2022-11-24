use std::env;
use std::fs;
use std::io::{Error, Read};

#[derive(Debug, Default)]
pub struct Data {
    pub id: i32,
    pub size: i32,
}

impl Data {
    // fn load2(mut reader: impl Read) -> Result<Self, Error> {
    // let mut data = Data::default();

    // unsafe {
    //     let buffer: &mut [u8] = std::slice::from_raw_parts_mut(
    //         data.as_mut_ptr().cast(),
    //         std::mem::size_of::<Self>(),
    //     );

    //     reader.read_exact(buffer)?;
    //     Ok(data)
    // }
    // }
    pub fn read_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut data = Data::default();

        let mut buffer = [0; 4];
        buffer.copy_from_slice(&bytes[0..4]);
        data.id = i32::from_le_bytes(buffer);

        buffer.copy_from_slice(&bytes[8..12]);
        data.size = i32::from_le_bytes(buffer);

        Ok(data)
    }
    pub fn load<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let mut id = [0; 4];
        reader.read_exact(&mut id)?;

        let mut size = [0; 4];
        reader.read_exact(&mut size)?;

        Ok(Self {
            id: i32::from_le_bytes(id),
            size: i32::from_le_bytes(size),
        })
    }
}

const FILE_PATH: &str = "test.dat";

fn main() {
    println!("Hello, world!");
    let mut file = fs::File::open(FILE_PATH).expect("file");

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("read");

    let data = Data::read_bytes(buf.as_slice());

    // let contents = fs::read(FILE_PATH).expect("Should have been able to read the file");
    println!("With text:\n{data:?}");
}
