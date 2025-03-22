pub type Optional<T> = Option<T>;

pub fn is_defined<T>(value: Optional<T>) -> bool {
    value.is_some()
}

pub fn is_undefined<T>(value: Optional<T>) -> bool {
    value.is_none()
}

pub fn undefined<T>() -> Optional<T> {
    None
}

pub fn defined<T>(value: T) -> Optional<T> {
    Some(value)
}
