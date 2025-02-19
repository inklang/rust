pub fn strip_ctl(input: String) -> String {
    input.chars().filter(|&c| !c.is_control()).collect()
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
