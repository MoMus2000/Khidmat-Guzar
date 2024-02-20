use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
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

    fn write_to_all_connections(streams: &mut Vec<TcpStream>, current_stream: TcpStream){
        for st in streams{
            let message = format!("New Connection {}\n", current_stream.local_addr().unwrap().to_string());
            st.write_all(message.as_bytes());
        }
    }

    fn start_server(&mut self){
        let address = format!("{}:{}", self.address, self.port);
        self.connections = Some(Vec::<TcpStream>::new());
        self.listener = TcpListener::bind(address).ok();

        let shared_data = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
        
        let data_processor = Arc::clone(&shared_data);

        for stream in self.listener.as_mut().unwrap().incoming(){
            match stream{
                Ok(stream) => {
                    let clone = stream.try_clone().unwrap();
                    let clone3 = stream.try_clone().unwrap();

                    let mut shared_vector = shared_data.lock().unwrap();
                    shared_vector.push(clone3);

                    self.connections.as_mut().unwrap().push(clone);
                    TcpServer::write_to_all_connections(self.connections.as_mut().unwrap(), stream.try_clone().unwrap());
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
                        Ok(res) => {
                            print!("{}", res);
                            for mut stream in shared_data.lock().unwrap().iter_mut(){
                                stream.write_all(res.as_bytes());
                            }
                            println!("Current size {}", shared_data.lock().unwrap().len())
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
