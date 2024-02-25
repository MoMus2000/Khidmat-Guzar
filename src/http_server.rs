use core::panic;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::str;

use crate::http::response_writer::ResponseWriter;
use crate::http::router::Router;
use crate::server::HTTP_Server;
use crate::http::{self, router};
use crate::http::http_request::HttpRequest;

pub struct HttpServer{
    address: String,
    port: String,
    listener: Option<TcpListener>,
    router: Option<router::Router>,
    running: bool
}

const BUFFER_SIZE : usize = 8096;

impl HTTP_Server for HttpServer{
    fn start_server(&mut self){
        let ip_address = format!("{}:{}",self.address, self.port);
        self.running = true;
        match TcpListener::bind(&ip_address){
           Ok(listener) => {
            self.listener = Some(listener);
           }
           Err(err) => {
            eprintln!("ERROR: binding to {}", &ip_address);
            panic!("{}",err);
           }
        }

        let shared_data = Arc::new(Mutex::new(true));

        for stream in self.listener.as_mut().unwrap().incoming(){
            match stream{
                Ok(stream) => {
                    let cloned_router = self.router.clone();
                    let running_state = Arc::clone(&shared_data);
                    thread::spawn(move || {
                        HttpServer::handle_connection(stream, cloned_router, Arc::clone(&running_state));
                    });
                }
                Err(err) => {
                    eprintln!("ERROR: with incoming connection from: {}", &ip_address);
                    panic!("{}",err);
                }
            }
        }

    }

    fn close(&mut self) {
        println!("Running set to false");
        self.running = false;
    }

    // Implementation of HTTP -> this is what will differ in comparision to TCP;
    // Closing the http connection after serving the required resource
    fn handle_connection(stream_data: TcpStream, router: Option<Router>, running_state: Arc<Mutex<bool>>){
        let mut stream_data = stream_data.try_clone().unwrap();
        // let buffer  = "Connected .. Send some data over. \n".as_bytes();
        // // stream_data.write_all(buffer).expect("Should have sent the data");
        let mut buffer = [0; BUFFER_SIZE];
        loop{
            if *running_state.lock().unwrap() { 
                match stream_data.read(&mut buffer){
                    Ok(len) =>{
                        let string_result = str::from_utf8(&buffer[0..len]);
                        match string_result {
                            Ok(msg) => {
                                let parsed_request = match HttpRequest::parse_request(msg) {
                                    Ok(request) => request,
                                    Err(error) => {
                                        let mut rw = ResponseWriter::new(stream_data.try_clone().unwrap());
                                        rw.write_status_code(500);
                                        http::http_builder::write_http_status(&rw);
                                        eprintln!("Error parsing request: {}", error);
                                        panic!("Path NOT Found");
                                    }
                                };

                                // Path matching would happen here
                                let function_to_run = router.as_ref().unwrap_or_else(|| {
                                    // Handle the None case here, e.g., return a default function or panic with a specific message.

                                        let mut rw = ResponseWriter::new(stream_data.try_clone().unwrap());
                                        rw.write_status_code(500);
                                        http::http_builder::write_http_status(&rw);
                                        panic!("Internal erorr please check");

                                }).fetch_function_based_on_path(&parsed_request.path);

                                match function_to_run{
                                    Some(fetched_func) => {
                                        let mut rw = ResponseWriter::new(stream_data.try_clone().unwrap());
                                        (fetched_func.callback_function)(&mut rw,
                                        parsed_request);
                                    }
                                    None => {
                                        let mut rw = ResponseWriter::new(stream_data.try_clone().unwrap());
                                        rw.write_status_code(404);
                                        http::http_builder::write_http_status(&rw);
                                    }
                                }

                                break
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

}

impl HttpServer{
    pub fn new(address : String, port : String) -> HttpServer{
        HttpServer{
            address,
            port,
            listener: None,
            router: None,
            running: false
        }
    }

    pub fn attach_router(&mut self, router: Router){
        self.router = Some(router);
    }
}