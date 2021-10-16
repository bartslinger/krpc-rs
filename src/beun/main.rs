use krpc_rs;



#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

   
    let conn = krpc_rs::connection::Connection::new("127.0.0.1:50000").await?;
    let space_center = krpc_rs::space_center::SpaceCenter::new(&conn);
    
    let vessel = space_center.active_vessel().await?;
    println!("Vessel: {:?}", vessel);
    
    let control = vessel.control().await?;
    println!("Control: {:?}", control);
    
    let surface_reference_frame = vessel.surface_reference_frame().await?;
    println!("Reference frame: {:?}", surface_reference_frame);
    
    let flight = vessel.flight(&surface_reference_frame).await?;
    println!("Flight: {:?}", flight);
    
    let available_torque = vessel.available_torque().await?;
    println!("{:?}", available_torque);

    // let _ = control.activate_next_stage().await?;

    loop {
        let (roll, pitch, heading, velocity) = tokio::join!(
            flight.roll(),
            flight.pitch(),
            flight.heading(),
            vessel.velocity(&surface_reference_frame),
        );
        println!("{:.2}, {:.2}, {:.2}, {:?}", roll?, pitch?, heading?, velocity?);
    }
    // loop {
    //     let (met, mass, funds, science) = tokio::join!(
    //         vessel.met(),
    //         vessel.mass(),
    //         space_center.funds(),
    //         space_center.science(),
    //     );
    //     println!("{:.2}:: Mass: {:?}; Got {:?} funds and {:?} science!", met?, mass, funds, science);
    // }
    
    // tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    Ok(())
}

#[derive(Debug)]
enum Error {
    ConnectionError(krpc_rs::connection::Error),
    SpaceCenterError(krpc_rs::space_center::Error),
}

impl From<krpc_rs::connection::Error> for Error {
    fn from(e: krpc_rs::connection::Error) -> Self {
        Error::ConnectionError(e)
    }
}

impl From<krpc_rs::space_center::Error> for Error {
    fn from(e: krpc_rs::space_center::Error) -> Self {
        Error::SpaceCenterError(e)
    }
}


