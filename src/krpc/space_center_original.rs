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

    pub async fn velocity(&self, reference_frame: &'a ReferenceFrame<'a>) -> Result<(f64, f64, f64), Error> {
        let arguments = vec![
            schema::Argument {
                position: 0,
                value: encoder::encode_u64(self.id)?,
            },
            schema::Argument {
                position: 1,
                value: encoder::encode_u64(reference_frame.id)?,
            },
        ];

        let result = self.parent.conn.execute_procedure("SpaceCenter", "Vessel_Velocity", arguments).await?;
        let a = decoder::decode_tuple_3(result)?;
        println!("{:?}", a);
        let velocity = (decoder::decode_double(a.0)?, decoder::decode_double(a.1)?, decoder::decode_double(a.2)?);
        Ok(velocity)
    }
   
    pub async fn available_torque(&self) -> Result<((f64, f64, f64), (f64, f64, f64)), Error> {
        let arguments = vec![schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        }];
        let result = self.parent.conn.execute_procedure("SpaceCenter", "Vessel_get_AvailableTorque", arguments).await?;
        let a = decoder::decode_tuple_2(result)?;
        let first = decoder::decode_tuple_3(a.0)?;
        let second = decoder::decode_tuple_3(a.1)?;
        Ok(
            ((decoder::decode_double(first.0)?, decoder::decode_double(first.1)?, decoder::decode_double(first.2)?),
             (decoder::decode_double(second.0)?, decoder::decode_double(second.1)?, decoder::decode_double(second.2)?)
            )
        )
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

    pub async fn flight(&'a self, reference_frame: &'a ReferenceFrame<'a>) -> Result<Flight<'a>, Error> {
        let arguments = vec![
            schema::Argument {
                position: 0,
                value: encoder::encode_u64(self.id)?,
            },
            schema::Argument {
                position: 1,
                value: encoder::encode_u64(reference_frame.id)?,
            },
        ];
        let result = self.parent.conn.execute_procedure("SpaceCenter", "Vessel_Flight", arguments).await?;
        let id = decoder::decode_class(result)?;
        Ok(Flight{
            id,
            parent: &self.parent,
        })
    }
    
    pub async fn surface_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, Error> {
        let arguments = vec![schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        }];
        let result = self.parent.conn.execute_procedure("SpaceCenter", "Vessel_get_SurfaceReferenceFrame", arguments).await?;
        let id = decoder::decode_class(result)?;
        Ok(ReferenceFrame{
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
pub struct Flight<'a> {
    id: u64,
    parent: &'a SpaceCenter<'a>,
}

impl<'a> Flight<'a> {

    pub async fn roll(&self) -> Result<f32, Error> {
        let arguments = vec![schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        }];

        let result = self.parent.conn.execute_procedure("SpaceCenter", "Flight_get_Roll", arguments).await?;
        Ok(decoder::decode_float(result)?)
    }

    pub async fn pitch(&self) -> Result<f32, Error> {
        let arguments = vec![schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        }];

        let result = self.parent.conn.execute_procedure("SpaceCenter", "Flight_get_Pitch", arguments).await?;
        Ok(decoder::decode_float(result)?)
    }

    pub async fn heading(&self) -> Result<f32, Error> {
        let arguments = vec![schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        }];

        let result = self.parent.conn.execute_procedure("SpaceCenter", "Flight_get_Heading", arguments).await?;
        Ok(decoder::decode_float(result)?)
    }

}

#[derive(Debug)]
pub struct ReferenceFrame<'a> {
    id: u64,
    parent: &'a SpaceCenter<'a>,
}
