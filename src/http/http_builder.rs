use std::io::Write;

use super::response_writer::ResponseWriter;
use crate::http::http_headers::HttpHeaders;
use crate::http;

pub fn build_http_payload(status_code : i32, content: Option<http::http_content::http_content>) -> String {
    let response_line =HttpHeaders::set_http(status_code);
    let date_header =  HttpHeaders::Date.value();
    let name_header =  HttpHeaders::Server.value();

    let content_header = match &content{
       Some(data) => {
            HttpHeaders::set_content_and_length(content.as_ref().unwrap().content_type.as_str(), data.content_length)
       }
       _ => {Default::default()}
    };

    let payload : String;

    if content.is_some(){
        payload = content.unwrap().content;
    }else{
        payload = String::from("");
    }

    let blank_line = "\r\n";

    if content_header.len() > 0{
        format!("{response_line}{date_header}{name_header}{content_header}{blank_line}\r\n{payload}")
    }
    else{
        format!("{response_line}{date_header}{name_header}{blank_line}")
    }
}

pub fn write_http_response(response_writer : &ResponseWriter, payload: Option<String>){

    let status_code= response_writer.status_code;
    let mut stream = &response_writer.stream;

    let mut http_headers = String::from("");

    if payload.is_none(){
        http_headers = build_http_payload(status_code, None);
    }else{
        http_headers = payload.unwrap();
    }

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