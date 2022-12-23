

pub struct Request {
    path: String,
    query: Option<String>,
    method: Method 
}

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}