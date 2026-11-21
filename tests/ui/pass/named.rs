use ensure_uniform_type::ensure_uniform_type;

#[ensure_uniform_type]
pub struct Generic<T> {
    x: T,
    y: T,
}

#[ensure_uniform_type]
pub struct Nested<T> {
    a: Option<Vec<T>>,
    b: Option<Vec<T>>,
}

fn main() {}
