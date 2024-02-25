pub struct http_content{
    content_length: usize,
    content_type: String,
    content: String
}

impl http_content{
    pub fn new(content: String, content_type: String) -> http_content{
        http_content{
            content_length: content.clone().len(),
            content: content,
            content_type: content_type
        }
    }
}