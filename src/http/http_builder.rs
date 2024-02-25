use std::os::macos::raw::stat;
use std::str;
use crate::http;
use std::net::TcpStream;
use std::io::Write;

use super::response_writer::ResponseWriter;

pub struct HttpContent{
    pub body: String,
    pub html_tags : Vec<HtmlNode>
}

#[derive(Clone)]
pub struct HtmlNode {
    pub tag_name: String,
    pub tag_id : String,
    pub tag_value: String,
}

impl HtmlNode{

    pub fn new_with_str(tag_name: &str, tag_id: &str, tag_value: &str) -> HtmlNode{
        HtmlNode{
            tag_name: tag_name.to_string(),
            tag_id: tag_id.to_string(),
            tag_value: tag_value.to_string(),
        }
    }

    fn to_string(&self) -> String{
        let empty_id = String::from("");
        match &self.tag_id{
            _empty_string => {
                format!("<{} id=\"\">{}</{}>", self.tag_name, self.tag_value, self.tag_name)
            }
            _ => {
                format!("<{} id={}>{}</{}>", self.tag_name, &self.tag_id, self.tag_value,self.tag_name)
            }
        }
    }
}

impl HttpContent{

    pub fn new() -> HttpContent{
        HttpContent{
            body : "".to_string(),
            html_tags: Vec::<HtmlNode>::new()
        }
    }


    pub fn add_html_node(&mut self, html_node: HtmlNode){
        self.html_tags.push(html_node)
    }

    pub fn generate_boilerplate(&mut self){
        let mut body_tags = String::from("");
        for html_node in &self.html_tags{
            body_tags.push_str(html_node.to_string().as_str());
            body_tags.push_str("\n");
        }

        println!("{:?}", body_tags);

        let body_string = format!(r#"
       <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Your Page Title</title>
        </head>
        <body>
        {}
        </body>
        </html> 
        "#, body_tags).to_string();

        self.body = body_string;
    }

    pub fn convert_to_bytes(&self) -> &[u8]{
        self.body.as_bytes()
    }

}

pub fn write_http_status(response_writer : &ResponseWriter){
    let statuscode = response_writer.status_code;
    let mut stream = &response_writer.stream;
    let mut response_line = "";
    println!("THE HTTP STATUS CODE {}", statuscode);
    if statuscode == 200{
        response_line = "HTTP/1.1 200 OK\r\n";
    }
    else if statuscode == 404{
        response_line = "HTTP/1.1 404 Not Found\r\n";
    }
    else{
        response_line = "HTTP/1.1 500 Internal Server Error\r\n";
    } 

    let header_1 = b"Server: Crude Server\r\n";
    let header_2 = b"Content-type: text/html\r\n";
    let blank_line = b"\r\n";
    let mut html = HttpContent::new();

    let h1_node = http::http_builder::HtmlNode::new_with_str("h1", "" , "Some Random Text");

    html.add_html_node(h1_node.clone());

    // html.generate_boilerplate();

    // Calculate the length of the HTML content
    let content_length = html.body.len().to_string();
    let content_length_header = format!("Content-Length: {}\r\n", content_length);

    let response: Vec<u8> = [

        response_line.as_bytes().iter(),
        header_1.iter(),
        blank_line.into_iter(),
    ]
    .into_iter()
    .flatten()
    .cloned()
    .collect();

    match stream.write_all(&response){
        Ok(_)=>{},
        Err(e)=>{println!("Something went wrong writing: {}",e)}
    };

}