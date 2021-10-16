use tokio::io::{AsyncWriteExt, AsyncReadExt};
use prost::Message;

pub mod schema {
    include!(concat!(env!("OUT_DIR"), "/krpc.schema.rs"));
}

mod decoder;
mod encoder;
pub mod connection;
pub mod space_center;

#[derive(Debug)]
pub struct Vessel {
    id: u64,
}
impl Vessel {
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EncodeError(prost::EncodeError),
    DecodeError(prost::DecodeError),
    NoResult,
    UnexpectedResult,
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

impl From<prost::DecodeError> for Error {
    fn from(e: prost::DecodeError) -> Self {
        Error::DecodeError(e)
    }
}

pub struct Client {
    tcp_stream: Option<tokio::net::TcpStream>,
    buf: Vec<u8>,
    client_identifier: Vec<u8>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            tcp_stream: None,
            buf: vec![],
            client_identifier: vec![],
        }
    }

    pub async fn connect(&mut self, address: impl tokio::net::ToSocketAddrs) -> Result<(), Error> {
        self.tcp_stream = Some(tokio::net::TcpStream::connect(address).await?);

        self.send_connection_request().await?;
        self.wait_for_connection_confirmation().await?;
        // println!("{:?}", self.client_identifier);
        Ok(())
    }

    async fn write_message(&mut self, message: &impl prost::Message) -> Result<(), Error> {

        let buf =  message.encode_length_delimited_to_vec();

        match &mut self.tcp_stream {
            Some(s) => {
                let result = s.write_all(&buf).await;
                if let Err(e) = result {
                    println!("Error writing to buf: {:?}", e);
                }
                // println!("{:?}", result);
            },
            None => {
                println!("Not connected, cannot write message");
                return Err(Error::NotConnected);
            }
        };
        Ok(())
    }

    async fn send_connection_request(&mut self) -> Result<(), Error> {
        let mut test = schema::ConnectionRequest {
            r#type: schema::connection_request::Type::Rpc as i32,
            client_name: "rust_client".to_string(),
            client_identifier: vec![],
        };
        self.write_message(&mut test).await?;

        Ok(())
    }

    async fn wait_for_connection_confirmation(&mut self) -> Result<(), Error> {
        match &mut self.tcp_stream {
            Some(s) => {
                let mut buf = vec![];
                s.read_buf(&mut buf).await?;
                let mut slice = &*buf;

                let _len = prost::encoding::decode_varint(&mut slice)?;
                let response = schema::ConnectionResponse::decode(slice)?;
                self.client_identifier = response.client_identifier;
            },
            None => {}
        };
        
        // let buf: Vec<u8> = vec![1,2,3];
        // schema::ConnectionResponse::decode(buf.as_slice());
        Ok(())
    }

    // pub async fn get_status(&mut self) -> Result<(), Error> {
    //     let mut request = schema::Request::default();
    //     let mut call = schema::ProcedureCall::default();
    //     call.service = "KRPC".to_string();
    //     call.procedure = "GetClientName".to_string(); // GetClientName
    //     request.calls.push(call);
    //     self.write_message(&request).await?;
    //     self.write_message(&request).await?;
    //     self.write_message(&request).await?;
    //     self.write_message(&request).await?;
    //     self.write_message(&request).await?;
    //     self.write_message(&request).await?;
    //     self.write_message(&request).await?;
    //     self.write_message(&request).await?;
    //     self.write_message(&request).await?;
    //     self.write_message(&request).await?;
    //     let return_value = self.read_response().await?;
    //     println!("{:?}", return_value);
    //     let mut slice = &*return_value;
    //     let len = prost::encoding::decode_varint(&mut slice);
    //     match std::str::from_utf8(slice) {
    //         Ok(name) => println!("Name: {}", name),
    //         Err(e) => {}
    //     };
    //     return Ok(());
    //     // Now we need to parse this
    //     let status = schema::Status::decode(return_value.as_slice())?;
    //     if status.version == "0.4.8" {
    //         // println!("STATUS: {:?}", status);
    //         return Ok(())
    //     } else {
    //         return Err(Error::UnexpectedResult)
    //     }

    //     println!("{:?}", status);

    //     Ok(())
    // }
 
    async fn read_response(&mut self) -> Result<Vec<u8>, Error> {
        match &mut self.tcp_stream {
            Some(s) => {

                let mut read_messages = 0;
                loop {
                    // read some stuff into the buf
                    let before = self.buf.len();
                    let _i = s.read_buf(&mut self.buf).await?;
                    println!("{} {}", before, self.buf.len());

                    loop {
                        // try to decode a varint out of this, might not be possible
                        let mut slice = &*(self.buf);
                        let len = match prost::encoding::decode_varint(&mut slice) {
                            Ok(l) => l,
                            Err(e) => panic!("Invalid varint {:?}", e),
                        };
                        
                        let varint_size = prost::encoding::encoded_len_varint(len);
                        let encoded_message_size = varint_size + len as usize;
                        if self.buf.len() >= encoded_message_size{
                            read_messages += 1;
                            println!("+1");
                            let new_buf = self.buf[encoded_message_size..].to_vec();
                            self.buf = new_buf;
                        } else {
                            break
                        }
                        if self.buf.len() == 0 {
                            break
                        }
                    }
                    if read_messages == 10 {
                        break
                    }
                }

                println!("{:?}", read_messages);
                return Ok(vec![read_messages]);
                // let mut len: u64 = 0;
                // let response = loop {
                //     let before = self.buf.len();
                //     let i = s.read_buf(&mut self.buf).await?;
                //     let after = self.buf.len();

                //     if len == 0 {
                //         let mut slice = &*(self.buf);
                //         len = prost::encoding::decode_varint(&mut slice)?;
                //         let varint_space = prost::encoding::encoded_len_varint(len);
                //         self.buf = self.buf[varint_space..].to_vec();
                //     }
                //     let afterafter = self.buf.len();
                //     println!("Buf: {}, {}, {}, {:?}", before, after, afterafter, i);
                //     if self.buf.len() < len as usize {
                //         println!("continue");
                //         continue
                //     }
                //     let decode_slice = self.buf.get(0..(len as usize)).ok_or(Error::NoResult)?;
                //     let response = schema::Response::decode(decode_slice);
                //     match response {
                //         Ok(response) => { 
                //             self.buf = self.buf[(len as usize)..].to_vec();
                //             break response
                //         },
                //         Err(e) => {
                //             println!("Cannot decode: {:?}", e);
                //         },
                //     }
                // };

                // let return_value = response.results.get(0).ok_or(Error::NoResult)?.value.clone();
                // return Ok(return_value)
            },
            None => {},
        };
        
        // let buf: Vec<u8> = vec![1,2,3];
        // schema::ConnectionResponse::decode(buf.as_slice());
        Err(Error::NotConnected)
    }

    pub async fn list_services(&mut self) -> Result<(), Error> {
        self.perform_request("KRPC", "GetServices", vec![]).await?;
        let return_value = self.read_response().await?;
        let services = schema::Services::decode(return_value.as_slice())?.services;
        for s in &services {
            if s.name != "KRPC" {
                continue
            }
            for p in &s.procedures {
                println!("{:?}", p);
                if p.name == "Vessel_get_Control" {
                    println!("{:?}", p.name);
                }
                // let types = match &p.return_type {
                //     Some(t) => {
                //         if schema::r#type::TypeCode::from_i32(t.code) == Some(schema::r#type::TypeCode::Tuple) {
                //             println!("{:?} {:?}", p.name, t);
                //         }
                //         format!("{:?}", schema::r#type::TypeCode::from_i32(t.code))
                //     },
                //     _ => "".to_string(),
                // };
                // println!("{:?}: {:?}", p.name, types);
                // println!("{:?}", p);
                // println!("");
            }
        }
        Ok(())
    }

    pub async fn get_active_vessel(&mut self) -> Result<Vessel, Error> {
        self.perform_request("SpaceCenter", "get_ActiveVessel", vec![]).await?;
        let return_value = self.read_response().await?;
        println!("Procedure result: {:?}", return_value);
        
        let id = prost::encoding::decode_varint(&mut return_value.as_slice())?;
        // Parse return value as uint64
        self.perform_request("SpaceCenter", "Vessel_get_Control", vec![1]).await?;
        let return_value = self.read_response().await?;
        println!("Procedure result: {:?}", return_value);

        Ok(Vessel {
            id
        })
    }

    pub async fn activate_next_stage(&mut self) -> Result<(), Error> {
        self.perform_request("SpaceCenter", "Control_ActivateNextStage", vec![2]).await?;
        let return_value = self.read_response().await?;
        println!("Next stage response: {:?}", return_value);
        Ok(())
    }
    
    async fn perform_request(&mut self, service: &str, procedure: &str, args: Vec<u8>) -> Result<(), Error> {
        let mut request = schema::Request::default();
        let mut call = schema::ProcedureCall::default();
        call.service = service.to_string();
        call.procedure = procedure.to_string();
        for a in args {
            let mut argument = schema::Argument::default();
            argument.value = vec![a];
            call.arguments.push(argument);
        }
        request.calls.push(call);

        println!("{:?}", request);

        self.write_message(&request).await?;
        Ok(())
    }
 
}