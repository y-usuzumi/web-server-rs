use super::{
    request::Request,
    response::Response
};

pub type Handler = Box<dyn Fn(&Request) -> Response>;
