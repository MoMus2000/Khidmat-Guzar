#[cfg(test)]
mod tests {
    use crate::http::response_writer::ResponseWriter;
    use crate::http_server;
    use crate::http;
    use crate::server::HTTP_Server;
    use std::borrow::BorrowMut;
    use std::thread;
    use std::io::{Write, Read};
    use crate::http::http_request;
    use std::sync::{Arc, Mutex};

    fn print_hello_world(_response_writer : &mut ResponseWriter, _request : http_request::HttpRequest){
        println!("Hello World! ");
    }

    fn set_http_status_201(response_writer : &mut ResponseWriter, _request : http_request::HttpRequest){
        println!("Hello World! ");
        response_writer.write_status_code(201);
        http::http_builder::write_http_status(response_writer);
        println!("Wrote the status code")
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
                println!("Closing the server");
            });
        
            if let Err(err) = result {
                eprintln!("Thread panicked: {:?}", err);
            }

        });

        let resp = reqwest::blocking::get("http://localhost:6969/some_random_path");

        let resp = resp.expect("Oh no something went wrong !");

        assert!(resp.status().is_client_error(), "Expected 404 but got {}", resp.status());

    }

    #[test]
    fn test_http_success() {
        let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

        let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/", "GET", print_hello_world);

        http_server.attach_router(router);

        let server_thread = thread::spawn({
            move || {
                let result = std::panic::catch_unwind(move || {
                    println!("Spawning the webserver");
                        // Your web server logic here
                        http_server.start_server();
                });
    
                if let Err(err) = result {
                    eprintln!("Thread panicked: {:?}", err);
                }
            }
        }); 

        let resp = reqwest::blocking::get("http://localhost:6969/some_random_path");

        let resp = resp.expect("Oh no something went wrong !");

        assert!(resp.status().is_client_error(), "Expected 404 but got {}", resp.status());

    }

    #[test]
    fn test_setting_http_status(){
        let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

        let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/", "GET", set_http_status_201);

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

        println!("Sending the request");

        let resp = reqwest::blocking::get("http://localhost:6969/");

        let resp = resp.expect("Oh no something went wrong !");

        assert!(resp.status().as_u16() == 201, "Expected 201 but got {}", resp.status())

    }

    #[test]
    fn test_http_start_close(){
        let http_server = Arc::new(Mutex::new(http_server::HttpServer::new("localhost".to_string(), "6969".to_string())));

        let mut router =  http::router::Router::new().unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/", "GET", set_http_status_201);

        http_server.lock().unwrap().attach_router(router);

        let cloned_server = Arc::clone(&http_server);
        let server_thread = thread::spawn(
            move || {
            let result = std::panic::catch_unwind(move || {
                println!("Spawning the webserver");
                cloned_server.lock().unwrap().start_server();
                println!("Stopping the http Server");
            });
        
            if let Err(err) = result {
                eprintln!("Thread panicked: {:?}", err);
            }

        });

        http_server.lock().unwrap().close();

        println!("Waiting for server thread to finish");
        server_thread.join().unwrap();
        println!("Execution complete");

    }

}