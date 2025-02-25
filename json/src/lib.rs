pub type value = serde_json::Value;

pub fn parse(json: String) -> self::value {
    serde_json::from_str(&json).unwrap()
}

pub fn get(value: self::value, key: String) -> self::value {
    value[key].clone()
}

pub fn as_array(value: self::value) -> Vec<self::value> {
    value.as_array().unwrap().to_vec()
}

pub fn as_string(value: self::value) -> String {
    value.as_str().unwrap().to_string()
}

pub fn as_boolean(value: self::value) -> bool {
    value.as_bool().unwrap()
}

pub fn as_u64(value: self::value) -> u64 {
    value.as_u64().unwrap()
}

pub fn as_f64(value: self::value) -> f64 {
    value.as_f64().unwrap()
}

pub fn as_i64(value: self::value) -> i64 {
    value.as_i64().unwrap()
}
