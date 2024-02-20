use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::str;

const BUFFER_SIZE : usize = 8096;
// Things to do
// 1. Graceful shutdown of connection
// 2. Handle connections from multiple clients
// 3. Explore HTTP Protocol and work towards implementation

struct TcpServer{
    connections: Option<Vec::<TcpStream>>,
    listener: Option<TcpListener>,
    address: String,
    port: String
}

impl TcpServer{
    fn new(address: String, port: String) -> TcpServer{
        TcpServer{
            address: address,
            port: port,
            listener: None,
            connections: None
        }
    }

    fn start_server(&mut self){
        let address = format!("{}:{}", self.address, self.port);
        self.connections = Some(Vec::<TcpStream>::new());
        self.listener = TcpListener::bind(address).ok();
        for stream in self.listener.as_mut().unwrap().incoming(){
            match stream{
                Ok(stream) => {
                    let clone = stream.try_clone().unwrap();
                    self.connections.as_mut().unwrap().push(clone);
                    let clone2 = stream.try_clone().unwrap();
                    thread::spawn(move || {
                        TcpServer::handle_connection(clone2);
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

    fn handle_connection(stream_data: TcpStream){
        let mut stream_data = stream_data.try_clone().unwrap();
        let buffer  = "Connected .. Send some data over. \n".as_bytes();
        stream_data.write_all(buffer).expect("Should have sent the data");
        let mut buffer = [0; BUFFER_SIZE];
        loop{
            match stream_data.read(&mut buffer){
                Ok(len) =>{
                    let string_result = str::from_utf8(&buffer[0..len]);
                    match string_result {
                        Ok(res) => {
                            print!("{}", res);
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


fn main() {
    let mut server = TcpServer::new("localhost".to_string(), "6969".to_string());
    server.start_server()
}
