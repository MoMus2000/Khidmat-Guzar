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
let router = http::router::new();
router.add_route("/", "GET", func_to_run);
http_server.attach_router(router);
```

### Serve Static files: ثابت فائلوں کی فراہمی

```rust
http_server.serve_static_file("/file/index.html");
```
