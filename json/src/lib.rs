pub type Value = serde_json::Value;

pub fn parse(json: String) -> Value {
    serde_json::from_str(&json).unwrap()
}

pub use inklang_json_macro::get_property;

pub fn as_array(value: Value) -> Vec<Value> {
    value.as_array().unwrap().to_vec()
}

pub fn as_string(value: Value) -> String {
    value.as_str().unwrap().to_string()
}

pub fn as_boolean(value: Value) -> bool {
    value.as_bool().unwrap()
}

pub fn as_u64(value: Value) -> u64 {
    value.as_u64().unwrap()
}

pub fn as_f64(value: Value) -> f64 {
    value.as_f64().unwrap()
}

pub fn as_i64(value: Value) -> i64 {
    value.as_i64().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let mut json = parse(r#"{"key": "value", "key2": "value"}"#.into());

        let key = get_property!(json, "key".to_string());
        let key2 = get_property!(json, "key2".to_string());

        assert_eq!(as_string(key), "value".to_string());
        assert_eq!(as_string(key2), "value".to_string());
    }
}
