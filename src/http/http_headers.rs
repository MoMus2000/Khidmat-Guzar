use chrono::{DateTime, Utc};

pub enum HttpHeaders{
   Date,
   Server,
}

impl HttpHeaders {
    pub fn set_http(status_code: i32)->String{
        match status_code {
            code if code >= 200 && code < 300 => {
                format!("HTTP/1.1 200 OK\r\n")
            }
            code if code >= 300 && code < 400 => {
                if code == 302 {
                    format!("HTTP/1.1 302 Found\r\n")
                }
                else {
                    format!("HTTP/1.1 301 Moved Permanently\r\n")
                }
            }
            code if code >= 400 && code < 500  => {
                if code == 404 {
                    format!("HTTP/1.1 404 Not Found\r\n")
                }
                else if code == 400 {
                    format!("HTTP/1.1 400 Bad Request\r\n")
                }
                else{
                    format!("HTTP/1.1 403 Forbidden\r\n")
                }
            }
            code if code >= 500 && code < 600  => {
                format!("HTTP/1.1 500 Internal Server Error\r\n")
            }
            _ => {todo!("Not yet implemented this code.")}
        }
    }

    pub fn value(&self) -> String{
        match *self{
            HttpHeaders::Date => {
                let current_utc_time: DateTime<Utc> = Utc::now();

                let formatted_date = current_utc_time.format("%a, %d %b %Y %H:%M:%S GMT");

                format!("Date: {formatted_date}\r\n")
            }
            HttpHeaders::Server => {
                format!("Server: Khadim Rizvi\r\n")
            }
            _ => todo!("Not yet implemented this value")
        }
    }

    pub fn set_content_and_length(content_type: &str, content_length: usize) -> String{
        format!("Content-Type: {}\r\nContent-Length: {}",content_type, content_length)
    }
}