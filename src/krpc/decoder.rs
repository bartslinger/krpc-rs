use std::convert::TryInto;
use super::schema;
use prost::Message;

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
    
    #[test]
    fn test_uint32() {
        let input = vec![0xFA, 0xBC, 0x7E];
        let expected = 2072186;
        let decoded = decode_uint32(input).unwrap();
        assert_eq!(expected, decoded);
    }
}

pub fn decode_float(input: Vec<u8>) -> Result<f32, Error> {
    let array: Result<[u8; 4], _> = input.try_into();
    Ok(f32::from_le_bytes(array.or(Err(Error::InvalidInput))?))
}

pub fn decode_double(input: Vec<u8>) -> Result<f64, Error> {
    let array: Result<[u8; 8], _> = input.try_into();
    Ok(f64::from_le_bytes(array.or(Err(Error::InvalidInput))?))
}

#[allow(dead_code)]
pub fn decode_uint32(input: Vec<u8>) -> Result<u32, Error> {
    let res = prost::encoding::decode_varint(&mut &*input)?;
    let output = res as u32;
    Ok(output)
}

pub fn decode_class(input: Vec<u8>) -> Result<u64, Error> {
    let id = prost::encoding::decode_varint(&mut input.as_slice())?;
    Ok(id)
}

pub fn decode_tuple_2(input: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let tuple = schema::Tuple::decode(input.as_slice())?;
    if tuple.items.len() != 2 {
        return Err(Error::InvalidInput);
    }
    let first = tuple.items.get(0).ok_or(Error::InvalidInput)?.clone();
    let second = tuple.items.get(1).ok_or(Error::InvalidInput)?.clone();
    Ok((first, second))
}

pub fn decode_tuple_3(input: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Error> {
    let tuple = schema::Tuple::decode(input.as_slice())?;
    if tuple.items.len() != 3 {
        return Err(Error::InvalidInput);
    }
    let first = tuple.items.get(0).ok_or(Error::InvalidInput)?.clone();
    let second = tuple.items.get(1).ok_or(Error::InvalidInput)?.clone();
    let third = tuple.items.get(1).ok_or(Error::InvalidInput)?.clone();
    Ok((first, second, third))
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
