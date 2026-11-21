# `#[ensure_uniform_type]`: Ensure uniform struct field types at compile-time

[![crates.io](https://img.shields.io/crates/v/ensure-uniform-type.svg)](https://crates.io/crates/ensure-uniform-type)
[![docs.rs](https://img.shields.io/docsrs/ensure-uniform-type)](https://docs.rs/ensure-uniform-type)
[![CI](https://github.com/sunsided/ensure-uniform-type-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/ensure-uniform-type-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sunsided/ensure-uniform-type-rs/graph/badge.svg)](https://codecov.io/gh/sunsided/ensure-uniform-type-rs)
[![MSRV](https://img.shields.io/crates/msrv/ensure-uniform-type)](https://crates.io/crates/ensure-uniform-type)
[![License](https://img.shields.io/crates/l/ensure-uniform-type)](https://github.com/sunsided/ensure-uniform-type-rs#license)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A compile-time check to ensure that a struct uses the same type for all of its fields.

An example use for this macro is to ensure that a struct `#[repr(C)]` layout can
be correctly mapped onto a slice of the (uniform) field type.

## Usage

```toml
[dependencies]
ensure-uniform-type = "1"
```

Assume the following type:

```rust,compile_fail
use ensure_uniform_type::ensure_uniform_type;

#[ensure_uniform_type]
pub struct Example<T> {
    /// First field
    x: T,

    // Different type
    offending: u32,
}
```

The above would fail to compile, instead giving the error:

```text
error: Struct Example has fields of different types. Expected uniform use of T, found u32 in field offending.
  --> src/lib.rs:8:16
   |
 8 |     offending: u32,
   |                ^^^
```

By contrast, the following would compile without an error:

```rust
use ensure_uniform_type::ensure_uniform_type;

#[ensure_uniform_type]
pub struct Example<T> {
    x: T,
    not_offending: T,
}
```

Tuple structs are supported as well. Unit and empty structs are trivially uniform.

```rust
use ensure_uniform_type::ensure_uniform_type;

#[ensure_uniform_type]
#[repr(C)]
pub struct Vec3(f32, f32, f32);
```

The check is name-based: `f32` and `std::primitive::f32`, or a type and its alias,
count as different types.

## MSRV

The minimum supported Rust version is 1.85. It covers the library only; running the
test suite may need a newer toolchain. Raising it is not a breaking change; it
happens in a minor release and is noted in the changelog.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-Apache-2.0](https://github.com/sunsided/ensure-uniform-type-rs/blob/main/LICENSE-Apache-2.0) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/sunsided/ensure-uniform-type-rs/blob/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

## Contributing

See [CONTRIBUTING.md](https://github.com/sunsided/ensure-uniform-type-rs/blob/main/CONTRIBUTING.md).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
