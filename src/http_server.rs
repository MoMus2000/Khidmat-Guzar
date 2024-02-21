use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Write, Read};
use std::str;

use crate::server::Server;

pub struct HttpServer{
    address: String,
    port: String,
    listener: Option<TcpListener>
}

const BUFFER_SIZE : usize = 8096;

impl Server for HttpServer{
    fn start_server(&mut self){
        let shared_data = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
        let ip_address = format!("{}:{}",self.address, self.port);
        match TcpListener::bind(&ip_address){
           Ok(listener) => {
            self.listener = Some(listener);
           }
           Err(err) => {
            eprintln!("ERROR: binding to {}", &ip_address);
            panic!("{}",err);
           }
        }
        for stream in self.listener.as_mut().unwrap().incoming(){
            match stream{
                Ok(stream) => {
                    let moved_shared_data = Arc::clone(&shared_data);
                    thread::spawn(move || {
                        HttpServer::handle_connection(stream, moved_shared_data);
                    });
                }
                Err(err) => {
                    eprintln!("ERROR: with incoming connection from: {}", &ip_address);
                    panic!("{}",err);
                }
            }
        }
    }

    // Implementation of HTTP -> this is what will differ in comparision to TCP;
    fn handle_connection(stream_data: TcpStream, _ : Arc<Mutex<Vec<TcpStream>>>){
        let mut stream_data = stream_data.try_clone().unwrap();
        // let buffer  = "Connected .. Send some data over. \n".as_bytes();
        // // stream_data.write_all(buffer).expect("Should have sent the data");
        let mut buffer = [0; BUFFER_SIZE];
        loop{
            match stream_data.read(&mut buffer){
                Ok(len) =>{
                    let string_result = str::from_utf8(&buffer[0..len]);
                    match string_result {
                        Ok(msg) => {
                            println!("Got msg:");
                            println!("{}", msg);
                            let stream_data_copied= stream_data.try_clone();
                            HttpServer::write_http_status(stream_data_copied.unwrap());
                        },
                        Err(e)=>{
                            eprintln!("Something went wrong: {}", e);
                            break
                        }
                    }
                },
                Err(e)=>{
                    eprintln!("Something went wrong reading data from the server ...");
                    eprint!("{}", e);
                    break
                }
            }
        }
    }

}

impl HttpServer{
    pub fn new(address : String, port : String) -> HttpServer{
        HttpServer{
            address,
            port,
            listener: None
        }
    }

    fn write_http_status(mut stream: TcpStream){
        let response_line = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";

        let response: Vec<u8> = [
        response_line.iter(),
        ]
        .into_iter()
        .flatten()
        .cloned()
        .collect();


        match stream.write_all(&response){
            Ok(_)=>{},
            Err(e)=>{println!("Something went wrong writing: {}",e)}
        };

        println!("Got request .. Sending response");
        println!("{:?}", str::from_utf8(&response));
    }
}