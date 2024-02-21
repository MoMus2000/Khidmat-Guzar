// Things to do
// . Do code cleanup and learn how arc mutex works
// . Explore ncurses or alternatives for a tui
// . Explore Command pattern from zozin
// . Explore HTTP Protocol and work towards implementation
use crate::server::Server;

mod server;
mod tcp;
mod http_server;


fn main() {
    // let mut server = tcp::TcpServer::new("localhost".to_string(), "6969".to_string());
    // server.start_server()
    let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());
    http_server.start_server();
}