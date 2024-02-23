use std::char;

pub struct HttpRequest{
    pub method: String,
    pub path: String,
    pub headers: Option<Vec<String>>
}

impl HttpRequest{
    pub fn parse_request(payload : &str) -> HttpRequest{

        let payload : Vec<char> = payload.chars().collect();
        let new_line = 0xA as char;
        let mut i = 0;

        let mut parsed_string = String::from("");
        let mut parsed_result = Vec::<String>::new();

        while payload[i] != new_line {
            if payload[i] == ' '{
                parsed_result.push(parsed_string.clone());
                parsed_string.clear();
            }
            parsed_string.push(payload[i]);
            i += 1
        }

        assert!(parsed_result.len() == 2);

        let method_parsed = parsed_result.get(0).cloned().unwrap_or_default();
        let path_parsed = parsed_result.get(1).cloned().unwrap_or_default();

        println!("Parsed {method_parsed} {path_parsed}");

        HttpRequest{
            method: method_parsed,
            path: path_parsed,
            headers: None
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::http_server;
    use crate::http;
    use crate::server::HTTP_Server;
    use std::thread;
    use std::net::TcpStream;
    use crate::http::http_request;

    fn print_hello_world(response_writer : TcpStream, request : http_request::HttpRequest){
        println!("Hello World")
    }

    #[test]
    fn test_http_request_parsing() {
        let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

        let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/mustafa", "GET", print_hello_world);
        router.add_route("/mustafa1", "POST", print_hello_world);
        router.add_route("/mustafa2", "PUT", print_hello_world);
        router.add_route("/mustafa3", "DELETE", print_hello_world);

        http_server.attach_router(router);

        thread::spawn(move || {
            let result = std::panic::catch_unwind(move || {
                println!("Spawning the webserver");
                http_server.start_server();
            });
        
            if let Err(err) = result {
                eprintln!("Thread panicked: {:?}", err);
            }

        });

        let resp = reqwest::blocking::get("http://localhost:6969");

        resp.expect("Oh no something went wrong !");

    }

}