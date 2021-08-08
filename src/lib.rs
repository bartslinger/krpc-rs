use tokio::io::AsyncWriteExt;
use prost::Message;

pub mod krpc {
    include!(concat!(env!("OUT_DIR"), "/krpc.schema.rs"));
}


#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EncodeError(prost::EncodeError),
    NotConnected,
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

pub struct Client {
    tcp_stream: Option<tokio::net::TcpStream>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            tcp_stream: None
        }
    }

    pub async fn connect(&mut self, address: impl tokio::net::ToSocketAddrs) -> Result<(), Error> {
        self.tcp_stream = Some(tokio::net::TcpStream::connect(address).await?);

        self.send_connection_request().await?;
        self.wait_for_connection_confirmation().await?;
        Ok(())
    }

    async fn write_message(&mut self, message: &impl prost::Message) -> Result<(), Error> {

        let buf =  message.encode_length_delimited_to_vec();

        match &mut(self.tcp_stream) {
            Some(s) => {
                let result = s.write_all(&buf).await;
                println!("{:?}", result);
            },
            None => {
                return Err(Error::NotConnected);
            }
        };
        Ok(())
    }

    async fn send_connection_request(&mut self) -> Result<(), Error> {
        let mut test = krpc::ConnectionRequest {
            r#type: krpc::connection_request::Type::Rpc as i32,
            client_name: "rust_client".to_string(),
            client_identifier: vec![],
        };
        self.write_message(&mut test).await?;

        Ok(())
    }

    async fn wait_for_connection_confirmation(&mut self) -> Result<(), Error> {
        let buf: Vec<u8> = Vec::with_capacity(100);
        // krpc::ConnectionResponse::decode(buf);
        Ok(())
    }

}