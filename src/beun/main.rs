use krpc_rs;



#[tokio::main]
async fn main() -> Result<(), krpc_rs::Error> {
    println!("Hello, world!");

    let mut client = krpc_rs::Client::new();
    client.connect("127.0.0.1:50000").await?;

    // tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    client.activate_next_stage().await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    Ok(())
}