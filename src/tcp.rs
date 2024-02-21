use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use std::thread;
use std::str;

use crate::server::Server;

const BUFFER_SIZE : usize = 8096;

pub struct TcpServer{
    listener: Option<TcpListener>,
    address: String,
    port: String
}

impl Server for TcpServer{
    fn start_server(&mut self){
        let address = format!("{}:{}", self.address, self.port);
        self.listener = TcpListener::bind(address).ok();

        let shared_data = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
        
        for stream in self.listener.as_mut().unwrap().incoming(){
            match stream{
                Ok(stream) => {
                    let clone3 = stream.try_clone().unwrap();

                    let mut shared_vector = shared_data.lock().unwrap();
                    shared_vector.push(clone3);

                    // TcpServer::write_to_all_connections(self.connections.as_mut().unwrap(), stream.try_clone().unwrap());
                    let clone2 = stream.try_clone().unwrap();
                    
                    let data_processor = Arc::clone(&shared_data);

                    thread::spawn(move || {
                        TcpServer::handle_connection(clone2, 
                        Arc::clone(&data_processor)
                    );
                    });
                }
                Err(err) => {
                    // Handle the error, e.g., print an error message or break the loop
                    eprintln!("Error accepting connection: {}", err);
                    break;
                }
            }
        }
    }

    fn handle_connection(stream_data: TcpStream, shared_data: Arc<Mutex<Vec<TcpStream>>>){
        let mut stream_data = stream_data.try_clone().unwrap();
        let buffer  = "Connected .. Send some data over. \n".as_bytes();
        stream_data.write_all(buffer).expect("Should have sent the data");
        let mut buffer = [0; BUFFER_SIZE];
        loop{
            match stream_data.read(&mut buffer){
                Ok(len) =>{
                    let string_result = str::from_utf8(&buffer[0..len]);
                    match string_result {
                        Ok(msg) => {
                            print!("{}", msg);
                            let data_clone = Arc::clone(&shared_data);
                            TcpServer::broadcast_to_all_connections(msg, data_clone);
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

impl TcpServer{
    pub fn new(address: String, port: String) -> TcpServer{
        TcpServer{
            address,
            port,
            listener: None,
        }
    }

    fn broadcast_to_all_connections(msg: &str, shared_data: Arc<Mutex<Vec<TcpStream>>>){
        for stream in shared_data.lock().unwrap().iter_mut(){
            let _ = stream.write_all(msg.as_bytes());
        }
    }
}