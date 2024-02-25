#[cfg(test)]
mod tests {
    use crate::http::response_writer::ResponseWriter;
    use crate::http_server;
    use crate::http;
    use crate::server::HTTP_Server;
    use std::thread;
    use std::io::{Write, Read};
    use crate::http::http_request;

    fn print_hello_world(_response_writer : &ResponseWriter, _request : http_request::HttpRequest){
        println!("Hello World! ");
    }

    #[test]
    fn test_internal_server_error() {
        let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

        let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/", "GET", print_hello_world);

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

        let listener= std::net::TcpStream::connect("localhost:6969");

        let clone1 = listener.unwrap().try_clone().unwrap();
        let mut clone2 = clone1.try_clone().unwrap();

        let mut buffer = [0; 2014];
        let mut list = clone1.try_clone().unwrap();

        clone2.write_all("Some random text \n".as_bytes());

        match list.read(&mut buffer){
            Ok(len) =>{
              let string_result = std::str::from_utf8(&buffer[0..len]);
              assert!(string_result.unwrap().to_string().contains("500"));
            }
            Err(err) => {
                println!("Error {}", err);
            }
        }

    }

    #[test]
    fn test_router_not_found() {
        let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

        let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/", "GET", print_hello_world);

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

        let resp = reqwest::blocking::get("http://localhost:6969/some_random_path");

        let resp = resp.expect("Oh no something went wrong !");

        assert!(resp.status().is_client_error(), "Expected 404 but got {}", resp.status())

    }

    #[test]
    fn test_http_success() {
        let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

        let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/", "GET", print_hello_world);

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

        let resp = reqwest::blocking::get("http://localhost:6969/some_random_path");

        let resp = resp.expect("Oh no something went wrong !");

        assert!(resp.status().is_client_error(), "Expected 404 but got {}", resp.status())

    }

}