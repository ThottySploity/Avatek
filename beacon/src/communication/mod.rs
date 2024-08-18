pub mod http;
pub mod pipes;
pub mod tcp;

#[derive(Clone)]
pub struct HttpConfiguration {
    pub useragent: String,
    pub http_uri: String,
    pub headers: Vec<String>,
    pub uri_append: String,
}

#[derive(Clone)]
pub struct PipeConfiguration {
    pub pipename: String,
}

#[derive(Clone)]
pub struct TcpConfiguration {
    pub host: String,
    pub port: String,
}