use krpc_rs;



#[tokio::main]
async fn main() -> Result<(), krpc_rs::connection::Error> {
    println!("Hello, world!");

   
    let conn = krpc_rs::connection::Connection::new("127.0.0.1:50000").await?;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    Ok(())
}