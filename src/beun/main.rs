use krpc_rs;



#[tokio::main]
async fn main() -> Result<(), krpc_rs::connection::Error> {
    println!("Hello, world!");

   
    let conn = krpc_rs::connection::Connection::new("127.0.0.1:50000").await?;
    
    Ok(())
}