use std::char;

#[derive(Debug, Clone)]
pub struct HttpRequest{
    pub method: String,
    pub path: String,
    pub headers: Option<Vec<String>>
}

impl HttpRequest{
    pub fn parse_request(payload : &str) -> HttpRequest{

        let payload : Vec<char> = payload.chars().collect();
        let new_line = 0xA as char;
        let mut i = 0;

        let mut parsed_string = String::from("");
        let mut parsed_result = Vec::<String>::new();

        while payload[i] != new_line {
            if payload[i] == ' '{
                parsed_result.push(parsed_string.clone());
                parsed_string.clear();
            }
            parsed_string.push(payload[i]);
            i += 1
        }

        assert!(parsed_result.len() == 2);

        let method_parsed = parsed_result.get(0).cloned().unwrap_or_default();
        let path_parsed = parsed_result.get(1).cloned().unwrap_or_default();

        println!("Parsed {method_parsed} {path_parsed}");

        HttpRequest{
            method: method_parsed,
            path: path_parsed,
            headers: None
        }

    }
}