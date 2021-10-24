#![allow(dead_code, unused_variables, unused_imports)]
use krpc_rs::{connection, space_center};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

   
    let conn = connection::Connection::new("127.0.0.1:50000").await?;
    let space_center = space_center::SpaceCenter::new(&conn);

    let vessel = space_center.get_active_vessel().await?;
    println!("Vessel: {:?}", vessel);
    
    let control = vessel.get_control().await?;
    println!("Control: {:?}", control);
    let autopilot = vessel.get_auto_pilot().await?;
    
    let vessel_name = vessel.get_name().await?;
    println!("Vessel name: {}", vessel_name);
    
    let resources = vessel.get_resources().await?;
    let resource_names = resources.get_names().await?;
    // println!("resource names: {}", resource_names);
    // try static method
    let density = space_center::Resources::density(&conn, "yolo".into()).await;
    println!("Density: {}", density.unwrap());
    
    let _ = control.set_sas(true).await;
    let _ = control.set_throttle(1.0).await;
    let _ = autopilot.set_target_heading(-90.0).await;
    let _ = autopilot.set_target_roll(-90.0).await;
    let _ = autopilot.set_target_pitch(28.0).await;
    let _ = autopilot.engage().await;

    
    let _ = control.activate_next_stage().await;
    
    let surface_reference_frame = vessel.get_surface_reference_frame().await?;
    println!("Reference frame: {:?}", surface_reference_frame);
    
    tokio::time::sleep(tokio::time::Duration::from_millis(3450)).await;
    let _ = control.set_throttle(0.2).await;
    let _ = autopilot.set_target_pitch(80.0).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let _ = control.set_throttle(0.0).await;
    let _ = control.activate_next_stage().await;
    let _ = control.activate_next_stage().await;
    let _ = control.activate_next_stage().await;
    let _ = control.activate_next_stage().await;
    
    // let flight = vessel.flight(&surface_reference_frame).await?;
    // println!("Flight: {:?}", flight);
    
    // let available_torque = vessel.available_torque().await?;
    // println!("{:?}", available_torque);

    // // let _ = control.activate_next_stage().await?;

    // loop {
    //     let (roll, pitch, heading, velocity) = tokio::join!(
    //         flight.roll(),
    //         flight.pitch(),
    //         flight.heading(),
    //         vessel.velocity(&surface_reference_frame),
    //     );
    //     println!("{:.2}, {:.2}, {:.2}, {:?}", roll?, pitch?, heading?, velocity?);
    // }
    // // loop {
    // //     let (met, mass, funds, science) = tokio::join!(
    // //         vessel.met(),
    // //         vessel.mass(),
    // //         space_center.funds(),
    // //         space_center.science(),
    // //     );
    // //     println!("{:.2}:: Mass: {:?}; Got {:?} funds and {:?} science!", met?, mass, funds, science);
    // // }
    
    // // tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    Ok(())
}

#[derive(Debug)]
enum Error {
    ConnectionError(krpc_rs::connection::Error),
    ServiceError(krpc_rs::error::Error),
}

impl From<krpc_rs::connection::Error> for Error {
    fn from(e: krpc_rs::connection::Error) -> Self {
        Error::ConnectionError(e)
    }
}

impl From<krpc_rs::error::Error> for Error {
    fn from(e: krpc_rs::error::Error) -> Self {
        Error::ServiceError(e)
    }
}


