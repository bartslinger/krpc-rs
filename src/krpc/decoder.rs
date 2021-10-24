use std::convert::TryInto;
use super::schema;
use prost::Message;

pub trait Decode<'a> {
    fn decode(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<Self, Error> where Self: Sized;
}

impl<'a> Decode<'a> for String {
    fn decode(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<String, Error> {
        let mut slice = input.as_slice();
        let len = prost::encoding::decode_varint(&mut slice);
        Ok(std::str::from_utf8(slice).map_err(|_e| Error::StringDecodeError)?.to_string())
    }
}

impl<'a> Decode<'a> for () {
    fn decode(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<(), Error> {
        Ok(())
    }
}

pub fn decode_none<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<(), Error> {
    Ok(())
}

pub fn decode_bool<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<bool, Error> {
    Ok(true)
}

pub fn decode_float<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<f32, Error> {
    let array: Result<[u8; 4], _> = input.try_into();
    Ok(f32::from_le_bytes(array.or(Err(Error::InvalidInput))?))
}

pub fn decode_double<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<f64, Error> {
    let array: Result<[u8; 8], _> = input.try_into();
    Ok(f64::from_le_bytes(array.or(Err(Error::InvalidInput))?))
}

pub fn decode_uint32<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<u32, Error> {
    let res = prost::encoding::decode_varint(&mut &*input)?;
    let output = res as u32;
    Ok(output)
}

pub fn decode_class<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<u64, Error> {
    let id = prost::encoding::decode_varint(&mut input.as_slice())?;
    Ok(id)
}

pub fn decode_string<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<String, Error> {
    println!("String input buffer: {:?}", input);
    let mut slice = input.as_slice();
    let len = prost::encoding::decode_varint(&mut slice);
    Ok(std::str::from_utf8(slice).map_err(|_e| Error::StringDecodeError)?.to_string())
}

pub fn decode_sint32<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<i32, Error> {
    Ok(0)
}

pub fn decode_list<'a, T: Decode<'a> + std::fmt::Debug>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<Vec<T>, Error> {
    let list = schema::List::decode(input.as_slice())?;
    for item in list.items {
        let test = T::decode(item, conn)?;
        println!("{:?}", test);
    }
    Ok(Vec::new())
}

pub fn decode_dictionary<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<(), Error> {
    Ok(())
}

pub fn decode_enumeration<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<(), Error> {
    Ok(())
}

pub fn decode_set<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<(), Error> {
    Ok(())
}
pub fn decode_tuple<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<(), Error> {
    Ok(())
}

pub fn decode_tuple_2<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let tuple = schema::Tuple::decode(input.as_slice())?;
    if tuple.items.len() != 2 {
        return Err(Error::InvalidInput);
    }
    let first = tuple.items.get(0).ok_or(Error::InvalidInput)?.clone();
    let second = tuple.items.get(1).ok_or(Error::InvalidInput)?.clone();
    Ok((first, second))
}

pub fn decode_tuple_3<'a>(input: Vec<u8>, conn: &'a super::connection::Connection) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Error> {
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
    StringDecodeError,
}

impl From<prost::DecodeError> for Error {
    fn from(e: prost::DecodeError) -> Self {
        Error::DecodeError(e)
    }
}

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
