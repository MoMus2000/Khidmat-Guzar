use std::{char, collections::HashMap};

#[derive(Debug, Clone)]
pub struct HttpRequest{
    pub method: String,
    pub path: String,
    pub headers: Option<HashMap<String, String>>,
    pub query_params: Option<String>
}

impl HttpRequest{

    fn strip_query_params(principal_path: String) -> (Option<String>, Option<String>){
        let split_path = principal_path.split("?");
        let vec : Vec<&str> = split_path.collect();
        if vec.len() >= 2{
            return (Some(vec[0].to_string()), Some(vec[1].to_string()))
        }
        (None, None)
    }

    fn parse_headers(start_index : usize, payload: Vec<char>) -> Option<HashMap<String, String>>{

        let new_line = 0xA as char;
        let mut i = start_index;
        let mut header_map = HashMap::<String,String>::new();
        let mut char_pointer = String::from("");
        let mut value_pointer = String::from("");

        while i < payload.len(){

            if payload[i] == ':'{
                if i < payload.len(){
                    i+= 1
                }

                while payload[i] != new_line{
                    value_pointer.push(payload[i]);
                    i += 1;
                }

                let modified_char_pointer = char_pointer
                    .replace("\r", "").replace("\n", "");
                let modified_value_pointer = value_pointer
                    .replace("\r", "").replace("\n", "");

                header_map.insert(modified_char_pointer.trim().to_string().clone(),
                 modified_value_pointer.trim().to_string().clone());

                char_pointer.clear();
                value_pointer.clear();

            }

            char_pointer.push(payload[i]);
            i+=1

        }

        Some(header_map)

    }

    pub fn parse_request(payload: &str) -> Result<HttpRequest, &'static str> {

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

        let header_map = HttpRequest::parse_headers(i, payload);

        if parsed_result.len() != 2 {
            return Err("Invalid request format");
        }

        if header_map.as_ref().unwrap().is_empty(){
            return Err("Invalid request format");
        }

        println!("{:?}", header_map);
    
        let method_parsed = parsed_result.get(0).cloned().unwrap_or_default();
        let path_parsed = parsed_result.get(1).cloned().unwrap_or_default();

        let (principal_path, query_params) = HttpRequest::strip_query_params(path_parsed.clone());

        if principal_path.is_some()  && query_params.is_some(){
            return Ok(HttpRequest {
                method: method_parsed,
                path: principal_path.unwrap(),
                headers: header_map,
                query_params
            })
        }
        Ok(HttpRequest {
            method: method_parsed,
            path: path_parsed,
            headers: header_map,
            query_params:None 
        })
    }

}