use prost;
use prost::bytes::BufMut;

pub trait KRPCEncode<'a> {
    fn krpc_encode(&'a self) -> Result<Vec<u8>, Error>;
}

impl<'a> KRPCEncode<'a> for String {
    fn krpc_encode(&self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::<u8>::new();
        prost::encoding::encode_varint(self.len() as u64, &mut buf);
        buf.put_slice(self.as_bytes());
        Ok(buf)
    }
}

pub fn encode_none() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_bool(input: bool) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::<u8>::new();
    let value = if input == true { 1 } else { 0 };
    prost::encoding::encode_varint(value, &mut buf);
    Ok(buf)
}

pub fn encode_float(input: f32) -> Result<Vec<u8>, Error> {
    Ok(Vec::from(f32::to_le_bytes(input)))
}

pub fn encode_double(input: f64) -> Result<Vec<u8>, Error> {
    Ok(Vec::from(f64::to_le_bytes(input)))
}

pub fn encode_uint32(input: u32) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_string(input: String) -> Result<Vec<u8>, Error> {
    input.krpc_encode()
}

pub fn encode_sint32(input: i32) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_list(input: (/*list*/)) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_dictionary(input: (/*dict*/)) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

// pub fn encode_enumeration(input: (/*enum*/)) -> Result<Vec<u8>, Error> {
//     Ok(Vec::new())
// }

pub fn encode_set(input: (/*set*/)) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}
pub fn encode_tuple(input: (/*tuple*/)) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}
pub fn encode_u64(input: u64) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::<u8>::new();
    prost::encoding::encode_varint(input, &mut buf);
    Ok(buf)
}

#[derive(Debug)]
pub enum Error {
    InvalidInput,
    EncodeError(prost::EncodeError),
}

impl From<prost::EncodeError> for Error {
    fn from(e: prost::EncodeError) -> Self {
        Error::EncodeError(e)
    }
}
