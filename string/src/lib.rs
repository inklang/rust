pub fn strip_ctl(input: String) -> String {
    input.chars().filter(|&c| !c.is_control()).collect()
}

pub fn concat(a: String, b: String) -> String {
    let mut result = String::with_capacity(a.len() + b.len());
    result.push_str(&a);
    result.push_str(&b);
    result
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
}
