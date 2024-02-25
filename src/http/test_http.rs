#[cfg(test)]
mod tests {
    use crate::http::response_writer::ResponseWriter;
    use crate::http_server;
    use crate::http;
    use crate::server::HTTP_Server;
    use std::thread;
    use crate::http::http_request;

    fn print_hello_world(_response_writer : &ResponseWriter, _request : http_request::HttpRequest){
        println!("Hello World! ");
    }

    #[test]
    fn test_http_request_success() {
        let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

        let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/mustafa", "GET", print_hello_world);

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

        let resp = reqwest::blocking::get("http://localhost:6969/mustafa");

        let resp = resp.expect("Oh no something went wrong !");

        assert!(resp.status().is_success(), "Expected 200 but got {}", resp.status())

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
}