use ensure_uniform_type::ensure_uniform_type;

#[ensure_uniform_type]
pub struct Example<T> {
    x: T,
    offending: u32,
}

fn main() {}
