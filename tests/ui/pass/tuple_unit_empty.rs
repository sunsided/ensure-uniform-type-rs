use ensure_uniform_type::ensure_uniform_type;

#[ensure_uniform_type]
pub struct Vec3(f32, f32, f32);

#[ensure_uniform_type]
pub struct Single(u8);

#[ensure_uniform_type]
pub struct Empty {}

#[ensure_uniform_type]
pub struct Unit;

fn main() {}
