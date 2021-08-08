pub mod krpc {
    include!(concat!(env!("OUT_DIR"), "/krpc.schema.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn write_message(message: &impl prost::Message) {
    println!("Message: {:?}", message);

    let len = message.encoded_len();

    let mut buf = Vec::with_capacity(len);

    let result = message.encode(&mut buf);
    println!("Result: {:?}", result);
    println!("{:?}", buf);

}

pub fn say_hello() {
    println!("hello");
    let test = krpc::ConnectionRequest {
        r#type: krpc::connection_request::Type::Rpc as i32,
        client_name: "rust_client".to_string(),
        client_identifier: vec![],
    };
    
    write_message(& test);

    // println!("{:?}", test);
}