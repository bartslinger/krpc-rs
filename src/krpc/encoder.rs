use prost;

pub fn encode_none() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_bool() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_float(input: f32) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_double(input: f64) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_uint32() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_string() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_sint32() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_list() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_dictionary() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_enumeration() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn encode_set() -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}
pub fn encode_tuple() -> Result<Vec<u8>, Error> {
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
