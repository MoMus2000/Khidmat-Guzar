use std::net::{TcpListener, TcpStream};
use std::io::{Error, Write};

fn serve(address: String){
    println!("Serving @ {}", address);
    let mut stream = TcpListener::bind(address);
    let listener = stream.expect("Server should have started");
    for stream in listener.incoming(){
        handle_request(stream)
    }
}

fn handle_request(res: Result<TcpStream, Error>){
    println!("Got the request, handling it now !");
    let mut stream_data = res.unwrap();
    let buffer  = "Some Data \n".as_bytes();
    stream_data.write_all(buffer);
}

fn main() {
    let mut address = String::from("127.0.0.1");
    let port = "6969";

    address = address + ":" + port;

    serve(address);
}
