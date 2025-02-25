# `json` support for Inklang

## Example output from the language

```rust
fn main() {
  let mut feeds: array::of<String> = array::create();
  let json: json::value = json::parse(r#"{ "results": [{"name": "feed1"}, {"name": "feed2"}] }"#.into());

  for feed in json::as_array(json::get(json, "results".into())) {
    let name = json::as_string(json::get(feed, "name".into()));
    feeds.push(name);
  }

  println!("{:?}", feeds);
}
```
