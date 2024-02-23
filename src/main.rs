// Things to do
// . Do code cleanup and learn how arc mutex works
// . Explore ncurses or alternatives for a tui
// . Explore Command pattern from zozin
// . Explore HTTP Protocol and work towards implementation
use crate::server::{HTTP_Server, Server};

mod server;
mod tcp;
mod http_server;
mod http;

fn print_hello_world(){
    println!("Hello world !")
}

fn main() {
    let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

    let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

    router.add_route("/mustafa", "GET", print_hello_world);
    router.add_route("/mustafa1", "POST", print_hello_world);
    router.add_route("/mustafa2", "PUT", print_hello_world);
    router.add_route("/mustafa3", "DELETE", print_hello_world);

    http_server.attach_router(router);

    http_server.start_server();
}