use prost;

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
