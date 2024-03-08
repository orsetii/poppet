use std::net::TcpStream;

use crate::PoppetResult;

pub enum Method {
    GET,
    POST,
    PUT,
    HEAD,
    OPTIONS,
}

pub struct URL(String);

pub struct Response {
    pub body: Vec<u8>,
}

impl Response {
    pub fn empty() -> Self {
        Self { body: vec![] }
    }
}

pub fn request(method: Method, url: URL) -> PoppetResult<Response> {
    use std::io::Read;
    use std::io::Write;
    let mut stream = TcpStream::connect(url.0)?;
    // TODO `peek` the http header byte size, then create
    // a buf of that size

    stream.write(&[1])?;
    let mut buf = [0; 0xFFF];

    stream.read(&mut [0; 128])?;
    Ok(Response::empty())
}
