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
http_server.serve_static_file("/", "/file/index.html");
```
