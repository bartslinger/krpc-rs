use crate::connection;
use crate::connection::Connection;
use crate::decoder;
use crate::encoder;

use super::schema;

#[derive(Debug)]
pub struct SpaceCenter<'c> {
    conn: &'c Connection,
}
impl<'c> SpaceCenter<'c> {
    pub fn new(conn: &'c Connection) -> SpaceCenter<'c> {
        SpaceCenter {
            conn: conn,
        }
    }
    
    pub async fn active_vessel(&'c self) -> Result<Vessel<'c>, Error> {
        let result = self.conn.execute_procedure("SpaceCenter", "get_ActiveVessel", vec![]).await?;
        let id = decoder::decode_class(result)?;
        Ok(Vessel{
            id,
            parent: &self,
        })
    }
    
    pub async fn science(&self) -> Result<f32, Error> {
        let result = self.conn.execute_procedure("SpaceCenter", "get_Science", vec![]).await?;
        if result.len() == 0 {
            return Err(Error::Unavailable);
        }
        Ok(decoder::decode_float(result)?)
    }
    
    pub async fn funds(&self) -> Result<f64, Error> {
        let result = self.conn.execute_procedure("SpaceCenter", "get_Funds", vec![]).await?;
        if result.len() == 0 {
            return Err(Error::Unavailable);
        }
        Ok(decoder::decode_double(result)?)
    }
}

#[derive(Debug)]
pub struct Vessel<'a> {
    id: u64,
    parent: &'a SpaceCenter<'a>,
}

impl<'a> Vessel<'a> {
    
    pub async fn met(&self) -> Result<f64, Error> {
         let arguments = vec![schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        }];

        let result = self.parent.conn.execute_procedure("SpaceCenter", "Vessel_get_MET", arguments).await?;
        Ok(decoder::decode_double(result)?)
    }

    pub async fn mass(&self) -> Result<f32, Error> {
        let arguments = vec![schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        }];

        let result = self.parent.conn.execute_procedure("SpaceCenter", "Vessel_get_Mass", arguments).await?;
        Ok(decoder::decode_float(result)?)
    }

    pub async fn control(&'a self) -> Result<Control<'a>, Error> {
        let arguments = vec![schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        }];
        let result = self.parent.conn.execute_procedure("SpaceCenter", "Vessel_get_Control", arguments).await?;
        let id = decoder::decode_class(result)?;
        Ok(Control{
            id,
            parent: &self.parent,
        })
    }
}

#[derive(Debug)]
pub struct Control<'a> {
    id: u64,
    parent: &'a SpaceCenter<'a>
}

impl<'a> Control<'a> {
    pub async fn activate_next_stage(&self) -> Result<(), Error> {
        let arguments = vec![schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        }];

        let _result = self.parent.conn.execute_procedure("SpaceCenter", "Control_ActivateNextStage", arguments).await?;
        Ok(())
        
    }
}

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
