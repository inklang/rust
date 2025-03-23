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

pub fn as_optional_array(value: Value) -> Option<Vec<Value>> {
    if let Some(array) = value.as_array() {
        return Some(array.to_vec());
    }

    None
}

pub fn as_optional_string(value: Value) -> Option<String> {
    if let Some(array) = value.as_str() {
        return Some(array.to_string());
    }

    None
}

pub fn as_optional_boolean(value: Value) -> Option<bool> {
    value.as_bool()
}

pub fn as_optional_u64(value: Value) -> Option<u64> {
    value.as_u64()
}

pub fn as_optional_f64(value: Value) -> Option<f64> {
    value.as_f64()
}

pub fn as_optional_i64(value: Value) -> Option<i64> {
    value.as_i64()
}

pub fn is_defined(value: Value) -> bool {
    !value.is_null()
}

pub fn is_undefined(value: Value) -> bool {
    value.is_null()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let mut json = parse(r#"{"key": "value", "key2": 2}"#.into());

        let key = get_property!(json, "key".to_string());
        let key2 = get_property!(json, "key2".to_string());

        assert_eq!(as_string(key), "value".to_string());
        assert_eq!(as_u64(key2), 2);

        let key3 = get_property!(json, "key3".to_string());
        assert!(is_undefined(key3.clone()));
    }
}
