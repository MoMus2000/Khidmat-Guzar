## Http Server خدمت گزار 

A http/1.1 implementation from scratch.

## Features

### Initialization: آغاز

```rust
let http_server = http_server::HTTPServer::new();
http_server.start_server();
```

### Routing: راستہ نمائی

```rust

fn print_hello_world(response_writer : &RequestWriter, request : http_request::HttpRequest){
    println!("Hello world !")
}

let router = http::router::new();
router.add_route("/", "GET", print_hello_world);
http_server.attach_router(router);

```

### Serve Static files: ثابت فائلوں کی فراہمی

```rust
fn serve_html_file(response_writer : &mut ResponseWriter,
 request : http_request::HttpRequest){
    let content = http_content::serve_static_file(response_writer, request, 
        "./assets/index.html".to_string(), "text/html".to_string());
    response_writer.write_status_code(200);
    let headers = http::http_builder::build_http_payload(200, Some(content));
    write_http_response(&response_writer, Some(headers));
}

router.add_route("/file", "GET", serve_html_file);
```
