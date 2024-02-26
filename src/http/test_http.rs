#[cfg(test)]
mod tests {
    use crate::http::response_writer::ResponseWriter;
    use crate::http::router::Router;
    use crate::http_server;
    use crate::http;
    use crate::http_server::HttpServer;
    use crate::server::HTTP_Server;
    use core::time;
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpStream;
    use std::thread;
    use chrono::Duration;
    use http::http_content::http_content;
    use crate::http::http_builder::write_http_response;
    use crate::http::http_request;

    fn set_http_status_200(response_writer : &mut ResponseWriter, request : http_request::HttpRequest){
        response_writer.write_status_code(200);
        write_http_response(&response_writer, None);
    }

    pub fn initialize(router: Router) -> HttpServer {
        let mut http_server = http_server::HttpServer::new("localhost".to_string(), "6969".to_string());

        http_server.attach_router(router);

        http_server
    }

    #[test]
    fn test_http_status_codes() {
        /*
        Checking 200, 404 and 500 response codes.
         */
        let mut router =  Router::new()
            .unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/",
             "GET",
            set_http_status_200 
        );

        let mut http_server = initialize(router);

        thread::spawn(move || {
            let result = std::panic::catch_unwind(move || {
                println!("Spawning the webserver");
                http_server.start_server();
            });
        
            if let Err(err) = result {
                eprintln!("Thread panicked: {:?}", err);
            }

        });

        let resp = reqwest::blocking::get("http://localhost:6969/")
        .unwrap_or_else(|err| {
            eprintln!("Request failed: {}", err);
            // Handle the error or return a default value as needed.
            // You can also propagate the error further using 'return Err(err);' if this is a function returning a Result.
            std::process::exit(1);
        });

        assert!(resp.status().is_success(), "Expected 200 but got {}", resp.status());

        let resp = reqwest::blocking::get("http://localhost:6969/random_path")
        .unwrap_or_else(|err| {
            eprintln!("Request failed: {}", err);
            // Handle the error or return a default value as needed.
            // You can also propagate the error further using 'return Err(err);' if this is a function returning a Result.
            std::process::exit(1);
        });

        assert!(resp.status().is_client_error(), "Expected 404 but got {}", resp.status());


        const BUFFER_SIZE : usize = 8096;
        let mut buffer = [0; BUFFER_SIZE];

        // Attempt to connect to the server
        let mut socket = match TcpStream::connect("localhost:6969") {
            Ok(socket) => socket,
            Err(err) => {
                eprintln!("Error: Unable to connect to the server - {}", err);
                return;
            }
        };

        match socket.write_all("Some Random Text\n".as_bytes()) {
            Ok(_) => {
                // Attempt to read data from the server
                match socket.read(&mut buffer) {
                    Ok(size) => {
                        let response = &buffer[0..size];
                        assert!(std::str::from_utf8(response).unwrap().to_string().contains("500"));
                    }
                    Err(err) => {
                        eprintln!("Error reading from the server - {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error writing to the server - {}", err);
            }
        }

    }

    #[test]
    fn test_static_file_serve(){
        fn serve_html_file(response_writer : &mut ResponseWriter, request : http_request::HttpRequest){
            let content = http_content::serve_static_file(response_writer, request, 
                "./assets/index.html".to_string(), "text/html".to_string());
            response_writer.write_status_code(200);
            let headers = http::http_builder::build_http_payload(200, Some(content));
            write_http_response(&response_writer, Some(headers));
        }
        
        fn serve_js_file(response_writer : &mut ResponseWriter, request : http_request::HttpRequest){
            let content = http_content::serve_static_file(response_writer, request, 
                "./assets/script.js".to_string(), "application/javascript".to_string());
            response_writer.write_status_code(200);
            let headers = http::http_builder::build_http_payload(200, Some(content));
            write_http_response(&response_writer, Some(headers));
        }
        
        fn serve_css_file(response_writer : &mut ResponseWriter, request : http_request::HttpRequest){
            let content = http_content::serve_static_file(response_writer, request, 
                "./assets/style.css".to_string(), "text/css".to_string());
            response_writer.write_status_code(200);
            let headers = http::http_builder::build_http_payload(200, Some(content));
            write_http_response(&response_writer, Some(headers));
        }

        let mut router =  Router::new()
            .unwrap_or_else(||panic!("Something went wrong"));

        router.add_route("/",
             "GET",
            serve_html_file 
        );

        router.add_route("/script.js",
             "GET",
            serve_js_file 
        );

        router.add_route("/styles.css",
             "GET",
            serve_css_file 
        );

        let mut http_server = initialize(router);

        thread::spawn(move || {
            let result = std::panic::catch_unwind(move || {
                println!("Spawning the webserver");
                http_server.start_server();
            });
        
            if let Err(err) = result {
                eprintln!("Thread panicked: {:?}", err);
            }

        });

        let resp = reqwest::blocking::get("http://localhost:6969/")
        .unwrap_or_else(|err| {
            eprintln!("Request failed: {}", err);
            // Handle the error or return a default value as needed.
            // You can also propagate the error further using 'return Err(err);' if this is a function returning a Result.
            std::process::exit(1);
        });

        assert!(resp.headers().get("content-type").unwrap().to_str().unwrap() == "text/html");


        let resp = reqwest::blocking::get("http://localhost:6969/script.js")
        .unwrap_or_else(|err| {
            eprintln!("Request failed: {}", err);
            // Handle the error or return a default value as needed.
            // You can also propagate the error further using 'return Err(err);' if this is a function returning a Result.
            std::process::exit(1);
        });

        assert!(resp.headers().get("content-type").unwrap().to_str().unwrap() == "application/javascript");

        let resp = reqwest::blocking::get("http://localhost:6969/styles.css")
        .unwrap_or_else(|err| {
            eprintln!("Request failed: {}", err);
            // Handle the error or return a default value as needed.
            // You can also propagate the error further using 'return Err(err);' if this is a function returning a Result.
            std::process::exit(1);
        });

        assert!(resp.headers().get("content-type").unwrap().to_str().unwrap() == "text/css");

    }

}