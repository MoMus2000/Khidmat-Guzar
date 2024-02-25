// Things to do
// . Do code cleanup and learn how arc mutex works
// . Explore ncurses or alternatives for a tui
// . Explore Command pattern from zozin
// . Explore HTTP Protocol and work towards implementation

use crate::server::HTTP_Server;
use crate::http::http_request;
use crate::http::http_builder::write_http_status;
use crate::http::response_writer::ResponseWriter;

mod server;
mod http_server;
mod http;

fn print_hello_world(response_writer : &mut ResponseWriter, request : http_request::HttpRequest){
    println!("The called operation is {}", request.method);
    response_writer.write_status_code(200);
    write_http_status(&response_writer);
}

fn main() {
    let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

    let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

    router.add_route("/", "GET", print_hello_world);

    http_server.attach_router(router);

    http_server.start_server();
}