pub fn push<T>(mut array: Vec<T>, value: T) -> Vec<T> {
    array.push(value);
    array
}

pub type Of<T> = Vec<T>;

pub fn create<T>() -> Vec<T> {
    Vec::new()
}
