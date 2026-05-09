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

#[derive(Debug, PartialEq)]
enum HttpToken {
    Variable(String, String),
    Separator,
    RequestLine(String, String),
    Header(String, String),
    Body(String),
    Comment(String),
}

fn tokenize(content: &str) -> Vec<HttpToken> {
    let mut tokens = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("###") {
            tokens.push(HttpToken::Separator);
        } else if trimmed.starts_with("#") {
            tokens.push(HttpToken::Comment(trimmed.to_string()))
        }
    }
    tokens
}

fn main() {
    let mut my_headers = HashMap::new();
    my_headers.insert("Content-Type".to_string(), "application/json".to_string());
    let my_http_request = HttpRequest {
        method: HttpMethod::Post,
        url: "".to_string(),
        headers: my_headers,
        body: Some("Hello World".to_string()),
    };
    println!("{:#?}", my_http_request);

    let raw_text = "
    # This is my test
    ###
    # Another comment
    ";

    let my_tokens = tokenize(raw_text);
    println!("{:#?}", my_tokens);
}
