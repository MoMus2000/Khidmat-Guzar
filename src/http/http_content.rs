use crate::http::response_writer::ResponseWriter;
use crate::http;

use super::http_request::HttpRequest;

pub struct http_content{
    pub content_length: usize,
    pub content_type: String,
    pub content: String
}

impl http_content{
    pub fn new(content: String, content_type: String) -> http_content{
        http_content{
            content_length: content.clone().len(),
            content,
            content_type
        }
    }

    pub fn serve_static_file(w : &mut ResponseWriter, r: HttpRequest, 
        file_path: String, content_type: String) -> http_content{
        let content = std::fs::read_to_string(file_path).expect("File not found");
        http::http_content::http_content::new(content, content_type)
    }
}