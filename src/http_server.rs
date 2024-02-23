use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Write, Read};
use std::str;

use crate::http::http_builder::{HttpContent, HtmlNode};
use crate::http::router::Router;
use crate::server::Server;
use crate::http::{self, router};

pub struct HttpServer{
    address: String,
    port: String,
    listener: Option<TcpListener>,
    router: Option<router::Router>
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
                    let cloned_router = self.router.clone();
                    let moved_shared_data = Arc::clone(&shared_data);
                    thread::spawn(move || {
                        HttpServer::handle_connection(stream, moved_shared_data, cloned_router);
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
    // Closing the http connection after serving the required resource
    fn handle_connection(stream_data: TcpStream, _ : Arc<Mutex<Vec<TcpStream>>>, router: Option<Router>){
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

                            // Path matching would happen here
                            let function_to_run = router.as_ref().expect("").fetch_function_based_on_path("/mustafa");
                            (function_to_run.expect("").callback_function)();
                            (router.as_ref().expect("something").router_elem[0].callback_function)();

                            let stream_data_copied= stream_data.try_clone();
                            HttpServer::write_http_status(stream_data_copied.unwrap());
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

impl HttpServer{
    pub fn new(address : String, port : String) -> HttpServer{
        HttpServer{
            address,
            port,
            listener: None,
            router: None
        }
    }

    pub fn attach_router(&mut self, router: Router){
        self.router = Some(router);
    }

    fn write_http_status(mut stream: TcpStream){

        let response_line = b"HTTP/1.1 200 OK\r\n";
        let header_1 = b"Server: Crude Server\r\n";
        let header_2 = b"Content-type: text/html\r\n";
        let blank_line = b"\r\n";
        let mut html = HttpContent::new();

        let h1_node = http::http_builder::HtmlNode::new_with_str("h1", "" , "Some Random Text");

        html.add_html_node(h1_node.clone());

        html.generate_boilerplate();

        // Calculate the length of the HTML content
        let content_length = html.body.len().to_string();
        let content_length_header = format!("Content-Length: {}\r\n", content_length);

        let response: Vec<u8> = [

            response_line.iter(),
            header_1.iter(),
            header_2.into_iter(),
            content_length_header.as_bytes().into_iter(),
            blank_line.into_iter(),
            html.convert_to_bytes().into_iter(),

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