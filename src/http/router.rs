/*
The router is responsible for routing traffic into functions
We get the path and the router is responsible for calling the 
function corresponding to that path.

A caveat we need to handle is how to get the corresponding connection
to write to ??

let r = Router::new();
r.add_func("/", some_func_to_handle);

So the router lives inside the server struct
when we a handle the request, we have the connection available
We can do a look up for the corresponding function
We can then run the function
Then write whatever response to the TCP connection available
*/

use std::net::TcpStream;

pub struct Router{
    pub router_elem: Vec::<RouterElement>
}

impl Router {
    pub fn new() -> Option<Router>{
        Some(Router{
            router_elem: Vec::<RouterElement>::new()
        })
    }
}

pub struct RouterElement {
    pub path: &'static str,
    pub callback_function: fn(method: &str) -> bool
}