// Things to do
// . Do code cleanup and learn how arc mutex works
// . Explore ncurses or alternatives for a tui
// . Explore Command pattern from zozin
// . Explore HTTP Protocol and work towards implementation

use crate::server::HTTP_Server;
use crate::http::http_request;
use crate::http::http_builder::write_http_response;
use crate::http::response_writer::ResponseWriter;
use crate::http::http_content::http_content;

mod server;
mod http_server;
mod http;

fn print_hello_world(response_writer : &mut ResponseWriter, request : http_request::HttpRequest){
    println!("The called operation is {}", request.method);
    response_writer.write_status_code(200);
    write_http_response(&response_writer, None);
}

fn serve_html_file(response_writer : &mut ResponseWriter, request : http_request::HttpRequest){
    let content = http_content::serve_static_file(response_writer, request, 
        "./assets/index.html".to_string(), "text/html".to_string());
    response_writer.write_status_code(200);
    let headers = http::http_builder::build_http_payload(200, Some(content));
    write_http_response(&response_writer, Some(headers));
}

fn main() {
    let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

    let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

    router.add_route("/", "GET", serve_html_file);
    router.add_route("/file", "GET", serve_html_file);

    http_server.attach_router(router);

    http_server.start_server();
}