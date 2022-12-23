pub struct Request {
    path: String,
    query: String,
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