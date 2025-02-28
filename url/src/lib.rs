pub use url::Url;

pub fn parse(url: String) -> Url {
    url::Url::parse(&url).unwrap()
}
