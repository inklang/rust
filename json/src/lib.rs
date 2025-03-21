pub type Value = serde_json::Value;

pub fn parse(json: String) -> self::Value {
    serde_json::from_str(&json).unwrap()
}

pub fn get(value: self::Value, key: String) -> self::Value {
    value[key].clone()
}

pub fn as_array(value: self::Value) -> Vec<self::Value> {
    value.as_array().unwrap().to_vec()
}

pub fn as_string(value: self::Value) -> String {
    value.as_str().unwrap().to_string()
}

pub fn as_boolean(value: self::Value) -> bool {
    value.as_bool().unwrap()
}

pub fn as_u64(value: self::Value) -> u64 {
    value.as_u64().unwrap()
}

pub fn as_f64(value: self::Value) -> f64 {
    value.as_f64().unwrap()
}

pub fn as_i64(value: self::Value) -> i64 {
    value.as_i64().unwrap()
}
