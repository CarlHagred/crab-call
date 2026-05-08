use std::collections::HashMap;

#[derive(Debug)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug)]
struct HttpRequest {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

fn main() {
    println!("Hello, world!");
}
