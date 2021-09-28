use tokio::io::{AsyncWriteExt, AsyncReadExt};
use prost::Message;

pub mod krpc {
    include!(concat!(env!("OUT_DIR"), "/krpc.schema.rs"));
}

pub struct Connection {
    rpc_socket: tokio::net::TcpStream,
    rpc_queue: u32,
    stream_socket: u32,
    client_identifier: Vec<u8>,
}

impl Connection {
    
    pub async fn new(address: impl tokio::net::ToSocketAddrs) -> Result<Connection, Error> {

        let rpc_socket = tokio::net::TcpStream::connect(address).await?;
        
        let mut connection = Connection {
            rpc_socket: rpc_socket,
            rpc_queue: 0,
            stream_socket: 0,
            client_identifier: vec![],
        };
        
        connection.register_client("rust_client").await?;

        Ok(connection)
    }
    
    async fn register_client(&mut self, client_name: &str) -> Result<(), Error> {
        self.send_connection_request(client_name).await?;
        self.wait_for_connection_confirmation().await?;
        Ok(())
    }

    async fn send_connection_request(&mut self, cilent_name: &str) -> Result<(), Error> {
        let mut test = krpc::ConnectionRequest {
            r#type: krpc::connection_request::Type::Rpc as i32,
            client_name: cilent_name.to_string(),
            client_identifier: vec![],
        };
        self.write_message(&mut test).await?;

        Ok(())
    }

    async fn wait_for_connection_confirmation(&mut self) -> Result<(), Error> {
        let mut buf = vec![];
        self.rpc_socket.read_buf(&mut buf).await?;
        let mut slice = &*buf;

        let _len = prost::encoding::decode_varint(&mut slice)?;
        let response = krpc::ConnectionResponse::decode(slice)?;
        self.client_identifier = response.client_identifier;
        
        // let buf: Vec<u8> = vec![1,2,3];
        // krpc::ConnectionResponse::decode(buf.as_slice());
        Ok(())
    }

    async fn write_message(&mut self, message: &impl prost::Message) -> Result<(), Error> {

        let buf =  message.encode_length_delimited_to_vec();

        let result = self.rpc_socket.write_all(&buf).await;
        if let Err(e) = result {
            println!("Error writing to buf: {:?}", e);
        }
        // println!("{:?}", result);
        Ok(())
    }


}



#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EncodeError(prost::EncodeError),
    DecodeError(prost::DecodeError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<prost::EncodeError> for Error {
    fn from(e: prost::EncodeError) -> Self {
        Error::EncodeError(e)
    }
}

impl From<prost::DecodeError> for Error {
    fn from(e: prost::DecodeError) -> Self {
        Error::DecodeError(e)
    }
}
