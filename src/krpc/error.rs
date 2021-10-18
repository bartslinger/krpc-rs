use crate::connection;
use crate::decoder;
use crate::encoder;

#[derive(Debug)]
pub enum Error {
    ConnectionError(connection::Error),
    Unavailable,
    DecoderError(decoder::Error),
    EncoderError(encoder::Error),
}

impl From<connection::Error> for Error {
    fn from(e: connection::Error) -> Self {
        Error::ConnectionError(e)
    }
}

impl From<decoder::Error> for Error {
    fn from(e: decoder::Error) -> Self {
        Error::DecoderError(e)
    }
}

impl From<encoder::Error> for Error {
    fn from(e: encoder::Error) -> Self {
        Error::EncoderError(e)
    }
}
