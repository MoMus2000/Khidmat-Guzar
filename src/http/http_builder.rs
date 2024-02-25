use std::io::Write;

use super::response_writer::ResponseWriter;
use crate::http::http_headers::HttpHeaders;

pub fn build_http_headers(status_code : i32) -> String {
    let response_line =HttpHeaders::set_http(status_code);
    let date_header =  HttpHeaders::Date.value();
    let name_header =  HttpHeaders::Server.value();
    let blank_line = "\r\n";

    format!("{response_line}{date_header}{name_header}{blank_line}")
}

pub fn build_http_content(){

}

pub fn write_http_status(response_writer : &ResponseWriter){

    let status_code= response_writer.status_code;
    let mut stream = &response_writer.stream;

    let http_headers = build_http_headers(status_code);

    let response: Vec<u8> = [
        http_headers.as_bytes().iter()
    ]
    .into_iter()
    .flatten()
    .cloned()
    .collect();

    println!("{}", std::str::from_utf8(&response).unwrap());

    match stream.write_all(&response){
        Ok(_)=>{},
        Err(e)=>{println!("Error writing data back to the connection: {}",e)}
    };

}