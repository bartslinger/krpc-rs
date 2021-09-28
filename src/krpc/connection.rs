use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use prost::Message;

pub mod krpc {
    include!(concat!(env!("OUT_DIR"), "/krpc.schema.rs"));
}

type RawRpcReply = Vec<u8>;
type RpcQueue = Arc<Mutex<Vec<tokio::sync::oneshot::Sender<RawRpcReply>>>>;

pub struct Connection {
    rpc_writer: tokio::net::tcp::OwnedWriteHalf,
    rpc_queue: RpcQueue,
    stream_socket: u32,
    client_identifier: Vec<u8>,
}

impl Connection {
    
    pub async fn new(address: impl tokio::net::ToSocketAddrs) -> Result<Connection, Error> {

        let rpc_socket = tokio::net::TcpStream::connect(address).await?;
        let (rpc_reader, rpc_writer) = rpc_socket.into_split();

        let rpc_queue = Arc::new(Mutex::new(vec![]));
        
        Connection::start_reader(rpc_reader, rpc_queue.clone()).await?;
        
        let mut connection = Connection {
            rpc_writer: rpc_writer,
            rpc_queue: rpc_queue,
            stream_socket: 0,
            client_identifier: vec![],
        };
        
        connection.register_client("rust_client").await?;

        Ok(connection)
    }
    
    async fn register_client(&mut self, client_name: &str) -> Result<(), Error> {
        
        let (oneshot_sender, oneshot_receiver) = tokio::sync::oneshot::channel::<RawRpcReply>();
        {
            let mut queue = self.rpc_queue.lock().await;
            queue.push(oneshot_sender);
        }
        self.send_connection_request(client_name).await?;
        
        println!("Waiting for reply:");
        let reply = oneshot_receiver.await;
        println!("Reply: {:?}", reply);
        // self.wait_for_connection_confirmation().await?;
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

    // async fn wait_for_connection_confirmation(&mut self) -> Result<(), Error> {
    //     let mut buf = vec![];
    //     self.rpc_reader.read_buf(&mut buf).await?;
    //     let mut slice = &*buf;

    //     let _len = prost::encoding::decode_varint(&mut slice)?;
    //     let response = krpc::ConnectionResponse::decode(slice)?;
    //     self.client_identifier = response.client_identifier;
        
    //     // let buf: Vec<u8> = vec![1,2,3];
    //     // krpc::ConnectionResponse::decode(buf.as_slice());
    //     Ok(())
    // }

    async fn write_message(&mut self, message: &impl prost::Message) -> Result<(), Error> {

        let buf =  message.encode_length_delimited_to_vec();

        let result = self.rpc_writer.write_all(&buf).await;
        if let Err(e) = result {
            println!("Error writing to buf: {:?}", e);
        }
        // println!("{:?}", result);
        Ok(())
    }

    
    async fn start_reader(mut rpc_reader: tokio::net::tcp::OwnedReadHalf, rpc_queue: RpcQueue) -> Result<(), Error> {
        tokio::spawn(async move {

            let mut buf = vec![];
            loop {
                let before = buf.len();
                if let Err(e) = rpc_reader.read_buf(&mut buf).await {
                    println!("Error reading TCP stream: {:?}", e);
                    return;
                }
                println!("{} {}", before, buf.len());

                // There are some bytes in the buffer now
                loop {
                    // try to decode a varint out of this, might not be possible
                    let mut slice = &*buf;
                    let len = match prost::encoding::decode_varint(&mut slice) {
                        Ok(l) => l,
                        Err(e) => panic!("Invalid varint {:?}", e),
                    };
                    
                    let varint_size = prost::encoding::encoded_len_varint(len);
                    let encoded_message_size = varint_size + len as usize;
                    if buf.len() >= encoded_message_size {
                        println!("+1 in reader");
                        let (response, remaining) = buf.split_at(encoded_message_size);
                        println!("response: {:?}", response);
                        {
                            let mut queue = rpc_queue.lock().await;
                            if let Some(oneshot) = queue.pop() {
                                let _ = oneshot.send(response.to_vec());
                            }
                        }
                        // let len = prost::encoding::decode_varint(&mut response);
                        // let test = krpc::ConnectionResponse::decode(response);
                        // println!("{:?}", test);
                        
                        
                        let new_buf = remaining.to_vec();
                        buf = new_buf;
                    } else {
                        break
                    }
                    if buf.len() == 0 {
                        break
                    }
                }
            }
        });
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
