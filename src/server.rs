use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub trait Server{
    fn start_server(&mut self);
    fn handle_connection(stream_data: TcpStream, shared_data: Arc<Mutex<Vec<TcpStream>>>);
}