pub fn to_utf8_string(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap()
}

pub fn from_utf8_string(string: String) -> Vec<u8> {
    string.into_bytes()
}
