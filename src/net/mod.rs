use std::{
    fmt::{format, Debug},
    net::TcpStream,
};

use openssl::ssl::{SslConnector, SslMethod};
use thiserror::Error;
use tracing::info;
use url::Url;

use crate::{PoppetError, PoppetResult};

#[derive(Error, Debug)]
pub enum NetError {
    #[error("No scheme found")]
    NoSchemeFound,
    #[error("No host found")]
    NoHostFound,
    #[error("Invalid Scheme provided")]
    InvalidScheme,
}
pub enum Method {
    GET,
    POST,
    PUT,
    HEAD,
    OPTIONS,
}

#[derive(Debug)]
pub struct Headers(Vec<(String, String)>);

pub struct Response {
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new<'a>(b: Vec<u8>) -> PoppetResult<Self> {
        if let Some(split_resp) = Self::b_as_string(b).split_once("\r\n\r\n") {
            Ok(Self {
                body: split_resp.1.as_bytes().to_vec(),
                headers: Self::parse_headers(split_resp.0),
            })
        } else {
            Err(crate::PoppetError::Unknown)
        }
    }
    pub fn b_as_string<'a>(b: Vec<u8>) -> String {
        String::from_utf8(b).unwrap()
    }

    fn parse_headers<'a>(header_section: &'a str) -> Headers {
        let mut h = Headers(Vec::new());
        for line in header_section.lines() {
            if let Some(pair) = line.split_once(":") {
                h.0.push((pair.0.trim().to_lowercase(), pair.1.trim().to_lowercase()));
            }
        }
        h
    }
}

impl Debug for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Response")
            .field("body", &Self::b_as_string(self.body.clone()).as_str())
            .field("headers", &self.headers)
            .finish()
    }
}

pub fn request(method: Method, url: Url) -> PoppetResult<Response> {
    use std::io::Read;
    use std::io::Write;

    let port = match url.scheme() {
        "http" => 80,
        "https" => 443,
        _ => return Err(NetError::InvalidScheme.into()),
    };

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect(format!(
        "{}:{}",
        url.host_str().ok_or(NetError::NoHostFound)?,
        port
    ))?;
    let mut stream = connector
        .connect(url.host_str().ok_or(NetError::NoHostFound)?, stream)
        .map_err(|_| PoppetError::Unknown)?;
    info!(
        "connecting to {}:443",
        url.host_str().ok_or(NetError::NoHostFound)?,
    );

    stream.write_all(&http_req(url, method)[..])?;
    let mut buf = vec![];

    let n = stream.read_to_end(&mut buf)?;
    info!("Read {} bytes", n);

    Ok(Response::new(buf)?)
}

fn send_req_insecure(url: Url) {}

pub fn http_req(url: Url, method: Method) -> Vec<u8> {
    info!(
        "Sending: GET {} HTTP/1.0\\r\\nHOST: {}\\r\\n\\r\\n",
        url.path(),
        url.host().unwrap()
    );
    format!(
        "GET {} HTTP/1.0\r\nHOST: {}\r\n\r\n",
        url.path(),
        url.host().unwrap()
    )
    .as_bytes()
    .to_vec()
}
