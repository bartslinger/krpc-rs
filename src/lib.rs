use tokio::io::AsyncWriteExt;

pub mod krpc {
    include!(concat!(env!("OUT_DIR"), "/krpc.schema.rs"));
}


#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EncodeError(prost::EncodeError),
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

        Ok(())
    }

    async fn write_message(&mut self, message: &impl prost::Message) -> Result<(), Error> {
        let len = message.encoded_len();

        let mut message_buf = Vec::with_capacity(len);
        message.encode(&mut message_buf)?;

        let mut buf = Vec::with_capacity(10);
        prost::encoding::encode_varint(len as u64, &mut buf);
        buf.append(&mut message_buf);

        match &mut(self.tcp_stream) {
            Some(s) => {
                let result = s.write_all(&buf).await;
                println!("{:?}", result);
            },
            None => {}
        };
        Ok(())
    }

    pub async fn say_hello(&mut self) {
        println!("hello");
        let mut test = krpc::ConnectionRequest {
            r#type: krpc::connection_request::Type::Rpc as i32,
            client_name: "rust_client".to_string(),
            client_identifier: vec![],
        };
        self.write_message(&mut test).await;

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    }

}