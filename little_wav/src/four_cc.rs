#[derive(Default, Debug)]
pub struct FourCC {
    value: [u8; 4],
}

impl FourCC {
    pub fn new(value: [u8; 4]) -> Result<Self, ()> {
        Ok(Self { value })
    }
    pub fn from(string: &str) -> Result<Self, &str> {
        if string.len() != 4 {
            return Err("FourCC must be 4 characters long");
        }

        let bytes = string.as_bytes();

        let tuple: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];

        Ok(Self { value: tuple })
    }
    pub fn as_bytes(&self) -> [u8; 4] {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fourcc() {
        let fourcc = FourCC::from("data").unwrap();
        assert_eq!(fourcc.as_bytes(), [b'd', b'a', b't', b'a']);
    }
    #[test]
    fn test_fourcc_too_long() {
        let fourcc = FourCC::from("dataa");
        assert!(fourcc.is_err());
    }
}
