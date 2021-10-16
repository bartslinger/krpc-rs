use krpc_rs;



#[tokio::main]
async fn main() -> Result<(), krpc_rs::connection::Error> {
    println!("Hello, world!");

   
    let conn = krpc_rs::connection::Connection::new("127.0.0.1:50000").await?;
    let space_center = krpc_rs::space_center::SpaceCenter::new(&conn);
    
    let _vessel = space_center.active_vessel();
    
    loop {
        let (funds, science) = tokio::join!(
            space_center.funds(),
            space_center.science(),
        );
        println!("Got {:?} funds and {:?} science!", funds.unwrap(), science.unwrap());
    }
    
    // tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    // Ok(())
}