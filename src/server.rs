use std::io::prelude::*;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpStream, TcpListener};
use super::{
    handler::Handler,
    request::Request,
    response::Response,
    errors::WebServerError,
    thread_pool::ThreadPool
};

pub struct Server {
    host: String,
    port: u16,
    handlers: HashMap<(String, String), Box<Handler>>
}


impl Server {
    pub fn new(host: &str, port: u16) -> Self {
        Server{
            host: String::from(host),
            port: port,
            handlers: HashMap::new()
        }
    }

    pub fn add_handler(&mut self, (selector_method, selector_uri): (&str, &str), h: Box<dyn Handler>) {
        let k = (selector_method.to_string(), selector_uri.to_string());
        self.handlers.insert(k, h);
    }

    pub fn run(&self) {
        let addr = SocketAddr::from((self.host.parse::<IpAddr>().unwrap(), self.port));
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(4);
        println!("Waiting for connections...");
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");
            pool.execute(|| {
                self.handle_connection(&mut stream);
            })
        }
    }

    fn handle_connection(&self, stream: &mut TcpStream) {
        let mut buffer = [0; 512];

        let cnt = stream.read(&mut buffer).unwrap();
        match resolve_request(&String::from_utf8_lossy(&buffer[..cnt])) {
            Ok(req) => {
                let resp = match self.select_handler(&req) {
                    Some(handler) => {
                        handler(&req)
                    },
                    None => {
                        Response::from_params(404, include_str!("pages/404.html"))
                    }
                };
                self.write_response(stream, &resp);

            },
            Err(err) => {
                eprintln!("Error occurred while resolving request:\n{:?}", err)
            }
        }
    }

    fn select_handler(&self, req: &Request) -> Option<&Box<dyn Handler>> {
        self.handlers.get(&(req.method(), req.uri()))
    }

    fn write_response(&self, stream: &mut TcpStream, resp: &Response) {
        stream.write("HTTP/1.1 ".as_bytes()).unwrap();
        stream.write(resp.status_code().to_string().as_bytes()).unwrap();
        stream.write("\r\n".as_bytes()).unwrap();
        // TODO: write headers
        stream.write("\r\n".as_bytes()).unwrap();
        stream.write(resp.body().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn read_until(s: &[char], mut pos: usize, until: &str) -> (String, usize) {
    let mut result = String::new();
    while pos < s.len() {
        if s[pos..pos + until.len()].iter().collect::<String>() == until {
            return (result, pos + until.len());
        }
        result.push(s[pos]);
        pos += 1;
    }
    return (result, s.len())
}

fn resolve_request(buf: &str) -> Result<Request, WebServerError> {
    let chars = buf.chars().collect::<Vec<_>>();
    let pos = 0;
    let (method, pos) = read_until(&chars, pos, " ");
    let (uri, pos) = read_until(&chars, pos, " ");
    let (http_version, pos) = read_until(&chars, pos, "\r\n");
    let (header_content, pos) = read_until(&chars, pos, "\r\n");
    let body = chars[pos..].iter().collect::<String>();
    let request = Request::with_params(&uri, &method, &http_version, &header_content, &body);
    Ok(request)
}
