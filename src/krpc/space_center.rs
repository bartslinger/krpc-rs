use crate::connection;
use crate::connection::Connection;
use crate::decoder;

pub mod schema {
    include!(concat!(env!("OUT_DIR"), "/krpc.schema.rs"));
}

pub struct SpaceCenter<'c> {
    conn: &'c Connection,
}
impl<'c> SpaceCenter<'c> {
    pub fn new(conn: &'c Connection) -> SpaceCenter<'c> {
        SpaceCenter {
            conn: conn,
        }
    }
    
    pub async fn active_vessel(&self) -> Result<Vessel, Error> {

        Ok(Vessel{})
    }
    
    pub async fn science(&self) -> Result<f32, Error> {
        let result = self.conn.execute_procedure("SpaceCenter", "get_Science", vec![]).await?;
        Ok(decoder::decode_float(result)?)
    }
    
    pub async fn funds(&self) -> Result<f64, Error> {
        let result = self.conn.execute_procedure("SpaceCenter", "get_Funds", vec![]).await?;
        Ok(decoder::decode_double(result)?)
    }
}

pub struct Vessel {}


#[derive(Debug)]
pub enum Error {
    ConnectionError(connection::Error),
    DecodeError(prost::DecodeError),
    DecoderError(decoder::Error)
}

impl From<connection::Error> for Error {
    fn from(e: connection::Error) -> Self {
        Error::ConnectionError(e)
    }
}

impl From<prost::DecodeError> for Error {
    fn from(e: prost::DecodeError) -> Self {
        Error::DecodeError(e)
    }
}

impl From<decoder::Error> for Error {
    fn from(e: decoder::Error) -> Self {
        Error::DecoderError(e)
    }
}
