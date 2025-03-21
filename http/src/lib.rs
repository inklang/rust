use reqwest::{header::HeaderMap, Client};

pub type Request = reqwest::Request;
pub type Response = reqwest::Response;

#[derive(Default, Clone)]
pub struct Headers {
    pub map: HeaderMap,
}

pub fn create_request(method: String, url: inklang_url::Url, headers: Headers) -> Request {
    let client = Client::new();
    let mut request = client.request(method.parse().unwrap(), url);

    for (key, value) in headers.map.iter() {
        request = request.header(key, value);
    }

    request.build().unwrap()
}

pub async fn send_request(request: Request) -> Response {
    let client = Client::new();
    client.execute(request).await.unwrap()
}

pub fn create_headers() -> Headers {
    Headers::default()
}

pub async fn read_response_body_as_string(response: Response) -> String {
    response.text().await.unwrap()
}

pub async fn read_response_body_as_bytes(response: Response) -> Vec<u8> {
    response.bytes().await.unwrap().to_vec()
}
