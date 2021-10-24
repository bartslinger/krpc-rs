use std::collections::VecDeque;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use prost::Message;
use super::schema;

type RawRpcReply = Vec<u8>;
type RpcQueue = Arc<Mutex<VecDeque<tokio::sync::oneshot::Sender<RawRpcReply>>>>;

#[derive(Debug)]
pub struct Connection {
    rpc_writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
    rpc_queue: RpcQueue,
    _stream_socket: u32,
    client_identifier: Vec<u8>,
}

impl Connection {
    
    pub async fn new(address: impl tokio::net::ToSocketAddrs) -> Result<Connection, Error> {

        let rpc_socket = tokio::net::TcpStream::connect(address).await?;
        let (rpc_reader, rpc_writer) = rpc_socket.into_split();

        let rpc_queue = Arc::new(Mutex::new(VecDeque::new()));
        
        Connection::start_reader(rpc_reader, rpc_queue.clone()).await?;
        
        let mut connection = Connection {
            rpc_writer: Arc::new(Mutex::new(rpc_writer)),
            rpc_queue: rpc_queue,
            _stream_socket: 0,
            client_identifier: vec![],
        };
        
        connection.register_client("rust_client").await?;

        Ok(connection)
    }
    
    async fn register_client(&mut self, client_name: &str) -> Result<(), Error> {
        let request = schema::ConnectionRequest {
            r#type: schema::connection_request::Type::Rpc as i32,
            client_name: client_name.to_string(),
            client_identifier: vec![],
        };
        let reply = self.execute_rpc(&request).await?;
       
        let mut slice = &*reply;
        let _len = prost::encoding::decode_varint(&mut slice);
        let connection_response = schema::ConnectionResponse::decode(slice)?;
        println!("Connection response: {:?}", connection_response);
        self.client_identifier = connection_response.client_identifier;
        Ok(())
    }
    
    pub async fn execute_procedure(&self, service: &str, procedure: &str, args: Vec<schema::Argument>) -> Result<Vec<u8>, Error> {
        let mut request = schema::Request::default();
        request.calls.push(schema::ProcedureCall {
            service: service.to_string(),
            procedure: procedure.to_string(),
            service_id: 0,
            procedure_id: 0,
            arguments: args,
        });
        let result = self.execute_rpc(&request).await?;
        let mut slice = &*result;
        let _len = prost::encoding::decode_varint(&mut slice);
        let response = schema::Response::decode(slice)?;
        let value = response.results.get(0).ok_or(Error::InvalidIndex)?.value.clone();
        Ok(value)
    }

    async fn execute_rpc(&self, message: &impl prost::Message) -> Result<Vec<u8>, Error> {
        
        let (oneshot_sender, oneshot_receiver) = tokio::sync::oneshot::channel::<RawRpcReply>();
        // Lock the writer already
        {
            let mut writer = self.rpc_writer.lock().await;
            // Register the reader first
            {
                let mut queue = self.rpc_queue.lock().await;
                (*queue).push_back(oneshot_sender);
            }
            
            // Then, send the request
            let buf =  message.encode_length_delimited_to_vec();

            let result = writer.write_all(&buf).await;
            if let Err(e) = result {
                println!("Error writing to buf: {:?}", e);
                // should probably remove the sender from the queue here
            }

            // drop the tcp writer
        }
        
        // And now we wait for the result
        Ok(oneshot_receiver.await?)
    }
    
    async fn start_reader(mut rpc_reader: tokio::net::tcp::OwnedReadHalf, rpc_queue: RpcQueue) -> Result<(), Error> {
        tokio::spawn(async move {

            let mut buf = vec![];
            loop {
                if let Err(e) = rpc_reader.read_buf(&mut buf).await {
                    println!("Error reading TCP stream: {:?}", e);
                    return;
                }

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
                        let (response, remaining) = buf.split_at(encoded_message_size);
                        {
                            let mut queue = rpc_queue.lock().await;
                            if let Some(oneshot) = queue.pop_front() {
                                let _ = oneshot.send(response.to_vec());
                            }
                        }
                        
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
    OneshotReceiveError(tokio::sync::oneshot::error::RecvError),
    InvalidIndex,
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

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(e: tokio::sync::oneshot::error::RecvError) -> Self {
        Error::OneshotReceiveError(e)
    }
}

