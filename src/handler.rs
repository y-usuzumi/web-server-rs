use super::{
    request::Request,
    response::Response
};

pub trait Handler = Fn(&Request) -> Response + Sync + 'static;
