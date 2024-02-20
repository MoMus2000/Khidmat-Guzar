use std::net::{TcpListener, TcpStream};
use std::str;
use std::io::{Error, Read, Write};

fn serve(address: String){
    println!("Serving @ {}", address);
    let stream = TcpListener::bind(address);
    let listener = stream.expect("Server should have started");
    for stream in listener.incoming(){
        handle_request(stream)
    }
}

fn handle_request(res: Result<TcpStream, Error>){
    println!("Got the request, handling it now !");
    let mut stream_data = res.unwrap();
    let buffer  = "Connected .. Send some data over. \n".as_bytes();
    stream_data.write_all(buffer).expect("Should have sent the data");
    let mut buffer = [0; 11];
    loop{
        let length_of_chars = stream_data.read(&mut buffer).expect("Unable to read from buffer");
        println!("Length of data got back {}", length_of_chars);
        println!("{:?}", str::from_utf8(&buffer[0..length_of_chars]).unwrap());
    }
}

fn main() {
    let mut address = String::from("127.0.0.1");
    let port = "6969";

    address = address + ":" + port;

    serve(address);
}
