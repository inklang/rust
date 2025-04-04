pub fn strip_ctl(input: String) -> String {
    input.chars().filter(|&c| !c.is_control()).collect()
}

pub fn strip_all(input: String, search: String) -> String {
    input.replace(&search, "")
}

pub fn split(input: String, separator: String) -> inklang_array::Of<String> {
    input.split(&separator).map(|s| s.to_string()).collect()
}

pub fn concat(segments: inklang_array::Of<String>) -> String {
    segments.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_strips_ctrl() {
        let string: String = "\0\0".into();
        let result = strip_ctl(string);
        assert_eq!(result, "");
    }

    #[test]
    fn it_strips_all() {
        let string: String = "hello world".into();
        let result = strip_all(string, "l".into());
        assert_eq!(result, "heo word");
    }

    #[test]
    fn it_splits() {
        let string: String = "hello world".into();
        let result = split(string, " ".into());
        assert_eq!(result, vec!["hello".to_string(), "world".to_string()]);
    }

    #[test]
    fn it_concats() {
        let segments = vec!["hello".to_string(), "world".to_string()];
        let result = concat(segments);
        assert_eq!(result, "helloworld");
    }
}
