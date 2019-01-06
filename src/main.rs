#![feature(box_syntax)]

use webserver::{
    Server,
    Handler,
    Request,
    Response
};

fn handlers() -> Vec<(String, String, Handler)> {
    vec![
        ("GET".to_string(), "/".to_string(), box |_| {
            Response::from_params(200, include_str!("pages/hello.html"))
        }),
        ("GET".to_string(), "/sleep".to_string(), box |_| {
            Response::from_params(200, "GG")
        })
    ]
}

fn main() {
    let mut s = Server::new("127.0.0.1", 7878);
    for (method, uri, h) in handlers() {
        s.add_handler((&method, &uri), h);
    }
    s.run();
}
