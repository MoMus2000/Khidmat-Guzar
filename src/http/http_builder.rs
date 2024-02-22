use std::{fmt::format, string};

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
            empty_string => {
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