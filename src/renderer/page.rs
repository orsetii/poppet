use crate::net::Response;

#[derive(Debug, Clone)]
pub struct Page {
    pub html_content: String,
}

impl Page {
    pub fn new(resp: Response) -> Self {
        Self {
            html_content: String::from_utf8(resp.body).unwrap(),
        }
    }

    pub fn all_text(&self) -> Vec<String> {
        todo!()
    }
}
