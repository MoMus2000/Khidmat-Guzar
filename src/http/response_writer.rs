use std::collections::HashMap;
use std::net::TcpStream;

pub struct ResponseWriter{
    pub stream : TcpStream,
    pub status_code : i32,
    pub headers : HashMap::<String, String>
}

impl ResponseWriter{
    pub fn new(stream : TcpStream) -> ResponseWriter{
        ResponseWriter{
            stream: stream,
            status_code: Default::default(),
            headers: HashMap::<String, String>::new()
        } 
    }

    pub fn write_status_code(&mut self, status_code : i32){
        self.status_code = status_code 
    }
}