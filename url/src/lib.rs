pub fn parse(url: String) -> url::Url {
    url::Url::parse(&url).unwrap()
}
