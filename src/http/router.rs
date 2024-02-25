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

UPDATE: 
So the connection is now closed after serving the required resource
This can be modified in the future, for now this is fine.

The router will sit on top of the handle_function.
We have to read the path before fetching the corresponding function
Then we send off the computation to run in a seperate thread and write the data back
whatever way required since the TCP Connection will be available in the
seperate thread.
*/


use super::{http_request, response_writer::ResponseWriter};


#[derive(Clone)]
pub struct Router{
    pub router_elem: Vec::<RouterElement>
}

impl Router {
    pub fn new() -> Option<Router>{
        Some(Router{
            router_elem: Vec::<RouterElement>::new()
        })
    }

    pub fn fetch_function_based_on_path(&self, principal_path: &str) -> Option<RouterElement>{
        for element in &self.router_elem{
            if element.path.trim() == principal_path.trim(){
                return Some(element.clone())
            }
        }
        None
    }

    pub fn add_route(&mut self, path: &'static str, method: &'static str, callback_function: fn(response_writer : &mut ResponseWriter, request : http_request::HttpRequest)){

        let element = RouterElement{
            path,
            callback_function,
            method
        };

        match method {
            "GET" => {
                self.router_elem.push(element);
            }
            "POST" => {
                self.router_elem.push(element);
            }
            "PUT" => {
                self.router_elem.push(element);
            }
            "DELETE" => {
                self.router_elem.push(element);
            }
            "" => {
                panic!("Method cannot be empty or undefined !");
            }
            _ => {panic!("Method not found either [GET, POST, PUT, DELETE]")}
        }
    }

}

#[derive(Clone, Debug)]
pub struct RouterElement {
    pub path: &'static str,
    pub callback_function: fn(response_writer : &mut ResponseWriter, request : http_request::HttpRequest),
    pub method: &'static str
}