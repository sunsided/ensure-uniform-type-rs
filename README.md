# Ensure uniform field types

A compile-time check to ensure that a type uses uniform types across its fields.

An example use for this macro is to ensure that a struct `#[repr(C)]` layout can
be correctly mapped onto a slice of the (uniform) field type.

## Example

Assume the following type:

```
#[ensure_uniform_type]
pub struct Example<T>
{
    /// First field
    x: T,

    // Different type
    offending: u32,
}
```

The above would fail to compile, instead giving the error:

```
error: Struct DifferentialDriveState has fields of different types. Expected uniform use of T, found u32 in field lol.
  --> src/differential_drive.rs:16:1
   |
16 | / /// A state of a differential drive robot, or differential wheeled robot.
18 | | #[ensure_uniform_type]
19 | | pub struct Example<T>
...  |
37 | |     offending: u32,
38 | | }
   | |_^
```

By contrast, the following would compile without an error:

```rust
#[ensure_uniform_type]
pub struct Example<T>
{
    x: T,
    not_offending: T,
}
```
