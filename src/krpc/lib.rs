use tokio::io::{AsyncWriteExt, AsyncReadExt};
use prost::Message;

pub mod krpc {
    include!(concat!(env!("OUT_DIR"), "/krpc.schema.rs"));
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
    client_identifier: Vec<u8>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            tcp_stream: None,
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
                let _result = s.write_all(&buf).await;
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
        let mut test = krpc::ConnectionRequest {
            r#type: krpc::connection_request::Type::Rpc as i32,
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
                let response = krpc::ConnectionResponse::decode(slice)?;
                self.client_identifier = response.client_identifier;
            },
            None => {}
        };
        
        // let buf: Vec<u8> = vec![1,2,3];
        // krpc::ConnectionResponse::decode(buf.as_slice());
        Ok(())
    }

    pub async fn get_status(&mut self) -> Result<(), Error> {
        let mut request = krpc::Request::default();
        let mut call = krpc::ProcedureCall::default();
        call.service = "KRPC".to_string();
        call.procedure = "GetStatus".to_string();
        request.calls.push(call);
        self.write_message(&request).await?;
        let return_value = self.read_response().await?;
        // Now we need to parse this
        let status = krpc::Status::decode(return_value.as_slice())?;
        if status.version == "0.4.8" {
            println!("STATUS: {:?}", status);
            return Ok(())
        } else {
            return Err(Error::UnexpectedResult)
        }

        println!("{:?}", status);

        Ok(())
    }

    pub async fn list_services(&mut self) -> Result<(), Error> {
        self.perform_request("KRPC", "GetServices", vec![]).await?;
        let return_value = self.read_response().await?;
        let services = krpc::Services::decode(return_value.as_slice())?.services;
        for s in &services {
            println!("{:?}", s.name);
            for p in &s.procedures {
                if p.name == "Vessel_get_Control" {
                    println!("{:?}", p);
                }
                // println!(" - {:?}", p.name);
            }
        }
        Ok(())
    }

    pub async fn get_active_vessel(&mut self) -> Result<(), Error> {
        self.perform_request("SpaceCenter", "get_ActiveVessel", vec![]).await?;
        let return_value = self.read_response().await?;
        println!("Procedure result: {:?}", return_value);

        self.perform_request("SpaceCenter", "Vessel_get_Control", vec![1]).await?;
        let return_value = self.read_response().await?;
        println!("Procedure result: {:?}", return_value);

        Ok(())
    }

    pub async fn activate_next_stage(&mut self) -> Result<(), Error> {
        self.perform_request("SpaceCenter", "Control_ActivateNextStage", vec![2]).await?;
        let return_value = self.read_response().await?;
        println!("Next stage response: {:?}", return_value);
        Ok(())
    }
    
    async fn read_response(&mut self) -> Result<Vec<u8>, Error> {
        match &mut self.tcp_stream {
            Some(s) => {
                let mut buf = vec![];
                let mut cnt = 0;
                let response = loop {
                    cnt += 1;
                    s.read_buf(&mut buf).await?;
                    // let aaa = buf.len();
                    let mut slice = &*buf;

                    let _len = prost::encoding::decode_varint(&mut slice)?;
                    let response = krpc::Response::decode(slice);
                    match response {
                        Ok(response) => { break response },
                        Err(_) => {},
                    }
                };

                let return_value = response.results.get(0).ok_or(Error::NoResult)?.value.clone();
                return Ok(return_value)
            },
            None => {}
        };
        
        // let buf: Vec<u8> = vec![1,2,3];
        // krpc::ConnectionResponse::decode(buf.as_slice());
        Err(Error::NotConnected)
    }

    async fn perform_request(&mut self, service: &str, procedure: &str, args: Vec<u8>) -> Result<(), Error> {
        let mut request = krpc::Request::default();
        let mut call = krpc::ProcedureCall::default();
        call.service = service.to_string();
        call.procedure = procedure.to_string();
        for a in args {
            let mut argument = krpc::Argument::default();
            argument.value = vec![a];
            call.arguments.push(argument);
        }
        request.calls.push(call);

        println!("{:?}", request);

        self.write_message(&request).await?;
        Ok(())
    }
 
}