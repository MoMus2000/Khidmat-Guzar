use std::char;

#[derive(Debug, Clone)]
pub struct HttpRequest{
    pub method: String,
    pub path: String,
    pub headers: Option<Vec<String>>
}

impl HttpRequest{
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