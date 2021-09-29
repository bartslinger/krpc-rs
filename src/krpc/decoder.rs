use std::convert::TryInto;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_float() {
        let input = vec![31, 20, 21, 69]; // 1F 14 15 45
        let expected = 2385.25756836;
        let decoded = decode_float(input).unwrap();
        assert_eq!(expected, decoded);
    }
    
    #[test]
    fn test_double() {
        let input = vec![247, 208, 219, 17, 159, 66, 66, 65];
        let expected = 2393406.13952076;
        let decoded = decode_double(input).unwrap();
        assert_eq!(expected, decoded);
    }
}

pub fn decode_float(input: Vec<u8>) -> Result<f32, Error> {
    let array: Result<[u8; 4], _> = input.try_into();
    Ok(
        f32::from_le_bytes(
            array.or(Err(Error::InvalidInput))?
        )
    )
}

pub fn decode_double(input: Vec<u8>) -> Result<f64, Error> {
    let array: Result<[u8; 8], _> = input.try_into();
    Ok(
        f64::from_le_bytes(
            array.or(Err(Error::InvalidInput))?
        )
    )
}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    DecodeError(prost::DecodeError),
}

impl From<prost::DecodeError> for Error {
    fn from(e: prost::DecodeError) -> Self {
        Error::DecodeError(e)
    }
}