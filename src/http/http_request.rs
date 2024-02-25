use std::{char, collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
pub struct HttpRequest{
    pub method: String,
    pub path: String,
    pub headers: Option<HashMap<String, String>>
}

impl HttpRequest{
    pub fn parse_request(payload: &str) -> Result<HttpRequest, &'static str> {
        println!("{:?}", payload);
        let payload: Vec<char> = payload.chars().collect();
        let new_line = 0xA as char;
        let mut i = 0;
    
        let mut parsed_string = String::from("");
        let mut parsed_result = Vec::<String>::new();
    
        while payload[i] != new_line {
            if payload[i] == ' ' {
                parsed_result.push(parsed_string.clone());
                parsed_string.clear();
            }
            parsed_string.push(payload[i]);
            i += 1;
        }

        let mut char_pointer = String::from("");
        let mut header_map = HashMap::<String,String>::new();

        let mut current_key = String::from("");

        while i < payload.len(){
            if payload[i] == ':'{
                current_key = char_pointer.clone();
                char_pointer.clear();
                if i < payload.len(){
                    i+= 1
                }
            }
            if payload[i] == new_line{
                header_map.insert(current_key.clone(), char_pointer.clone());
            }
            char_pointer.push(payload[i]);
            i+=1
        }

        println!("{:?}", header_map);
    
        if parsed_result.len() != 2 {
            return Err("Invalid request format");
        }
    
        let method_parsed = parsed_result.get(0).cloned().unwrap_or_default();
        let path_parsed = parsed_result.get(1).cloned().unwrap_or_default();
    
        Ok(HttpRequest {
            method: method_parsed,
            path: path_parsed,
            headers: None,
        })
    }
}